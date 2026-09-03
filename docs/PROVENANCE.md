# Build and release provenance

MIRRA releases use content-addressed evidence. A release statement must bind
all of the following to one named Git commit:

- source tree and `SOURCE_MANIFEST.sha256`;
- Rust 1.88.0 toolchain and LLVM details;
- main and PocketIC integration `Cargo.lock` files;
- 99-vector corpus and actual P1 certificate;
- production and integration RustSec results;
- CycloneDX SBOMs;
- generated Candid interface;
- byte-identical clean Wasm builds;
- PocketIC authorization and upgrade log;
- final Wasm SHA-256.

## Current artifact — post-format certified-snapshot build

The current artifact is `artifacts/mirra_canister.wasm`, rebuilt using the
pinned Rust 1.88.0 toolchain. This is the only current Wasm digest. The
`9171f7e7...dd53` digest in the dated RC2/RC3 reports is historical, not an
attestation subject for the current source.

| Artifact | SHA-256 |
|---|---|
| 99 vectors | `21c64f9ea4aef2440f3a3929497551b4d289b49fee28753d4a1f4412bef01b1b` |
| Wasm (878,189 bytes) | `acf963dadeb9eeb4c7ee5e6065045a1dcdc93af0700ce47868b5b2aa7485dfa2` |
| Main Cargo.lock | `76a47e3609d60ee06d8993366eae6dcf546bfb019eae2a8464f6bf4e89453d73` |
| Integration Cargo.lock | `643c2773a806be3bb4c464ccbd6bfad1b52690965d71b67d6873b2b94be15f78` |
| Candid | `b9f031d8dc354d93935b1bc29001b875a3616cc55a7dfc95ec3ccae6250765f8` |
| Production audit JSON | `3b3224edb13e2be1ccad0307d6bc88bb2ab725abf494c9e1721730afb58f973e` |
| Integration audit JSON | `f5ca74f546693a9cf728e1ef08c7204c0415e8917841991f0d3b92418eb90a6e` |
| Production SBOM | `6dadf58bfda8891fa167bd5ca1af83c22d8bdb235e68b802718eac49a4ff027b` |
| Integration SBOM | `4616160906d6f9772b2ab65ee4eb1708b8cfabffb28ac86546695f3127d97357` |

See `VERIFICATION_2026-09-01.md` and its JSON companion for the completed local
verification pass and exact remaining gates. Both audit JSON files bind to
RustSec commit `72f8b23d78ea6c4c9ded301a4c6ec4260e8b4c27` (1,235 advisories).
Both SBOMs reproduce, including after normalization from another source path.
This remains a release candidate, not a release declaration.

The current archive originated from an audited Markdown handoff rather than an
authoritative repository. A local Git snapshot can make subsequent changes
traceable, but it does not establish upstream authorship, history, or deployment
provenance. The authoritative owner must merge and sign or otherwise attest the
final release commit.

The canonical Wasm builder remaps absolute project, Cargo registry, and Rustup
paths to stable virtual prefixes. Reproducibility testing copies the source to
two different directory depths and compares both builds to the release Wasm.
