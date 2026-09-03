# Upstream merge and release attestation

The local reconstructed history is evidence of this candidate's contents, not
evidence of the original project's authorship. The authoritative repository
owner closes that gap with the following controlled merge.

## Required upstream procedure

1. Import or cherry-pick the candidate and review every source change against
   the audited handoff. Do not preserve the local candidate commit as proof of
   upstream authorship.
2. Independently review and reissue the P1 evidence. The original proof digest
   `b9d1c0d5e0d8144d88f6e1375429fc844967731090d878ebd311072ee456a57b`
   is withdrawn because its argument enclosure is invalid. Restoring those bytes
   cannot close the gate. Review the corrected derivation, source/constants,
   checker and provenance; issue replacement evidence; then bind its actual
   digest in the compiled pin and intake record. Rebuild as a new candidate and
   rerun all artifact-specific gates. Keep the withdrawn digest blocked.
   See [the withdrawal record](P1_PROOF_WITHDRAWAL_2026-09-03.md).
3. Protect the default branch: require pull-request review, the `MIRRA
   verification` workflow, and dismissal of approvals after new commits.
4. Run `MIRRA release attestation` on a protected `mirra-v*` tag. Its Ubuntu
   runner must pass the PocketIC authorization/upgrade test and every other
   release gate before it can attest the Wasm and CycloneDX SBOM.
5. Verify the resulting provenance from a clean checkout:

   ```bash
   gh attestation verify release/mirra_canister.wasm -R OWNER/REPOSITORY
   ```

6. Publish the tag, upstream commit, Wasm SHA-256, vector SHA-256, P1
   certificate SHA-256, SBOM, and verification workflow URL together.

Any failure or skipped job blocks the release. A local Git bundle, maintainer
signature, or native stable-memory test alone does not replace the protected
upstream workflow and PocketIC upgrade evidence.
