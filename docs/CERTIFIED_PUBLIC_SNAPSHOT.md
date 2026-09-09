# Certified public snapshot

MIRRA exposes a compact public snapshot and commits its canonical SHA-256 hash
to ICP certified data. This makes the consensus-visible protocol identity and
state independently checkable without trusting a MIRRA-operated API.

The commitment covers the protocol name and fingerprint, stable schema,
correction sequence, expert count, correction state root, frozen 99-vector
corpus hash, and declared P1 certificate hash. It uses domain
`mirra.snapshot.v1\0`, big-endian integers, and four-byte big-endian length
prefixes for variable fields.

`public_snapshot` returns the data. `certified_snapshot` additionally returns
the commitment and ICP data certificate. A relying party must:

1. verify the ICP certificate against the subnet root of trust and canister ID;
2. confirm its certified-data value equals `snapshot_commitment`;
3. recompute the commitment from `snapshot` using the canonical encoding; and
4. apply its own allowlist for the protocol fingerprint and artifact hashes.

`verification/verify_snapshot.py` performs step 3 from a JSON representation.
It intentionally does not claim to verify the ICP certificate; that requires
the canister ID and a trusted ICP certificate-verification implementation.

`verification/mainnet-verifier` performs all four steps for the pinned initial
mainnet deployment. It also checks the installed module hash, exact controller
set, governance principal, and initial-state invariants. It uses `ic-agent`'s
hard-coded IC mainnet root key and deliberately never fetches a replacement
root key from the network.

This primitive is deliberately small. It is useful to wallets, agents,
indexers, DAOs, and research pipelines because they can pin one protocol
fingerprint and detect silent arithmetic, artifact, or state substitutions.
