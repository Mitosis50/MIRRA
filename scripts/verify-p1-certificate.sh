#!/usr/bin/env bash
set -euo pipefail

project_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
certificate="${1:-${project_root}/verification/p1/MIRRA_P1_CERTIFICATE.bin}"
expected_file="${project_root}/verification/p1/CERTIFICATE.sha256"
expected="$(cut -d ' ' -f 1 "${expected_file}")"

# The original bytes were recovered, but their argument enclosure is invalid.
# A matching digest proves byte identity; it cannot rehabilitate this proof.
# See docs/P1_PROOF_WITHDRAWAL_2026-09-03.md before changing this policy.
withdrawn="b9d1c0d5e0d8144d88f6e1375429fc844967731090d878ebd311072ee456a57b"
if [[ "${expected}" == "${withdrawn}" ]]; then
  echo "FAIL: the pinned P1 proof is withdrawn: invalid half-sized argument enclosure." >&2
  echo "Recovered bytes or a matching hash cannot close this gate." >&2
  echo "A reviewed replacement and a newly bound candidate are required." >&2
  exit 1
fi

if [[ ! -s "${certificate}" ]]; then
  echo "FAIL: required P1 certificate is missing or empty: ${certificate}" >&2
  exit 1
fi
actual="$(sha256sum "${certificate}" | cut -d ' ' -f 1)"
if [[ "${actual}" == "${withdrawn}" ]]; then
  echo "FAIL: supplied P1 proof is withdrawn: invalid argument enclosure." >&2
  exit 1
fi
if [[ "${actual}" != "${expected}" ]]; then
  echo "FAIL: P1 certificate SHA-256 mismatch" >&2
  echo "expected: ${expected}" >&2
  echo "actual:   ${actual}" >&2
  exit 1
fi
echo "PASS: P1 certificate bytes match ${expected}"
