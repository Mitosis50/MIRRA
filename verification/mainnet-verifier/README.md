# MIRRA mainnet trusted-root verifier

This is a zero-argument, read-only verifier for the initial MIRRA mainnet
deployment. It fails closed unless all approved identities and artifact pins
match.

It checks:

1. the `certified_snapshot` certificate's BLS signature, delegation,
   canister range, canister ID, and timestamp using `ic-agent`'s hard-coded IC
   mainnet root key;
2. certificate `certified_data == snapshot_commitment`;
3. MIRRA's canonical snapshot commitment recomputation;
4. the protocol, schema, protocol fingerprint, vector hash, P1 proof hash, and
   initial empty state;
5. the installed module hash through certified `read_state`;
6. the exact deployer + recovery controller set through certified
   `read_state`; and
7. governance and health invariants, cross-checking state-bearing health fields
   against the certified snapshot.

The verifier intentionally contains no `fetch_root_key()` call. Do not add one
for mainnet: that would replace the independent trust anchor with a key fetched
from the network being checked.

Run from this directory:

```sh
cargo run --locked --release
```

Success prints `MIRRA_MAINNET_TRUSTED_ROOT_OK`. Any mismatch exits non-zero.

Normal public-network use needs no environment variables. A controlled CI
environment that terminates TLS with its own CA may set
`MIRRA_EXTRA_TLS_CA_PEM` to that CA's PEM file. This affects only HTTPS
transport: the ICP certificate must still verify against `ic-agent`'s
hard-coded mainnet root.

Run and preserve this check **before** expert enrollment or the mainnet witness
sequence. This version deliberately requires the untouched initial state
(`seq = 0`, zero experts, and the zero state root), so later authorized writes
cannot be mistaken for the independently verified launch baseline.

The controller evidence step records the already verified controller set; it
does not require or authorize another controller mutation. Expert enrollment
and every write in the mainnet witness sequence require their own explicit,
bounded authorizations and are outside this verifier's scope.

This checker is bound to:

- canister `3rtnf-eyaaa-aaaal-qxina-cai`
- source commit `b8c56a1744b53aa9dae656bee17bc09fc717733e`
- source tree `1d7bf27932859b42e2fad1736ea1bf47490d7e80`
- module SHA-256 `2edf7aaba7e1e4eaf781349dc99e3425257d6f777b4d333c509c9538d2ac5aba`
- P1 proof SHA-256 `37f218d3fb9695dbc62cd60955337067b6e55fa45a4d8c0a93ef8415278a3a2d`
- 99-vector SHA-256 `21c64f9ea4aef2440f3a3929497551b4d289b49fee28753d4a1f4412bef01b1b`
- `ic-agent = 0.45.0`
- `ic-certification = 3.2.0`

The source commit/tree line is a release-evidence binding. The on-chain
certificate directly authenticates the canister ID, certified snapshot, and
installed module hash; source correspondence still depends on the separately
reviewed reproducible-build evidence.
