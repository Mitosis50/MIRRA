use candid::Principal;
use sha2::{Digest, Sha256};

use crate::types::{ExpertLoss, PublicSnapshot};

fn add_len_prefixed(hasher: &mut Sha256, bytes: &[u8]) {
    let len = u32::try_from(bytes.len()).expect("canonical field exceeds u32::MAX");
    hasher.update(len.to_be_bytes());
    hasher.update(bytes);
}

fn finish(hasher: Sha256) -> [u8; 32] {
    let digest = hasher.finalize();
    let mut output = [0; 32];
    output.copy_from_slice(&digest);
    output
}

pub fn expert_id(owner: &Principal, system_id: &str, version: &str) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(b"mirra.expert.v1\0");
    add_len_prefixed(&mut hasher, owner.as_slice());
    add_len_prefixed(&mut hasher, system_id.as_bytes());
    add_len_prefixed(&mut hasher, version.as_bytes());
    finish(hasher)
}

pub fn event_key(metric_id: &[u8], external_event_id: &[u8]) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(b"mirra.event.v1\0");
    add_len_prefixed(&mut hasher, metric_id);
    add_len_prefixed(&mut hasher, external_event_id);
    finish(hasher)
}

pub fn payload_hash(metric_id: &[u8], sorted_losses: &[ExpertLoss]) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(b"mirra.payload.v1\0");
    add_len_prefixed(&mut hasher, metric_id);
    hasher.update((sorted_losses.len() as u32).to_be_bytes());
    for loss in sorted_losses {
        add_len_prefixed(&mut hasher, &loss.expert_id);
        hasher.update(loss.loss_q32.to_le_bytes());
    }
    finish(hasher)
}

pub fn correction_id(event_key: &[u8; 32], payload_hash: &[u8; 32]) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(b"mirra.correction.v1\0");
    hasher.update(event_key);
    hasher.update(payload_hash);
    finish(hasher)
}

pub fn next_state_root(
    previous_root: &[u8],
    seq: u64,
    correction_id: &[u8; 32],
    payload_hash: &[u8; 32],
    expert_ids: &[[u8; 32]],
    penalties: &[i128],
    weights: &[u64],
) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(b"mirra.state.v1\0");
    add_len_prefixed(&mut hasher, previous_root);
    hasher.update(seq.to_le_bytes());
    hasher.update(correction_id);
    hasher.update(payload_hash);
    for ((expert_id, penalty), weight) in expert_ids.iter().zip(penalties).zip(weights) {
        hasher.update(expert_id);
        hasher.update(penalty.to_be_bytes());
        hasher.update(weight.to_le_bytes());
    }
    finish(hasher)
}

pub fn protocol_fingerprint(
    schema_version: u32,
    vector_sha256: &str,
    p1_cert_sha256: &str,
) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(b"mirra.protocol.v1\0");
    hasher.update(schema_version.to_be_bytes());
    add_len_prefixed(&mut hasher, vector_sha256.as_bytes());
    add_len_prefixed(&mut hasher, p1_cert_sha256.as_bytes());
    finish(hasher)
}

pub fn snapshot_commitment(snapshot: &PublicSnapshot) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(b"mirra.snapshot.v1\0");
    add_len_prefixed(&mut hasher, snapshot.protocol.as_bytes());
    add_len_prefixed(&mut hasher, &snapshot.protocol_fingerprint);
    hasher.update(snapshot.schema_version.to_be_bytes());
    hasher.update(snapshot.seq.to_be_bytes());
    hasher.update(snapshot.expert_count.to_be_bytes());
    add_len_prefixed(&mut hasher, &snapshot.state_root);
    add_len_prefixed(&mut hasher, snapshot.vector_sha256.as_bytes());
    add_len_prefixed(&mut hasher, snapshot.p1_cert_sha256.as_bytes());
    finish(hasher)
}
