# Historical RC5 Wasm

`mirra_canister.wasm` in this directory is the unchanged RC5 baseline with
SHA-256 `acf963dadeb9eeb4c7ee5e6065045a1dcdc93af0700ce47868b5b2aa7485dfa2`.
It embeds the withdrawn proof pin. It is not the staged replacement build.

The staged P1 branch compiles a fresh Wasm and publishes it as a GitHub Actions
candidate artifact with `BUILD_RECORD.json` and `SHA256SUMS`. Release packaging
must replace this historical binary and hash with the reviewed, measured subject;
`verify-release.sh` still enforces packaged-versus-built equality.
