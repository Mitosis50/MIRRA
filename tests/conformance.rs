//! Bit-exact conformance against the frozen 99-vector Python corpus.

use serde::Deserialize;

use candid::Principal;
#[rustfmt::skip]
use mirra_canister::{canonical, core, types::{ExpertLoss, PublicSnapshot}};

#[derive(Deserialize)]
struct Vector {
    k: usize,
    priors: Vec<u64>,
    penalties: Vec<i128>,
    bound: i64,
    floor: u64,
    expected: Vec<u64>,
}

fn vectors() -> Vec<Vector> {
    serde_json::from_str(include_str!("vectors.json")).expect("vectors.json must parse")
}

#[test]
fn conformance_99_vectors_bit_exact() {
    let vectors = vectors();
    assert_eq!(vectors.len(), 99, "frozen vector count drifted");
    for (index, vector) in vectors.iter().enumerate() {
        assert_eq!(vector.priors.len(), vector.k, "vector {index}: priors");
        assert_eq!(
            vector.penalties.len(),
            vector.k,
            "vector {index}: penalties"
        );
        assert_eq!(vector.expected.len(), vector.k, "vector {index}: expected");
        let actual = core::weights_from_penalties(
            &vector.penalties,
            &vector.priors,
            vector.bound,
            vector.floor,
        )
        .unwrap_or_else(|error| panic!("vector {index}: {error}"));
        assert_eq!(actual, vector.expected, "vector {index} mismatch");
    }
}

#[test]
fn frozen_constants_and_domain_are_enforced() {
    let zeros_65 = [0; 65];
    assert!(core::fixed_share_normalize(&zeros_65, &zeros_65, core::FLOOR_UNITS).is_err());
    assert_eq!(core::exp_q32(0).unwrap(), core::SCALE as u64);
    assert_eq!(core::exp_q32(-core::B_Q32).unwrap(), 483);
    assert!(core::exp_q32(1).is_err());
    assert!(core::exp_q32(-core::B_Q32 - 1).is_err());
    assert!(core::validate_losses(&[0, core::SCALE as u64]).is_ok());
    assert!(core::validate_losses(&[0, core::SCALE as u64 + 1]).is_err());
}

#[test]
fn round_to_nearest_ties_to_even_edges() {
    assert_eq!(core::rne_div_u(1, 2), 0);
    assert_eq!(core::rne_div_u(3, 2), 2);
    assert_eq!(core::rne_div_i(-1, 2), 0);
    assert_eq!(core::rne_div_i(-3, 2), -2);
    assert_eq!(core::rne_div_i(i128::MIN, 1), i128::MIN);
    assert_eq!(core::rne_div_u(u128::MAX, 1), u128::MAX);
}

#[test]
fn exact_sum_and_floor_hold_for_every_vector() {
    for (index, vector) in vectors().iter().enumerate() {
        let actual = core::weights_from_penalties(
            &vector.penalties,
            &vector.priors,
            vector.bound,
            vector.floor,
        )
        .unwrap();
        assert_eq!(
            actual.iter().map(|value| *value as u128).sum::<u128>(),
            core::SCALE,
            "vector {index}: exact sum"
        );
        assert!(
            actual.iter().all(|value| *value >= vector.floor),
            "vector {index}: floor"
        );
    }
}

#[test]
fn canonical_identity_and_transaction_hash_known_answers() {
    let alpha = canonical::expert_id(&Principal::from_slice(&[10, 1]), "alpha", "v1.0.0");
    let beta = canonical::expert_id(&Principal::from_slice(&[11, 1]), "beta", "v1.0.0");
    assert_eq!(
        hex::encode(alpha),
        "645ec5a849ada43e5aa02c5a8451a28376feed2fca48881ee25eba775dbfc8c5"
    );
    assert_eq!(
        hex::encode(beta),
        "c046daaf2b0d325a80fdeb05b2ce33ddb1b3ea383990b081d1db4785c4ba9478"
    );
    let event = canonical::event_key(&[7; 32], &[1]);
    assert_eq!(
        hex::encode(event),
        "331c1b3ee418d64d8427b8535a9b1129b699f0210189a71a1a3c8eac6550cdc2"
    );
    let mut losses = vec![
        ExpertLoss {
            expert_id: alpha.to_vec(),
            loss_q32: 0,
        },
        ExpertLoss {
            expert_id: beta.to_vec(),
            loss_q32: 1 << 32,
        },
    ];
    losses.sort_by(|left, right| left.expert_id.cmp(&right.expert_id));
    let payload = canonical::payload_hash(&[7; 32], &losses);
    assert_eq!(
        hex::encode(payload),
        "86a29acf2e0909dcdacae0a851b2696324ca7e2ddb988e1cbe52fc542e091cab"
    );
    assert_eq!(
        hex::encode(canonical::correction_id(&event, &payload)),
        "f423d61a6fd1c2098358dc09e6b5015f0acb4840ec71c4d6fbe7147064713dff"
    );
}

#[test]
fn public_snapshot_commitment_is_canonical_and_sensitive() {
    let snapshot = PublicSnapshot {
        protocol: "mirra.v1-pilot".into(),
        protocol_fingerprint: vec![3; 32],
        schema_version: 1,
        seq: 7,
        expert_count: 2,
        state_root: vec![9; 32],
        vector_sha256: "21c64f9ea4aef2440f3a3929497551b4d289b49fee28753d4a1f4412bef01b1b".into(),
        p1_cert_sha256: "b9d1c0d5e0d8144d88f6e1375429fc844967731090d878ebd311072ee456a57b".into(),
    };
    let commitment = canonical::snapshot_commitment(&snapshot);
    assert_eq!(commitment.len(), 32);
    let mut changed = snapshot.clone();
    changed.seq += 1;
    assert_ne!(commitment, canonical::snapshot_commitment(&changed));
}
