//! MIRRA v1 deterministic correction weighting canister.
//!
//! Consensus-visible arithmetic and encodings are frozen in `core` and
//! `canonical`. Durable state is held directly in versioned stable structures;
//! no heap snapshot is required during an upgrade.

pub mod canonical;
pub mod core;
pub mod stable;
pub mod types;

use std::cell::RefCell;

use candid::Principal;
use stable::{Key32, Storage};
use types::{
    ConfigRecord, CorrectionReceipt, CorrectionRecord, ExpertLoss, ExpertRecord, GovernanceEvent,
    CertifiedSnapshot, Health, InitArgs, ProtocolConstants, PublicSnapshot, Readout,
    RecordCorrectionRequest, RegisterExpertRequest, StoredReceipt, ROLE_ALL,
    ROLE_CORRECTION_WRITER, ROLE_GOVERNANCE_ADMIN, ROLE_REGISTRAR, SCHEMA_VERSION,
};

pub const VECTOR_SHA256: &str = "21c64f9ea4aef2440f3a3929497551b4d289b49fee28753d4a1f4412bef01b1b";
pub const P1_CERT_SHA256: &str = "b9d1c0d5e0d8144d88f6e1375429fc844967731090d878ebd311072ee456a57b";
const MAX_REGISTRY_SIZE: u64 = 4_096;
const MAX_PAGE_SIZE: u32 = 100;

thread_local! {
    static STATE: RefCell<Storage> = RefCell::new(Storage::init(Default::default()));
}

fn with_state<R>(function: impl FnOnce(&Storage) -> R) -> R {
    STATE.with(|state| function(&state.borrow()))
}

fn with_state_mut<R>(function: impl FnOnce(&mut Storage) -> R) -> R {
    STATE.with(|state| function(&mut state.borrow_mut()))
}

fn reject_anonymous(principal: &Principal, field: &str) -> Result<(), String> {
    if *principal == Principal::anonymous() {
        Err(format!("{field} must not be the anonymous principal"))
    } else {
        Ok(())
    }
}

fn require_role(storage: &Storage, caller: &Principal, role: u8) -> Result<(), String> {
    reject_anonymous(caller, "caller")?;
    if storage.writer_roles(caller) & role == role {
        Ok(())
    } else {
        Err("caller is not authorized for this operation".into())
    }
}

fn require_governance(storage: &Storage, caller: &Principal) -> Result<(), String> {
    let config = storage.config();
    if config.governance != *caller || storage.writer_roles(caller) & ROLE_GOVERNANCE_ADMIN == 0 {
        return Err("caller is not the active governance principal".into());
    }
    Ok(())
}

fn validate_hash(value: &[u8], field: &str) -> Result<(), String> {
    if value.len() != 32 {
        Err(format!("{field} must contain exactly 32 bytes"))
    } else {
        Ok(())
    }
}

fn validate_label(value: &str, field: &str, max_len: usize, extra: &[u8]) -> Result<(), String> {
    let bytes = value.as_bytes();
    if bytes.is_empty() || bytes.len() > max_len {
        return Err(format!("{field} length must be in 1..={max_len} bytes"));
    }
    if !bytes[0].is_ascii_lowercase() && !bytes[0].is_ascii_digit() {
        return Err(format!(
            "{field} must begin with a lowercase ASCII letter or digit"
        ));
    }
    if !bytes[bytes.len() - 1].is_ascii_lowercase() && !bytes[bytes.len() - 1].is_ascii_digit() {
        return Err(format!(
            "{field} must end with a lowercase ASCII letter or digit"
        ));
    }
    if bytes
        .iter()
        .any(|byte| !byte.is_ascii_lowercase() && !byte.is_ascii_digit() && !extra.contains(byte))
    {
        return Err(format!("{field} is not in canonical lowercase ASCII form"));
    }
    Ok(())
}

fn canonical_losses(losses: &[ExpertLoss]) -> Result<(Vec<ExpertLoss>, Vec<Key32>), String> {
    let raw_losses: Vec<u64> = losses.iter().map(|loss| loss.loss_q32).collect();
    core::validate_losses(&raw_losses)?;
    let mut keyed = losses
        .iter()
        .map(|loss| {
            Ok((
                Key32::from_slice(&loss.expert_id, "expert_id")?,
                loss.clone(),
            ))
        })
        .collect::<Result<Vec<_>, String>>()?;
    keyed.sort_by_key(|(key, _)| *key);
    if keyed.windows(2).any(|window| window[0].0 == window[1].0) {
        return Err("loss vector contains a duplicate expert_id".into());
    }
    let keys = keyed.iter().map(|(key, _)| *key).collect();
    let sorted = keyed.into_iter().map(|(_, loss)| loss).collect();
    Ok((sorted, keys))
}

pub fn initialize(
    storage: &mut Storage,
    caller: Principal,
    args: Option<InitArgs>,
    now_ns: u64,
) -> Result<(), String> {
    if storage.config().governance != Principal::anonymous() {
        return Err("canister is already initialized".into());
    }
    let governance = args.and_then(|value| value.governance).unwrap_or(caller);
    reject_anonymous(&governance, "governance")?;
    let config = ConfigRecord {
        governance,
        ..ConfigRecord::default()
    };
    storage.replace_config(config);
    storage.set_writer_roles(&governance, ROLE_ALL);
    storage.append_governance_event(
        governance,
        "initialize",
        Some(governance),
        Some(ROLE_ALL),
        now_ns,
    );
    Ok(())
}

pub fn set_writer_roles_impl(
    storage: &mut Storage,
    caller: Principal,
    principal: Principal,
    roles: u8,
    now_ns: u64,
) -> Result<GovernanceEvent, String> {
    require_governance(storage, &caller)?;
    reject_anonymous(&principal, "writer")?;
    if roles & !ROLE_ALL != 0 {
        return Err("roles contain an unknown authorization bit".into());
    }
    if principal == caller && roles & ROLE_GOVERNANCE_ADMIN == 0 {
        return Err("active governance cannot remove its own governance-admin role".into());
    }
    storage.set_writer_roles(&principal, roles);
    Ok(storage.append_governance_event(
        caller,
        "set_writer_roles",
        Some(principal),
        Some(roles),
        now_ns,
    ))
}

pub fn propose_governance_impl(
    storage: &mut Storage,
    caller: Principal,
    successor: Principal,
    now_ns: u64,
) -> Result<GovernanceEvent, String> {
    require_governance(storage, &caller)?;
    reject_anonymous(&successor, "successor")?;
    if successor == caller {
        return Err("successor must differ from active governance".into());
    }
    let mut config = storage.config();
    config.pending_governance = Some(successor);
    storage.replace_config(config);
    Ok(
        storage.append_governance_event(
            caller,
            "propose_governance",
            Some(successor),
            None,
            now_ns,
        ),
    )
}

pub fn accept_governance_impl(
    storage: &mut Storage,
    caller: Principal,
    now_ns: u64,
) -> Result<GovernanceEvent, String> {
    reject_anonymous(&caller, "caller")?;
    let mut config = storage.config();
    if config.pending_governance != Some(caller) {
        return Err("caller is not the pending governance principal".into());
    }
    let previous = config.governance;
    config.governance = caller;
    config.pending_governance = None;
    storage.replace_config(config);
    storage.set_writer_roles(&previous, 0);
    storage.set_writer_roles(&caller, ROLE_ALL);
    Ok(storage.append_governance_event(
        caller,
        "accept_governance",
        Some(previous),
        Some(ROLE_ALL),
        now_ns,
    ))
}

pub fn set_paused_impl(
    storage: &mut Storage,
    caller: Principal,
    paused: bool,
    now_ns: u64,
) -> Result<GovernanceEvent, String> {
    require_governance(storage, &caller)?;
    let mut config = storage.config();
    config.paused = paused;
    storage.replace_config(config);
    Ok(storage.append_governance_event(
        caller,
        if paused { "pause" } else { "unpause" },
        None,
        None,
        now_ns,
    ))
}

pub fn register_expert_impl(
    storage: &mut Storage,
    caller: Principal,
    request: RegisterExpertRequest,
    now_ns: u64,
) -> Result<ExpertRecord, String> {
    require_role(storage, &caller, ROLE_REGISTRAR)?;
    let config = storage.config();
    if config.paused {
        return Err("canister is paused".into());
    }
    reject_anonymous(&request.owner, "expert owner")?;
    validate_label(&request.system_id, "system_id", 128, b"-._:/")?;
    validate_label(&request.version, "version", 64, b"-._+")?;
    if let Some(metadata_hash) = &request.metadata_hash {
        validate_hash(metadata_hash, "metadata_hash")?;
    }
    let key = Key32(canonical::expert_id(
        &request.owner,
        &request.system_id,
        &request.version,
    ));
    if let Some(existing) = storage.expert(&key) {
        if existing.owner == request.owner
            && existing.system_id == request.system_id
            && existing.version == request.version
            && existing.metadata_hash == request.metadata_hash
        {
            return Ok(existing);
        }
        return Err("canonical expert_id is already bound to different metadata".into());
    }
    if config.active_set_locked {
        return Err("active expert set is locked after the first correction".into());
    }
    if storage.expert_count() >= MAX_REGISTRY_SIZE {
        return Err(format!("expert registry limit {MAX_REGISTRY_SIZE} reached"));
    }
    if storage.active_expert_count() >= core::K_MAX as u64 {
        return Err(format!("active expert limit {} reached", core::K_MAX));
    }
    let record = ExpertRecord {
        expert_id: key.to_vec(),
        owner: request.owner,
        system_id: request.system_id,
        version: request.version,
        metadata_hash: request.metadata_hash,
        active: true,
        registered_at_ns: now_ns,
    };
    storage.insert_expert(key, record.clone());
    storage.set_penalty(key, 0);
    Ok(record)
}

pub fn set_expert_active_impl(
    storage: &mut Storage,
    caller: Principal,
    expert_id: Vec<u8>,
    active: bool,
) -> Result<ExpertRecord, String> {
    require_role(storage, &caller, ROLE_REGISTRAR)?;
    let config = storage.config();
    if config.paused {
        return Err("canister is paused".into());
    }
    if config.active_set_locked {
        return Err("active expert set is locked after the first correction".into());
    }
    let key = Key32::from_slice(&expert_id, "expert_id")?;
    let mut record = storage
        .expert(&key)
        .ok_or_else(|| "expert_id is not registered".to_string())?;
    if active && !record.active && storage.active_expert_count() >= core::K_MAX as u64 {
        return Err(format!("active expert limit {} reached", core::K_MAX));
    }
    record.active = active;
    storage.insert_expert(key, record.clone());
    Ok(record)
}

pub fn record_correction_impl(
    storage: &mut Storage,
    caller: Principal,
    request: RecordCorrectionRequest,
    now_ns: u64,
) -> Result<CorrectionReceipt, String> {
    require_role(storage, &caller, ROLE_CORRECTION_WRITER)?;
    let config = storage.config();
    if config.paused {
        return Err("canister is paused".into());
    }
    if request.external_event_id.is_empty() || request.external_event_id.len() > 128 {
        return Err("external_event_id length must be in 1..=128 bytes".into());
    }
    validate_hash(&request.metric_id, "metric_id")?;
    let (losses, supplied_keys) = canonical_losses(&request.losses)?;
    let payload_hash = canonical::payload_hash(&request.metric_id, &losses);
    let event_key = Key32(canonical::event_key(
        &request.metric_id,
        &request.external_event_id,
    ));
    if let Some(stored) = storage.receipt(&event_key) {
        if stored.payload_hash == payload_hash {
            return Ok(stored.response(true));
        }
        return Err("idempotency conflict: event key already has a different payload".into());
    }
    let current_seq = storage.sequence();
    if let Some(expected_seq) = request.expected_seq {
        if expected_seq != current_seq {
            return Err(format!(
                "sequence conflict: expected {expected_seq}, current {current_seq}"
            ));
        }
    }
    let active = storage.active_experts();
    if !(2..=core::K_MAX).contains(&active.len()) {
        return Err(format!(
            "active expert count must be in 2..={}",
            core::K_MAX
        ));
    }
    let active_keys: Vec<Key32> = active.iter().map(|(key, _)| *key).collect();
    if supplied_keys != active_keys {
        return Err("losses must contain every active expert_id exactly once".into());
    }
    let penalties: Vec<i128> = active_keys.iter().map(|key| storage.penalty(key)).collect();
    let loss_values: Vec<u64> = losses.iter().map(|loss| loss.loss_q32).collect();
    let updated_penalties = core::accumulate_penalties(&penalties, &loss_values)?;
    let priors = core::uniform_priors(active.len())?;
    let weights = core::weights_v1(&updated_penalties, &priors)?;
    let seq = current_seq
        .checked_add(1)
        .ok_or_else(|| "correction sequence overflow".to_string())?;
    let correction_id = canonical::correction_id(&event_key.0, &payload_hash);
    let expert_ids: Vec<[u8; 32]> = active_keys.iter().map(|key| key.0).collect();
    let state_root = canonical::next_state_root(
        &config.state_root,
        seq,
        &correction_id,
        &payload_hash,
        &expert_ids,
        &updated_penalties,
        &weights,
    );
    let correction = CorrectionRecord {
        seq,
        event_key: event_key.to_vec(),
        correction_id: correction_id.to_vec(),
        payload_hash: payload_hash.to_vec(),
        writer: caller,
        external_event_id: request.external_event_id,
        metric_id: request.metric_id,
        losses,
        accepted_at_ns: now_ns,
    };
    let stored = StoredReceipt {
        seq,
        correction_id: correction_id.to_vec(),
        payload_hash: payload_hash.to_vec(),
        state_root: state_root.to_vec(),
        weights,
    };
    let mut next_config = config;
    next_config.active_set_locked = true;
    next_config.state_root = state_root.to_vec();
    let penalty_updates: Vec<(Key32, i128)> =
        active_keys.into_iter().zip(updated_penalties).collect();
    storage.commit_correction(
        seq,
        &penalty_updates,
        correction,
        event_key,
        stored.clone(),
        next_config,
    );
    Ok(stored.response(false))
}

pub fn readout_impl(storage: &Storage) -> Result<Readout, String> {
    let active = storage.active_experts();
    if !(2..=core::K_MAX).contains(&active.len()) {
        return Err(format!(
            "active expert count must be in 2..={}",
            core::K_MAX
        ));
    }
    let penalties: Vec<i128> = active.iter().map(|(key, _)| storage.penalty(key)).collect();
    let priors = core::uniform_priors(active.len())?;
    let weights = core::weights_v1(&penalties, &priors)?;
    let config = storage.config();
    Ok(Readout {
        seq: storage.sequence(),
        expert_ids: active.iter().map(|(key, _)| key.to_vec()).collect(),
        weights,
        state_root: config.state_root,
    })
}

pub fn health_impl(storage: &Storage) -> Health {
    let config = storage.config();
    Health {
        schema_version: config.schema_version,
        seq: storage.sequence(),
        expert_count: storage.expert_count(),
        active_expert_count: storage.active_expert_count(),
        correction_count: storage.correction_count(),
        receipt_count: storage.receipt_count(),
        writer_count: storage.writer_count(),
        governance_event_count: storage.governance_event_count(),
        paused: config.paused,
        active_set_locked: config.active_set_locked,
        governance: config.governance,
        state_root: config.state_root,
    }
}

pub fn constants_impl() -> ProtocolConstants {
    ProtocolConstants {
        contract: "mirra.v1-pilot".into(),
        schema_version: SCHEMA_VERSION,
        k_min: 2,
        k_max: core::K_MAX as u32,
        scale: core::SCALE.to_string(),
        b_q32: core::B_Q32,
        floor_units: core::FLOOR_UNITS,
        eta_q32: core::ETA_Q32,
        eta_text: "1/8".into(),
        loss_min_q32: core::LOSS_MIN_Q32,
        loss_max_q32: core::LOSS_MAX_Q32,
        loss_encoding: "unsigned Q32.32 in inclusive [0,1]; smaller is better".into(),
        expert_id_encoding:
            "sha256(domain || len(owner) || owner || len(system_id) || system_id || len(version) || version)"
                .into(),
        idempotency_scope:
            "global sha256(metric_id,external_event_id); identical retry returns original receipt; conflicting payload rejects"
                .into(),
        vector_sha256: VECTOR_SHA256.into(),
        p1_cert_sha256: P1_CERT_SHA256.into(),
    }
}

pub fn public_snapshot_impl(storage: &Storage) -> PublicSnapshot {
    let config = storage.config();
    PublicSnapshot {
        protocol: "mirra.v1-pilot".into(),
        protocol_fingerprint: canonical::protocol_fingerprint(
            SCHEMA_VERSION,
            VECTOR_SHA256,
            P1_CERT_SHA256,
        )
        .to_vec(),
        schema_version: SCHEMA_VERSION,
        seq: storage.sequence(),
        expert_count: storage.expert_count(),
        state_root: config.state_root,
        vector_sha256: VECTOR_SHA256.into(),
        p1_cert_sha256: P1_CERT_SHA256.into(),
    }
}

pub fn snapshot_commitment_impl(storage: &Storage) -> [u8; 32] {
    canonical::snapshot_commitment(&public_snapshot_impl(storage))
}

fn set_certified_snapshot(storage: &Storage) {
    ic_cdk::api::set_certified_data(&snapshot_commitment_impl(storage));
}

#[ic_cdk::init]
fn init(args: Option<InitArgs>) {
    with_state_mut(|storage| {
        initialize(storage, ic_cdk::caller(), args, ic_cdk::api::time())
            .unwrap_or_else(|error| ic_cdk::trap(&error));
        set_certified_snapshot(storage);
    });
}

#[ic_cdk::post_upgrade]
fn post_upgrade() {
    with_state(|storage| {
        let config = storage.config();
        if config.schema_version != SCHEMA_VERSION {
            ic_cdk::trap("unsupported MIRRA stable schema after upgrade");
        }
        set_certified_snapshot(storage);
    });
}

#[ic_cdk::update]
fn set_writer_roles(principal: Principal, roles: u8) -> Result<GovernanceEvent, String> {
    with_state_mut(|storage| {
        set_writer_roles_impl(
            storage,
            ic_cdk::caller(),
            principal,
            roles,
            ic_cdk::api::time(),
        )
    })
}

#[ic_cdk::update]
fn propose_governance(successor: Principal) -> Result<GovernanceEvent, String> {
    with_state_mut(|storage| {
        propose_governance_impl(storage, ic_cdk::caller(), successor, ic_cdk::api::time())
    })
}

#[ic_cdk::update]
fn accept_governance() -> Result<GovernanceEvent, String> {
    with_state_mut(|storage| accept_governance_impl(storage, ic_cdk::caller(), ic_cdk::api::time()))
}

#[ic_cdk::update]
fn set_paused(paused: bool) -> Result<GovernanceEvent, String> {
    with_state_mut(|storage| {
        set_paused_impl(storage, ic_cdk::caller(), paused, ic_cdk::api::time())
    })
}

#[ic_cdk::update]
fn register_expert(request: RegisterExpertRequest) -> Result<ExpertRecord, String> {
    with_state_mut(|storage| {
        let response =
            register_expert_impl(storage, ic_cdk::caller(), request, ic_cdk::api::time())?;
        set_certified_snapshot(storage);
        Ok(response)
    })
}

#[ic_cdk::update]
fn set_expert_active(expert_id: Vec<u8>, active: bool) -> Result<ExpertRecord, String> {
    with_state_mut(|storage| set_expert_active_impl(storage, ic_cdk::caller(), expert_id, active))
}

#[ic_cdk::update]
fn record_correction(request: RecordCorrectionRequest) -> Result<CorrectionReceipt, String> {
    with_state_mut(|storage| {
        let response =
            record_correction_impl(storage, ic_cdk::caller(), request, ic_cdk::api::time())?;
        set_certified_snapshot(storage);
        Ok(response)
    })
}

#[ic_cdk::query]
fn get_readout() -> Result<Readout, String> {
    with_state(readout_impl)
}

#[ic_cdk::query]
fn health() -> Health {
    with_state(health_impl)
}

#[ic_cdk::query]
fn protocol_constants() -> ProtocolConstants {
    constants_impl()
}

#[ic_cdk::query]
fn public_snapshot() -> PublicSnapshot {
    with_state(public_snapshot_impl)
}

#[ic_cdk::query]
fn certified_snapshot() -> CertifiedSnapshot {
    with_state(|storage| {
        let snapshot = public_snapshot_impl(storage);
        CertifiedSnapshot {
            snapshot_commitment: canonical::snapshot_commitment(&snapshot).to_vec(),
            snapshot,
            certificate: ic_cdk::api::data_certificate(),
        }
    })
}

#[ic_cdk::query]
fn get_expert(expert_id: Vec<u8>) -> Result<Option<ExpertRecord>, String> {
    let key = Key32::from_slice(&expert_id, "expert_id")?;
    Ok(with_state(|storage| storage.expert(&key)))
}

#[ic_cdk::query]
fn list_experts(start_after: Option<Vec<u8>>, limit: u32) -> Result<Vec<ExpertRecord>, String> {
    let start = start_after
        .as_deref()
        .map(|value| Key32::from_slice(value, "start_after"))
        .transpose()?;
    let bounded_limit = limit.clamp(1, MAX_PAGE_SIZE) as usize;
    Ok(with_state(|storage| {
        storage.expert_page(start, bounded_limit)
    }))
}

#[ic_cdk::query]
fn get_correction(sequence: u64) -> Option<CorrectionRecord> {
    with_state(|storage| storage.correction(sequence))
}

#[ic_cdk::query]
fn get_governance_event(sequence: u64) -> Option<GovernanceEvent> {
    with_state(|storage| storage.governance_event(sequence))
}

#[ic_cdk::query]
fn get_writer_roles(principal: Principal) -> u8 {
    with_state(|storage| storage.writer_roles(&principal))
}

ic_cdk::export_candid!();
