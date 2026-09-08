# P1 proof withdrawal and replacement requirements

Staging update: the complete corrected derivation, premise census and candidate
proof now appear in [the review packet](../verification/p1/README.md). This
branch increments both pins to those candidate bytes while retaining a pending
independent-review gate. The original withdrawal account below describes the
baseline repair; it remains historical evidence.

The recovered original proof JSON has SHA-256
`b9d1c0d5e0d8144d88f6e1375429fc844967731090d878ebd311072ee456a57b`.
That digest proves which bytes were recovered. The bytes contain an invalid
half-sized series-argument enclosure and must not satisfy release approval.

At `u=16373744595`, the frozen kernel reaches argument magnitude
`0.346573590278192344982244321727193892002105712890625`, exceeding the old
proof's approximately `0.17328679` enclosure. The kernel returns `94906266`,
which agrees with the rounded reference at that witness. This does not repair
the domain-wide proof.

The September 3 corrected derivation produces the conservative bound
`0.5000000425665135563095684854886…` output ulp, with outward ceiling
`0.500000043`. It remains a derived review candidate. Its full derivation,
exact fractions and calculator are retained in
`MIRRA_PROJECT_REVIEW_PACKAGE_2026-09-03.zip` under the corrected-budget filenames.
The package is provided alongside this handoff, not incorporated into the
consensus source or represented as an approved replacement certificate.

`scripts/verify-p1-certificate.sh` now fails when the configured pin is withdrawn,
even if no file was supplied. It also rejects the withdrawn bytes if supplied
under a later pin. The unchanged RC5 pin therefore keeps release blocked.
Changing a pin alone establishes no mathematical or maintainer authority.

To close the gate:

1. Have an identified independent reviewer accept or correct the derivation,
   integer bounds, source mapping, downstream assumptions and provenance.
2. Issue replacement proof bytes, a documented schema/checker and the review
   decision. Preserve the original artifact as withdrawn historical evidence.
3. Bind the replacement proof's actual hash to `P1_CERT_SHA256` and
   `verification/p1/CERTIFICATE.sha256`; measure a new Wasm artifact.
4. Rerun conformance, Candid, reproducibility, upgrade and certificate checks;
   attest that exact subject in the authoritative repository.

This patch changes release controls and tests. It does not issue a certificate,
alter the arithmetic, change the compiled proof pin, or approve RC5 for use.
