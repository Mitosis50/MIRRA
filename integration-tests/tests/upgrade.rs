use std::{env, fs, time::SystemTime};

use candid::{Decode, Encode, Principal};
use ic_agent::{Agent, Certificate};
use ic_certification::LookupResult;
use mirra_canister::canonical;
use mirra_canister::types::{
    CertifiedSnapshot, CorrectionReceipt, ExpertLoss, ExpertRecord, Health, InitArgs, Readout,
    RecordCorrectionRequest, RegisterExpertRequest, ROLE_CORRECTION_WRITER,
};
use pocket_ic::{PocketIc, PocketIcBuilder, RejectResponse};

fn principal(tag: u8) -> Principal {
    Principal::from_slice(&[tag, 1])
}

fn reply(result: Result<Vec<u8>, RejectResponse>) -> Vec<u8> {
    result.expect("PocketIC call failed")
}

fn update(
    pocket_ic: &PocketIc,
    canister: Principal,
    caller: Principal,
    method: &str,
    payload: Vec<u8>,
) -> Vec<u8> {
    reply(pocket_ic.update_call(canister, caller, method, payload))
}

fn query(pocket_ic: &PocketIc, canister: Principal, method: &str, payload: Vec<u8>) -> Vec<u8> {
    reply(pocket_ic.query_call(canister, Principal::anonymous(), method, payload))
}

fn register(
    pocket_ic: &PocketIc,
    canister: Principal,
    governance: Principal,
    owner: Principal,
    name: &str,
) -> ExpertRecord {
    let request = RegisterExpertRequest {
        owner,
        system_id: name.into(),
        version: "v1.0.0".into(),
        metadata_hash: None,
    };
    Decode!(
        &update(
            pocket_ic,
            canister,
            governance,
            "register_expert",
            Encode!(&request).unwrap(),
        ),
        Result<ExpertRecord, String>
    )
    .unwrap()
    .unwrap()
}

fn correction(event: u8, ids: &[Vec<u8>], losses: [u64; 2]) -> RecordCorrectionRequest {
    RecordCorrectionRequest {
        external_event_id: vec![event],
        metric_id: vec![7; 32],
        losses: vec![
            ExpertLoss {
                expert_id: ids[1].clone(),
                loss_q32: losses[1],
            },
            ExpertLoss {
                expert_id: ids[0].clone(),
                loss_q32: losses[0],
            },
        ],
        expected_seq: Some(event as u64 - 1),
    }
}

fn health(pocket_ic: &PocketIc, canister: Principal) -> Health {
    Decode!(
        &query(pocket_ic, canister, "health", Encode!().unwrap()),
        Health
    )
    .unwrap()
}

fn readout(pocket_ic: &PocketIc, canister: Principal) -> Readout {
    Decode!(
        &query(pocket_ic, canister, "get_readout", Encode!().unwrap()),
        Result<Readout, String>
    )
    .unwrap()
    .unwrap()
}

#[test]
fn authorization_idempotency_and_upgrade_preserve_state() {
    let wasm_path = env::var("MIRRA_WASM").expect("MIRRA_WASM must point to the release Wasm");
    let upgrade_wasm_path = env::var("MIRRA_UPGRADE_WASM").unwrap_or_else(|_| wasm_path.clone());
    let wasm = fs::read(wasm_path).expect("failed to read install Wasm");
    let upgrade_wasm = fs::read(upgrade_wasm_path).expect("failed to read upgrade Wasm");
    let pocket_ic = PocketIcBuilder::new()
        .with_nns_subnet()
        .with_application_subnet()
        .build();
    let root_key = pocket_ic
        .root_key()
        .expect("PocketIC instance with NNS subnet has no root key");
    let application_subnet = pocket_ic.topology().get_app_subnets()[0];
    let canister = pocket_ic.create_canister_on_subnet(None, None, application_subnet);
    pocket_ic.add_cycles(canister, 10_000_000_000_000);

    let governance = principal(1);
    let writer = principal(2);
    let outsider = principal(99);
    pocket_ic.install_canister(
        canister,
        wasm,
        Encode!(&Some(InitArgs {
            governance: Some(governance),
        }))
        .unwrap(),
        None,
    );
    let grant = update(
        &pocket_ic,
        canister,
        governance,
        "set_writer_roles",
        Encode!(&writer, &ROLE_CORRECTION_WRITER).unwrap(),
    );
    Decode!(&grant, Result<mirra_canister::types::GovernanceEvent, String>)
        .unwrap()
        .unwrap();
    let alpha = register(&pocket_ic, canister, governance, principal(10), "alpha");
    let beta = register(&pocket_ic, canister, governance, principal(11), "beta");
    let ids = vec![alpha.expert_id, beta.expert_id];

    let first_request = correction(1, &ids, [0, 1 << 32]);
    let health_before_rejection = health(&pocket_ic, canister);
    let readout_before_rejection = readout(&pocket_ic, canister);
    let unauthorized = update(
        &pocket_ic,
        canister,
        outsider,
        "record_correction",
        Encode!(&first_request).unwrap(),
    );
    assert!(
        Decode!(&unauthorized, Result<CorrectionReceipt, String>)
            .unwrap()
            .is_err(),
        "non-writer correction unexpectedly succeeded"
    );
    assert_eq!(health(&pocket_ic, canister), health_before_rejection);
    assert_eq!(readout(&pocket_ic, canister), readout_before_rejection);
    let first = Decode!(
        &update(
            &pocket_ic,
            canister,
            writer,
            "record_correction",
            Encode!(&first_request).unwrap(),
        ),
        Result<CorrectionReceipt, String>
    )
    .unwrap()
    .unwrap();
    assert_eq!(first.seq, 1);
    assert!(!first.replayed);
    assert_eq!(health(&pocket_ic, canister).correction_count, 1);
    let before_upgrade = readout(&pocket_ic, canister);
    let certified_before = verify_certified_snapshot(&pocket_ic, canister, root_key.clone());

    // Demonstrate correction -> identical retry -> upgrade -> next correction.
    // Retry once before and once after upgrade, checking the entire receipt.
    let mut expected_replay = first.clone();
    expected_replay.replayed = true;
    let retry_before_upgrade = Decode!(
        &update(
            &pocket_ic,
            canister,
            writer,
            "record_correction",
            Encode!(&first_request).unwrap(),
        ),
        Result<CorrectionReceipt, String>
    )
    .unwrap()
    .unwrap();
    assert_eq!(retry_before_upgrade, expected_replay);
    assert_eq!(health(&pocket_ic, canister).correction_count, 1);
    assert_eq!(health(&pocket_ic, canister).receipt_count, 1);
    assert_eq!(readout(&pocket_ic, canister), before_upgrade);
    let certified_retry = verify_certified_snapshot(&pocket_ic, canister, root_key.clone());
    assert_eq!(certified_retry.snapshot, certified_before.snapshot);

    pocket_ic
        .upgrade_canister(canister, upgrade_wasm, Encode!().unwrap(), None)
        .expect("canister upgrade failed");
    let immediately_after = readout(&pocket_ic, canister);
    assert_eq!(health(&pocket_ic, canister).correction_count, 1);
    assert_eq!(immediately_after, before_upgrade);
    assert_eq!(immediately_after.state_root, first.state_root);
    assert_eq!(immediately_after.weights, first.weights);
    let certified_after = verify_certified_snapshot(&pocket_ic, canister, root_key.clone());
    assert_eq!(certified_after.snapshot, certified_before.snapshot);
    assert_eq!(
        certified_after.snapshot.state_root,
        immediately_after.state_root
    );

    let replay = Decode!(
        &update(
            &pocket_ic,
            canister,
            writer,
            "record_correction",
            Encode!(&first_request).unwrap(),
        ),
        Result<CorrectionReceipt, String>
    )
    .unwrap()
    .unwrap();
    assert_eq!(replay, expected_replay);
    assert_eq!(health(&pocket_ic, canister).correction_count, 1);
    assert_eq!(health(&pocket_ic, canister).receipt_count, 1);
    assert_eq!(readout(&pocket_ic, canister), before_upgrade);

    let second = Decode!(
        &update(
            &pocket_ic,
            canister,
            writer,
            "record_correction",
            Encode!(&correction(2, &ids, [1 << 32, 0])).unwrap(),
        ),
        Result<CorrectionReceipt, String>
    )
    .unwrap()
    .unwrap();
    assert_eq!(second.seq, 2);
    assert!(!second.replayed);
    assert_eq!(health(&pocket_ic, canister).correction_count, 2);
    assert_eq!(health(&pocket_ic, canister).receipt_count, 2);
    assert_ne!(second.correction_id, first.correction_id);
    let certified_second = verify_certified_snapshot(&pocket_ic, canister, root_key);
    assert_eq!(certified_second.snapshot.seq, second.seq);
    assert_eq!(certified_second.snapshot.state_root, second.state_root);
    println!("MIRRA_RUNTIME_OK correction=1 retry=1 upgrade=1 retry_after_upgrade=1 next=2");
}

fn verify_certified_snapshot(
    pocket_ic: &PocketIc,
    canister: Principal,
    root_key: Vec<u8>,
) -> CertifiedSnapshot {
    // Keep freshness checks enabled: synchronize the emulator's certified clock
    // with the verifier's wall clock instead of accepting arbitrarily old data.
    pocket_ic.set_certified_time(SystemTime::now().into());
    let snapshot = Decode!(
        &query(
            pocket_ic,
            canister,
            "certified_snapshot",
            Encode!().unwrap()
        ),
        CertifiedSnapshot
    )
    .unwrap();
    assert_eq!(
        snapshot.snapshot_commitment,
        canonical::snapshot_commitment(&snapshot.snapshot).to_vec(),
        "returned snapshot does not match its canonical commitment",
    );
    let certificate_bytes = snapshot
        .certificate
        .as_deref()
        .expect("certified query returned no data certificate");
    let certificate: Certificate =
        serde_cbor::from_slice(certificate_bytes).expect("invalid certificate CBOR");
    assert!(
        certificate.delegation.is_some(),
        "application-subnet certificate must exercise delegation to the NNS root",
    );

    // This agent is used only for offline verification, never for HTTP calls.
    let agent = Agent::builder()
        .with_url("http://127.0.0.1")
        .build()
        .expect("failed to construct certificate verifier");
    agent.set_root_key(root_key);
    agent
        .verify(&certificate, canister)
        .expect("certificate does not verify against the PocketIC root key");
    assert!(
        agent
            .verify(&certificate, Principal::management_canister())
            .is_err(),
        "application-subnet certificate unexpectedly authorizes an out-of-range canister",
    );
    let mut forged_certificate = certificate.clone();
    *forged_certificate
        .signature
        .first_mut()
        .expect("certificate has an empty signature") ^= 1;
    assert!(
        agent.verify(&forged_certificate, canister).is_err(),
        "altered certificate signature unexpectedly verified",
    );

    let certified_data = certificate.tree.lookup_path([
        b"canister".as_slice(),
        canister.as_slice(),
        b"certified_data".as_slice(),
    ]);
    assert_eq!(
        certified_data,
        LookupResult::Found(snapshot.snapshot_commitment.as_slice()),
        "certificate does not bind MIRRA's snapshot commitment",
    );
    let mut wrong_root = agent.read_root_key();
    *wrong_root.last_mut().expect("empty root key") ^= 1;
    agent.set_root_key(wrong_root);
    assert!(
        agent.verify(&certificate, canister).is_err(),
        "certificate unexpectedly verified against an altered root key",
    );
    snapshot
}
