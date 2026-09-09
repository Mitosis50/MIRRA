use candid::{CandidType, Decode, Encode, Principal};
use ic_agent::{lookup_value, Agent, Certificate};
use serde::Deserialize;
use sha2::{Digest, Sha256};
use std::{env, error::Error, fs, io};

const IC_GATEWAY: &str = "https://icp-api.io";
const CANISTER_TEXT: &str = "3rtnf-eyaaa-aaaal-qxina-cai";
const DEPLOYER_TEXT: &str = "zquv5-mqxnx-n5r4l-aimqg-tbvrl-ksxyw-u6dzf-b6frz-5tbdj-rc2xr-qqe";
const RECOVERY_TEXT: &str = "ogeck-zn67c-irmo3-6x3fx-6vpbx-ylpuo-x2ugb-xsegf-7jscb-spdyj-lae";
const SOURCE_COMMIT: &str = "b8c56a1744b53aa9dae656bee17bc09fc717733e";
const SOURCE_TREE: &str = "1d7bf27932859b42e2fad1736ea1bf47490d7e80";
const MODULE_SHA256: &str = "2edf7aaba7e1e4eaf781349dc99e3425257d6f777b4d333c509c9538d2ac5aba";
const PROTOCOL: &str = "mirra.v1-pilot";
const SCHEMA_VERSION: u32 = 1;
const PROTOCOL_FINGERPRINT: &str =
    "13ec68e1cae68c62b69124b4d06ecc7daa64963f62fe20c2bdb45faecc9f3168";
const VECTOR_SHA256: &str = "21c64f9ea4aef2440f3a3929497551b4d289b49fee28753d4a1f4412bef01b1b";
const P1_CERT_SHA256: &str = "37f218d3fb9695dbc62cd60955337067b6e55fa45a4d8c0a93ef8415278a3a2d";

type AnyResult<T> = Result<T, Box<dyn Error>>;

#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq)]
struct PublicSnapshot {
    protocol: String,
    protocol_fingerprint: Vec<u8>,
    schema_version: u32,
    seq: u64,
    expert_count: u64,
    state_root: Vec<u8>,
    vector_sha256: String,
    p1_cert_sha256: String,
}

#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq)]
struct CertifiedSnapshot {
    snapshot: PublicSnapshot,
    snapshot_commitment: Vec<u8>,
    certificate: Option<Vec<u8>>,
}

#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq)]
struct Health {
    schema_version: u32,
    seq: u64,
    expert_count: u64,
    active_expert_count: u64,
    correction_count: u64,
    receipt_count: u64,
    writer_count: u64,
    governance_event_count: u64,
    paused: bool,
    active_set_locked: bool,
    governance: Principal,
    state_root: Vec<u8>,
}

fn fail(message: impl Into<String>) -> Box<dyn Error> {
    Box::new(io::Error::other(message.into()))
}

fn require(condition: bool, message: impl Into<String>) -> AnyResult<()> {
    if condition {
        Ok(())
    } else {
        Err(fail(message))
    }
}

fn add_len_prefixed(hasher: &mut Sha256, bytes: &[u8]) -> AnyResult<()> {
    let length =
        u32::try_from(bytes.len()).map_err(|_| fail("canonical field exceeds u32::MAX"))?;
    hasher.update(length.to_be_bytes());
    hasher.update(bytes);
    Ok(())
}

fn snapshot_commitment(snapshot: &PublicSnapshot) -> AnyResult<[u8; 32]> {
    let mut hasher = Sha256::new();
    hasher.update(b"mirra.snapshot.v1\0");
    add_len_prefixed(&mut hasher, snapshot.protocol.as_bytes())?;
    add_len_prefixed(&mut hasher, &snapshot.protocol_fingerprint)?;
    hasher.update(snapshot.schema_version.to_be_bytes());
    hasher.update(snapshot.seq.to_be_bytes());
    hasher.update(snapshot.expert_count.to_be_bytes());
    add_len_prefixed(&mut hasher, &snapshot.state_root)?;
    add_len_prefixed(&mut hasher, snapshot.vector_sha256.as_bytes())?;
    add_len_prefixed(&mut hasher, snapshot.p1_cert_sha256.as_bytes())?;
    Ok(hasher.finalize().into())
}

async fn query<T>(agent: &Agent, canister_id: Principal, method: &str) -> AnyResult<T>
where
    T: for<'de> Deserialize<'de> + CandidType,
{
    let response = agent
        .query(&canister_id, method)
        .with_arg(Encode!()?)
        .call()
        .await?;
    Ok(Decode!(&response, T)?)
}

fn verify_snapshot_allowlist(snapshot: &PublicSnapshot) -> AnyResult<()> {
    require(snapshot.protocol == PROTOCOL, "unexpected protocol")?;
    require(
        snapshot.schema_version == SCHEMA_VERSION,
        "unexpected schema version",
    )?;
    require(
        hex::encode(&snapshot.protocol_fingerprint) == PROTOCOL_FINGERPRINT,
        "unexpected protocol fingerprint",
    )?;
    require(
        snapshot.vector_sha256 == VECTOR_SHA256,
        "unexpected 99-vector hash",
    )?;
    require(
        snapshot.p1_cert_sha256 == P1_CERT_SHA256,
        "unexpected P1 certificate hash",
    )?;
    require(snapshot.seq == 0, "initial-state gate requires seq = 0")?;
    require(
        snapshot.expert_count == 0,
        "initial-state gate requires expert_count = 0",
    )?;
    require(
        snapshot.state_root == vec![0; 32],
        "initial-state gate requires the zero state root",
    )?;
    Ok(())
}

fn verify_health(health: &Health, snapshot: &PublicSnapshot, deployer: Principal) -> AnyResult<()> {
    require(
        health.governance == deployer,
        "unexpected governance principal",
    )?;
    require(
        health.schema_version == SCHEMA_VERSION,
        "health schema mismatch",
    )?;
    require(
        health.seq == snapshot.seq,
        "health/snapshot sequence mismatch",
    )?;
    require(
        health.expert_count == snapshot.expert_count,
        "health/snapshot expert count mismatch",
    )?;
    require(
        health.state_root == snapshot.state_root,
        "health/snapshot state root mismatch",
    )?;
    require(!health.paused, "canister is paused")?;
    require(
        !health.active_set_locked,
        "active set is unexpectedly locked",
    )?;
    require(health.active_expert_count == 0, "unexpected active experts")?;
    require(health.correction_count == 0, "unexpected corrections")?;
    require(health.receipt_count == 0, "unexpected receipts")?;
    require(health.writer_count == 1, "unexpected writer count")?;
    require(
        health.governance_event_count == 1,
        "unexpected governance event count",
    )?;
    Ok(())
}

fn http_client() -> AnyResult<reqwest::Client> {
    let mut builder = reqwest::Client::builder();
    if let Ok(path) = env::var("MIRRA_EXTRA_TLS_CA_PEM") {
        require(!path.is_empty(), "MIRRA_EXTRA_TLS_CA_PEM is empty")?;
        let pem = fs::read(path)?;
        builder = builder.add_root_certificate(reqwest::Certificate::from_pem(&pem)?);
    }
    Ok(builder.build()?)
}

#[tokio::main]
async fn main() -> AnyResult<()> {
    let canister_id = Principal::from_text(CANISTER_TEXT)?;
    let deployer = Principal::from_text(DEPLOYER_TEXT)?;
    let recovery = Principal::from_text(RECOVERY_TEXT)?;

    // Agent 0.45.0 starts with DFINITY's hard-coded IC mainnet root key.
    // Deliberately never call fetch_root_key(): trusting a key supplied by the
    // network would defeat this verifier's independent trust anchor.
    let agent = Agent::builder()
        .with_url(IC_GATEWAY)
        .with_http_client(http_client()?)
        .build()?;

    let certified: CertifiedSnapshot = query(&agent, canister_id, "certified_snapshot").await?;
    let certificate_bytes = certified
        .certificate
        .as_deref()
        .ok_or_else(|| fail("certified_snapshot returned no ICP certificate"))?;
    let certificate: Certificate = serde_cbor::from_slice(certificate_bytes)?;

    // This verifies the BLS signature, delegation/canister ranges, canister ID,
    // and certificate freshness against the library's hard-coded mainnet root.
    agent.verify(&certificate, canister_id)?;

    let certified_data = lookup_value(
        &certificate,
        [
            b"canister".as_slice(),
            canister_id.as_slice(),
            b"certified_data".as_slice(),
        ],
    )?;
    require(
        certified_data == certified.snapshot_commitment,
        "certificate certified_data does not equal snapshot_commitment",
    )?;

    let recomputed = snapshot_commitment(&certified.snapshot)?;
    require(
        recomputed.as_slice() == certified.snapshot_commitment,
        "canonical snapshot commitment mismatch",
    )?;
    verify_snapshot_allowlist(&certified.snapshot)?;

    // read_state helpers independently verify their response certificates.
    let module_hash = agent.read_state_canister_module_hash(canister_id).await?;
    require(
        hex::encode(&module_hash) == MODULE_SHA256,
        "installed module hash is not the approved Wasm hash",
    )?;

    let mut controllers = agent.read_state_canister_controllers(canister_id).await?;
    controllers.sort_by(|left, right| left.as_slice().cmp(right.as_slice()));
    let mut expected_controllers = vec![deployer, recovery];
    expected_controllers.sort_by(|left, right| left.as_slice().cmp(right.as_slice()));
    require(
        controllers == expected_controllers,
        "controller set differs from deployer + recovery",
    )?;

    // `health` is a signed query response, not a substitute for certified_data.
    // Its state-bearing fields are cross-checked against the certified snapshot.
    let health: Health = query(&agent, canister_id, "health").await?;
    verify_health(&health, &certified.snapshot, deployer)?;

    println!("MIRRA_MAINNET_TRUSTED_ROOT_OK");
    println!("canister={CANISTER_TEXT}");
    println!("module_sha256={MODULE_SHA256}");
    println!("snapshot_commitment={}", hex::encode(recomputed));
    println!("protocol_fingerprint={PROTOCOL_FINGERPRINT}");
    println!("source_commit={SOURCE_COMMIT}");
    println!("source_tree={SOURCE_TREE}");
    println!("state_seq={}", certified.snapshot.seq);
    println!("expert_count={}", certified.snapshot.expert_count);
    println!("controllers={DEPLOYER_TEXT},{RECOVERY_TEXT}");
    println!("trust_anchor=ic-agent-0.45.0-hard-coded-ic-mainnet-root");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn approved_snapshot() -> PublicSnapshot {
        PublicSnapshot {
            protocol: PROTOCOL.into(),
            protocol_fingerprint: hex::decode(PROTOCOL_FINGERPRINT).unwrap(),
            schema_version: SCHEMA_VERSION,
            seq: 0,
            expert_count: 0,
            state_root: vec![0; 32],
            vector_sha256: VECTOR_SHA256.into(),
            p1_cert_sha256: P1_CERT_SHA256.into(),
        }
    }

    #[test]
    fn allowlisted_snapshot_passes() {
        verify_snapshot_allowlist(&approved_snapshot()).unwrap();
    }

    #[test]
    fn changed_protocol_fails_closed() {
        let mut snapshot = approved_snapshot();
        snapshot.protocol.push_str("-changed");
        assert!(verify_snapshot_allowlist(&snapshot).is_err());
    }

    #[test]
    fn changed_state_fails_initial_gate() {
        let mut snapshot = approved_snapshot();
        snapshot.seq = 1;
        assert!(verify_snapshot_allowlist(&snapshot).is_err());
    }
}
