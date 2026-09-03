use candid::Principal;
use ic_stable_structures::DefaultMemoryImpl;

use mirra_canister::{
    accept_governance_impl, health_impl, initialize, propose_governance_impl, public_snapshot_impl,
    readout_impl, record_correction_impl, register_expert_impl, set_expert_active_impl,
    set_writer_roles_impl,
    stable::Storage,
    types::{
        ExpertLoss, RecordCorrectionRequest, RegisterExpertRequest, ROLE_CORRECTION_WRITER,
        ROLE_REGISTRAR,
    },
};

fn principal(tag: u8) -> Principal {
    Principal::from_slice(&[tag, 1])
}

fn expert_request(owner: Principal, name: &str) -> RegisterExpertRequest {
    RegisterExpertRequest {
        owner,
        system_id: name.into(),
        version: "v1.0.0".into(),
        metadata_hash: Some(vec![name.as_bytes()[0]; 32]),
    }
}

fn setup() -> (
    DefaultMemoryImpl,
    Storage,
    Principal,
    Principal,
    Vec<Vec<u8>>,
) {
    let memory = DefaultMemoryImpl::default();
    let mut storage = Storage::init(memory.clone());
    let governance = principal(1);
    let writer = principal(2);
    initialize(&mut storage, governance, None, 10).unwrap();
    set_writer_roles_impl(&mut storage, governance, writer, ROLE_CORRECTION_WRITER, 11).unwrap();
    let alpha = register_expert_impl(
        &mut storage,
        governance,
        expert_request(principal(10), "alpha"),
        12,
    )
    .unwrap();
    let beta = register_expert_impl(
        &mut storage,
        governance,
        expert_request(principal(11), "beta"),
        13,
    )
    .unwrap();
    (
        memory,
        storage,
        governance,
        writer,
        vec![alpha.expert_id, beta.expert_id],
    )
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

#[test]
fn canonical_identity_and_registration_retries_are_stable() {
    let (_, mut storage, governance, _, _) = setup();
    let request = expert_request(principal(12), "gamma/model");
    let first = register_expert_impl(&mut storage, governance, request.clone(), 20).unwrap();
    let retry = register_expert_impl(&mut storage, governance, request, 999).unwrap();
    assert_eq!(first, retry);
    assert_eq!(first.registered_at_ns, 20);
    assert!(register_expert_impl(
        &mut storage,
        governance,
        expert_request(principal(13), "Not-Canonical"),
        21,
    )
    .is_err());
}

#[test]
fn correction_retries_are_idempotent_and_conflicts_do_not_mutate_state() {
    let (_, mut storage, governance, writer, ids) = setup();
    let request = correction(1, &ids, [0, 1 << 32]);
    let first = record_correction_impl(&mut storage, writer, request.clone(), 100).unwrap();
    assert_eq!(first.seq, 1);
    assert!(!first.replayed);

    for time in 101..1_101 {
        let retry = record_correction_impl(&mut storage, writer, request.clone(), time).unwrap();
        assert!(retry.replayed);
        assert_eq!(retry.seq, first.seq);
        assert_eq!(retry.correction_id, first.correction_id);
        assert_eq!(retry.state_root, first.state_root);
        assert_eq!(retry.weights, first.weights);
    }
    let mut conflict = request;
    conflict.losses[0].loss_q32 = 0;
    assert!(record_correction_impl(&mut storage, writer, conflict, 1_102).is_err());
    let health = health_impl(&storage);
    assert_eq!(health.seq, 1);
    assert_eq!(health.correction_count, 1);
    assert_eq!(health.receipt_count, 1);

    set_writer_roles_impl(&mut storage, governance, writer, 0, 1_103).unwrap();
    assert!(record_correction_impl(
        &mut storage,
        writer,
        correction(1, &ids, [0, 1 << 32]),
        1_104,
    )
    .is_err());
}

#[test]
fn writer_rotation_preserves_global_event_identity() {
    let (_, mut storage, governance, first_writer, ids) = setup();
    let request = correction(1, &ids, [0, 1 << 32]);
    let first = record_correction_impl(&mut storage, first_writer, request.clone(), 100).unwrap();
    let second_writer = principal(3);
    set_writer_roles_impl(&mut storage, governance, first_writer, 0, 101).unwrap();
    set_writer_roles_impl(
        &mut storage,
        governance,
        second_writer,
        ROLE_CORRECTION_WRITER,
        102,
    )
    .unwrap();
    let retry = record_correction_impl(&mut storage, second_writer, request, 103).unwrap();
    assert!(retry.replayed);
    assert_eq!(retry.correction_id, first.correction_id);
    assert_eq!(health_impl(&storage).correction_count, 1);
    assert_eq!(storage.correction(1).unwrap().writer, first_writer);
}

#[test]
fn stable_reopen_restores_hash_weights_receipts_and_unique_sequences() {
    let (memory, mut storage, _, writer, ids) = setup();
    let first =
        record_correction_impl(&mut storage, writer, correction(1, &ids, [0, 1 << 32]), 100)
            .unwrap();
    let before = readout_impl(&storage).unwrap();
    drop(storage);

    let mut restored = Storage::init(memory);
    let immediately_after = readout_impl(&restored).unwrap();
    assert_eq!(before, immediately_after);
    assert_eq!(health_impl(&restored).correction_count, 1);
    let retry = record_correction_impl(
        &mut restored,
        writer,
        correction(1, &ids, [0, 1 << 32]),
        101,
    )
    .unwrap();
    assert!(retry.replayed);
    assert_eq!(retry.state_root, first.state_root);
    assert_eq!(retry.weights, first.weights);

    let second = record_correction_impl(
        &mut restored,
        writer,
        correction(2, &ids, [1 << 32, 0]),
        102,
    )
    .unwrap();
    assert_eq!(second.seq, 2);
    assert_ne!(first.correction_id, second.correction_id);
    assert_eq!(health_impl(&restored).correction_count, 2);
    assert_eq!(restored.correction(1).unwrap().seq, 1);
    assert_eq!(restored.correction(2).unwrap().seq, 2);
}

#[test]
fn authorization_governance_transfer_and_active_set_lock_are_enforced() {
    let (_, mut storage, governance, writer, ids) = setup();
    let outsider = principal(99);
    assert!(record_correction_impl(
        &mut storage,
        outsider,
        correction(1, &ids, [0, 1 << 32]),
        100,
    )
    .is_err());
    record_correction_impl(&mut storage, writer, correction(1, &ids, [0, 1 << 32]), 101).unwrap();
    assert!(register_expert_impl(
        &mut storage,
        governance,
        expert_request(principal(12), "gamma"),
        102,
    )
    .is_err());
    assert!(set_expert_active_impl(&mut storage, governance, ids[0].clone(), false).is_err());

    let successor = principal(4);
    propose_governance_impl(&mut storage, governance, successor, 103).unwrap();
    accept_governance_impl(&mut storage, successor, 104).unwrap();
    assert_eq!(storage.writer_roles(&governance), 0);
    assert!(
        set_writer_roles_impl(&mut storage, governance, outsider, ROLE_REGISTRAR, 105,).is_err()
    );
    set_writer_roles_impl(&mut storage, successor, outsider, ROLE_REGISTRAR, 106).unwrap();
    assert_eq!(storage.writer_roles(&outsider), ROLE_REGISTRAR);
    assert_eq!(storage.governance_event_count(), 5);
}

#[test]
fn two_clean_deployments_produce_identical_transactions_and_state() {
    let (_, mut first, _, first_writer, first_ids) = setup();
    let (_, mut second, _, second_writer, second_ids) = setup();
    assert_eq!(first_ids, second_ids);
    for (event, losses, time) in [
        (1, [0, 1 << 32], 100),
        (2, [1 << 31, 1 << 30], 101),
        (3, [1 << 32, 0], 102),
    ] {
        let first_receipt = record_correction_impl(
            &mut first,
            first_writer,
            correction(event, &first_ids, losses),
            time,
        )
        .unwrap();
        let second_receipt = record_correction_impl(
            &mut second,
            second_writer,
            correction(event, &second_ids, losses),
            time,
        )
        .unwrap();
        assert_eq!(first_receipt, second_receipt);
        assert_eq!(
            first.correction(event as u64),
            second.correction(event as u64)
        );
    }
    assert_eq!(
        readout_impl(&first).unwrap(),
        readout_impl(&second).unwrap()
    );
    assert_eq!(health_impl(&first), health_impl(&second));
    assert_eq!(public_snapshot_impl(&first), public_snapshot_impl(&second));
}
