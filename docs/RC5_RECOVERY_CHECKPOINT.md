# RC5 recovery checkpoint — 2026-09-01

Final recovery update: the post-format Wasm and integration lockfile have been
recovered exactly. Native tests, both Clippy checks, Candid equality, three
clean-path Wasm builds, dfx, fresh audits, and normalized SBOM reproduction
passed. The expanded certificate-root harness compiles but remains runtime
unverified. See `VERIFICATION_2026-09-01.md` for the current gate status.
The following paragraphs preserve the initial recovery record; their
missing-binary statements are historical, not current artifact status.

This is an incomplete, reconstructed source checkpoint, not a release.
The previous transient RC5 checkout was cleared before durable packaging.
The base Git bundle was restored and its SHA-256 verified as
`ba9d55a11d9f7a1e0d3d41256fe75d37a580e1b9be961b285492076657f2c359`.

Recorded changes are being reconstructed as separate commits: whitespace-only
formatting, formatter policy, Candid regeneration, SBOM path normalization,
and certificate-root integration verification. The integration dependency
lockfile still requires regeneration at this checkpoint.

The previous run recorded a post-format Wasm SHA-256 of
`acf963dadeb9eeb4c7ee5e6065045a1dcdc93af0700ce47868b5b2aa7485dfa2`.
Those binary bytes did not survive; this is a recorded target, not a claim
that this checkpoint contains that artifact. The packaged `9171...` binary is
historical RC3 output and must not be attested as the current source build.

The previous run passed Rust 1.88.0 native tests (including the pinned 99
vectors), Candid equality, path-independent Wasm comparisons, and dependency
policy/SBOM checks. These are historical run observations until repeated on
the reconstructed source. PocketIC failed to initialize because this host
denied Unix-socket creation. The genuine P1 certificate and upstream owner
review/signature remain external requirements.
