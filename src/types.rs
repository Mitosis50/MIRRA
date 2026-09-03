use candid::{CandidType, Principal};
use serde::Deserialize;

pub const SCHEMA_VERSION: u32 = 1;
pub const ROLE_CORRECTION_WRITER: u8 = 1;
pub const ROLE_REGISTRAR: u8 = 1 << 1;
pub const ROLE_AUDITOR: u8 = 1 << 2;
pub const ROLE_GOVERNANCE_ADMIN: u8 = 1 << 3;
pub const ROLE_ALL: u8 =
    ROLE_CORRECTION_WRITER | ROLE_REGISTRAR | ROLE_AUDITOR | ROLE_GOVERNANCE_ADMIN;

#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct ConfigRecord {
    pub schema_version: u32,
    pub governance: Principal,
    pub pending_governance: Option<Principal>,
    pub paused: bool,
    pub active_set_locked: bool,
    pub state_root: Vec<u8>,
}

impl Default for ConfigRecord {
    fn default() -> Self {
        Self {
            schema_version: SCHEMA_VERSION,
            governance: Principal::anonymous(),
            pending_governance: None,
            paused: false,
            active_set_locked: false,
            state_root: vec![0; 32],
        }
    }
}

#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct ExpertRecord {
    pub expert_id: Vec<u8>,
    pub owner: Principal,
    pub system_id: String,
    pub version: String,
    pub metadata_hash: Option<Vec<u8>>,
    pub active: bool,
    pub registered_at_ns: u64,
}

#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct ExpertLoss {
    pub expert_id: Vec<u8>,
    pub loss_q32: u64,
}

#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct RegisterExpertRequest {
    pub owner: Principal,
    pub system_id: String,
    pub version: String,
    pub metadata_hash: Option<Vec<u8>>,
}

#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct RecordCorrectionRequest {
    pub external_event_id: Vec<u8>,
    pub metric_id: Vec<u8>,
    pub losses: Vec<ExpertLoss>,
    pub expected_seq: Option<u64>,
}

#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct CorrectionRecord {
    pub seq: u64,
    pub event_key: Vec<u8>,
    pub correction_id: Vec<u8>,
    pub payload_hash: Vec<u8>,
    pub writer: Principal,
    pub external_event_id: Vec<u8>,
    pub metric_id: Vec<u8>,
    pub losses: Vec<ExpertLoss>,
    pub accepted_at_ns: u64,
}

#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct CorrectionReceipt {
    pub seq: u64,
    pub correction_id: Vec<u8>,
    pub payload_hash: Vec<u8>,
    pub state_root: Vec<u8>,
    pub weights: Vec<u64>,
    pub replayed: bool,
}

#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct StoredReceipt {
    pub seq: u64,
    pub correction_id: Vec<u8>,
    pub payload_hash: Vec<u8>,
    pub state_root: Vec<u8>,
    pub weights: Vec<u64>,
}

impl StoredReceipt {
    pub fn response(&self, replayed: bool) -> CorrectionReceipt {
        CorrectionReceipt {
            seq: self.seq,
            correction_id: self.correction_id.clone(),
            payload_hash: self.payload_hash.clone(),
            state_root: self.state_root.clone(),
            weights: self.weights.clone(),
            replayed,
        }
    }
}

#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct GovernanceEvent {
    pub seq: u64,
    pub actor: Principal,
    pub action: String,
    pub subject: Option<Principal>,
    pub roles: Option<u8>,
    pub recorded_at_ns: u64,
}

#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct Readout {
    pub seq: u64,
    pub expert_ids: Vec<Vec<u8>>,
    pub weights: Vec<u64>,
    pub state_root: Vec<u8>,
}

#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct ProtocolConstants {
    pub contract: String,
    pub schema_version: u32,
    pub k_min: u32,
    pub k_max: u32,
    pub scale: String,
    pub b_q32: i64,
    pub floor_units: u64,
    pub eta_q32: u64,
    pub eta_text: String,
    pub loss_min_q32: u64,
    pub loss_max_q32: u64,
    pub loss_encoding: String,
    pub expert_id_encoding: String,
    pub idempotency_scope: String,
    pub vector_sha256: String,
    pub p1_cert_sha256: String,
}

#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct Health {
    pub schema_version: u32,
    pub seq: u64,
    pub expert_count: u64,
    pub active_expert_count: u64,
    pub correction_count: u64,
    pub receipt_count: u64,
    pub writer_count: u64,
    pub governance_event_count: u64,
    pub paused: bool,
    pub active_set_locked: bool,
    pub governance: Principal,
    pub state_root: Vec<u8>,
}

#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct PublicSnapshot {
    pub protocol: String,
    pub protocol_fingerprint: Vec<u8>,
    pub schema_version: u32,
    pub seq: u64,
    pub expert_count: u64,
    pub state_root: Vec<u8>,
    pub vector_sha256: String,
    pub p1_cert_sha256: String,
}

#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct CertifiedSnapshot {
    pub snapshot: PublicSnapshot,
    pub snapshot_commitment: Vec<u8>,
    pub certificate: Option<Vec<u8>>,
}

#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct InitArgs {
    pub governance: Option<Principal>,
}
