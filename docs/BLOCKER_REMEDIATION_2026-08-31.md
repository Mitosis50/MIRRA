# Release-blocker remediation — RC3

## Outcome

Two blockers are operationally contained and automated; two require external
evidence or execution. **RC3 remains not release-ready.**

| Blocker | RC3 work | Status |
|---|---|---|
| Missing P1 certificate | Added fail-closed intake/hash verifier and protected release gate | External bytes still required |
| PocketIC socket denied | Added hash-pinned PocketIC 15.0.0 Ubuntu release job with the full upgrade test | Must pass upstream CI |
| No authoritative repository | Added protected-tag upstream merge and Sigstore/GitHub attestation procedure | Owner must merge and attest |
| Unmaintained transitive crates | Confirmed latest PocketIC/Candid chain, added exact expiring exception policy and Dependabot | Contained; not eliminated upstream |

## Security policy behavior

`verification/check_audit.py` fails when either lockfile has a known
vulnerability, when an informational advisory is added or removed without
review, when its crate version changes, or after 2026-11-30. This prevents
`cargo audit` informational warnings from silently accumulating.

Current reviewed set:

- production: `paste 1.0.15` (`RUSTSEC-2024-0436`), through Candid;
- integration only: `backoff 0.4.0`, `instant 0.1.13`, `paste 1.0.15`, and
  `serde_cbor 0.11.2`, through PocketIC/Candid.

There are zero known RustSec vulnerabilities at the recorded database
revision. Since the affected packages are transitive and their current
upstreams still resolve them, replacing them locally would require patching or
forking consensus/test dependencies. RC3 avoids that higher-risk divergence
and instead makes the exception set exact, expiring, and continuously updated.

## External closure conditions

1. Supply the genuine P1 certificate bytes matching `b9d1c0d5…57b` and the
   original proof checker/schema. The SHA verifier authenticates bytes but does
   not invent or replace the omitted proof checker.
2. Merge RC3 into the authoritative repository and configure protected branch
   and tag policies.
3. Run `.github/workflows/release-attestation.yml` on an Ubuntu runner. It must
   report the real non-writer rejection, counts `1 -> 1 -> 2`, root and weight
   equality after upgrade, and unique sequences 1 and 2.
4. Verify the GitHub artifact attestation and publish its workflow URL with the
   exact upstream commit and Wasm digest.
