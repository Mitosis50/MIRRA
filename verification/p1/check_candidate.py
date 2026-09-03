#!/usr/bin/env python3
"""Recompute staged P1 proof bytes and verify their source/pin bindings.

--emit prints deterministic JSON bytes. --check validates a staged candidate.
--release additionally requires a separately recorded review decision. A local
review JSON is a gate input, not authentication of a reviewer or an attestation.
"""

import argparse
import hashlib
import json
import re
from pathlib import Path

from derive_candidate import compute, require

BASELINE = "4ad47db0671cad3a151b4b77d6f2248f5ed84bdc"
PIN_PATTERN = r'(pub const P1_CERT_SHA256: &str = ")([0-9a-f]{64})(";)'
BOUND = "0.500000043"


def digest(value):
    return hashlib.sha256(value).hexdigest()


def canonical(value):
    return (json.dumps(value, sort_keys=True, indent=2, ensure_ascii=True) + "\n").encode()


def candidate(root):
    result = compute(root)
    require(result["new_derivation"]["ceiling_9dp"] == BOUND, "surviving bound changed")
    bindings = {}
    for path in (
        "src/core.rs", "src/canonical.rs", "src/stable.rs", "src/types.rs",
        "verification/p1/exp_q32_production.py",
        "verification/p1/derive_candidate.py", "verification/p1/check_candidate.py",
        "verification/p1/PREMISE_CENSUS.md",
        "verification/p1/review/mirra_p1_corrected_budget.py",
        "verification/p1/review/MIRRA_P1_CORRECTED_BUDGET_2026-09-03.md",
        "verification/p1/review/MIRRA_P1_CORRECTED_BUDGET_2026-09-03.json",
        "verification/p1/review/WITHDRAWN_p1_proof_artifact.json",
    ):
        bindings[path] = {"normalization": "none", "sha256": digest((root / path).read_bytes())}
    lib = (root / "src/lib.rs").read_text()
    require(len(re.findall(PIN_PATTERN, lib)) == 1, "expected exactly one compiled P1 pin")
    normalized = re.sub(PIN_PATTERN, lambda m: m[1] + "0" * 64 + m[3], lib)
    bindings["src/lib.rs"] = {
        "normalization": "replace only the 64 hexadecimal digits of P1_CERT_SHA256 with zeros",
        "sha256": digest(normalized.encode()),
    }
    return {
        "schema": "mirra.p1.proof-candidate.v2",
        "status": "CANDIDATE_PENDING_INDEPENDENT_REVIEW",
        "baseline_main_commit": BASELINE,
        "supersedes_withdrawn_proof_sha256": result["inputs"]["old_proof_sha256"],
        "claim": {
            "absolute_error_output_ulps_le": BOUND,
            "domain": "all integer x_q32 in [-16*2^32, 0]",
            "reference": "2^32 * exp(x_q32 / 2^32)",
            "universal_correct_rounding_claimed": False,
            "downstream_total_variation_theorem_claimed": False,
        },
        "source_bindings": bindings,
        "derivation": result,
    }


def check(root, release=False):
    payload = (root / "verification/p1/MIRRA_P1_CERTIFICATE.bin").read_bytes()
    require(payload == canonical(candidate(root)), "candidate bytes differ from exact derivation/source bindings")
    proof_hash = digest(payload)
    pin = (root / "verification/p1/CERTIFICATE.sha256").read_text()
    require(pin == proof_hash + "  MIRRA_P1_CERTIFICATE.bin\n", "proof-file pin mismatch")
    compiled = re.findall(PIN_PATTERN, (root / "src/lib.rs").read_text())
    require(len(compiled) == 1 and compiled[0][1] == proof_hash, "compiled proof pin mismatch")
    if release:
        review = json.loads((root / "verification/p1/REVIEW.json").read_text())
        require(review.get("decision") == "accepted", "independent P1 review is pending or rejected")
        require(review.get("proof_sha256") == proof_hash, "review targets different proof bytes")
        require(review.get("checker_sha256") == digest((root / "verification/p1/check_candidate.py").read_bytes()),
                "review targets a different checker")
        for field in ("reviewer_identity", "reviewed_commit", "review_url", "reviewed_at_utc"):
            require(isinstance(review.get(field), str) and bool(review[field].strip()), "missing review field: " + field)
        require(re.fullmatch(r"[0-9a-f]{40}", review["reviewed_commit"]) is not None, "invalid reviewed commit")
        for field in ("mathematics", "source_correspondence", "integer_safety", "scope_and_downstream", "provenance"):
            require(review.get("scope", {}).get(field) == "accepted", "review scope incomplete: " + field)
    return proof_hash


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--root", type=Path, default=Path(__file__).resolve().parents[2])
    mode = parser.add_mutually_exclusive_group(required=True)
    mode.add_argument("--emit", action="store_true")
    mode.add_argument("--check", action="store_true")
    mode.add_argument("--release", action="store_true")
    args = parser.parse_args()
    try:
        if args.emit:
            print(canonical(candidate(args.root)).decode(), end="")
        else:
            proof_hash = check(args.root, release=args.release)
            print("PASS: exact P1 candidate, source bindings and both pins:", proof_hash)
            print("Outward bound:", BOUND, "output ulp; N=14")
            print("Recorded review accepted; authenticate its provenance separately." if args.release else
                  "Candidate validation only; independent review and release authorization are separate.")
    except (ValueError, OSError, KeyError, TypeError) as error:
        parser.exit(1, "FAIL: " + str(error) + "\n")


if __name__ == "__main__":
    main()
