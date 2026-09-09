# MIRRA initial mainnet trusted-root verification

Date: 2026-09-08 PT / 2026-09-09 UTC

## Scope

This is read-only evidence for the initial, empty MIRRA mainnet state. It is
not a release attestation, merge approval, expert enrollment, correction,
upgrade, or authorization for any write.

## Checker binding

| Item | Exact value |
|---|---|
| Checker source SHA-256 | `5d9f7b412b63f775b5de3a72b5cfa1251dac29f9ea7f7ab3696f6b1a33f11b7b` |
| Checker manifest SHA-256 | `437fb7ace06349cf1714b7d2ceb3cab3a771c78a9c4147d872723a31d53aafd1` |
| Checker lockfile SHA-256 | `508f824e35ebec0c1dc15a5f65611bfdb7a9d9f1a04b571a9a576c65491a52ea` |
| Rust toolchain | `1.88.0` |
| `ic-agent` | `0.45.0` |
| `ic-certification` | `3.2.0` |

The checker is `verification/mainnet-verifier`. It uses `ic-agent`'s
hard-coded IC mainnet root key and contains no call that fetches or replaces
that trust anchor.

## Pinned deployment

| Item | Exact value |
|---|---|
| Canister | `3rtnf-eyaaa-aaaal-qxina-cai` |
| Source commit | `b8c56a1744b53aa9dae656bee17bc09fc717733e` |
| Source tree | `1d7bf27932859b42e2fad1736ea1bf47490d7e80` |
| Installed module SHA-256 | `2edf7aaba7e1e4eaf781349dc99e3425257d6f777b4d333c509c9538d2ac5aba` |
| P1 proof SHA-256 | `37f218d3fb9695dbc62cd60955337067b6e55fa45a4d8c0a93ef8415278a3a2d` |
| 99-vector SHA-256 | `21c64f9ea4aef2440f3a3929497551b4d289b49fee28753d4a1f4412bef01b1b` |
| Protocol fingerprint | `13ec68e1cae68c62b69124b4d06ecc7daa64963f62fe20c2bdb45faecc9f3168` |

## Result

`cargo test --locked` passed all three fail-closed tests.

`cargo clippy --locked --all-targets -- -D warnings` passed.

The anonymous, read-only live run returned:

```text
MIRRA_MAINNET_TRUSTED_ROOT_OK
canister=3rtnf-eyaaa-aaaal-qxina-cai
module_sha256=2edf7aaba7e1e4eaf781349dc99e3425257d6f777b4d333c509c9538d2ac5aba
snapshot_commitment=1bc5c63c8393d52f8737a7d36134a99e735b8b75ba0a1c61e111872ca77f037a
protocol_fingerprint=13ec68e1cae68c62b69124b4d06ecc7daa64963f62fe20c2bdb45faecc9f3168
source_commit=b8c56a1744b53aa9dae656bee17bc09fc717733e
source_tree=1d7bf27932859b42e2fad1736ea1bf47490d7e80
state_seq=0
expert_count=0
controllers=zquv5-mqxnx-n5r4l-aimqg-tbvrl-ksxyw-u6dzf-b6frz-5tbdj-rc2xr-qqe,ogeck-zn67c-irmo3-6x3fx-6vpbx-ylpuo-x2ugb-xsegf-7jscb-spdyj-lae
trust_anchor=ic-agent-0.45.0-hard-coded-ic-mainnet-root
```

## Decision boundary

This closes the initial mainnet trusted-root snapshot check for the checker
and pins above. It does not independently prove source-to-Wasm correspondence;
that remains bound to the separate reproducible-build evidence. It also does
not convert passing verification into formal approval or independent P1
acceptance.

The subsequent gates remain separate:

1. record the already verified controller state in the mainnet evidence packet;
2. authorize exact expert enrollments individually (at least two active experts
   are required before MIRRA accepts a correction);
3. authorize each step of the mainnet witness sequence separately; and
4. obtain independent review bound to the exact checker commit and evidence.
