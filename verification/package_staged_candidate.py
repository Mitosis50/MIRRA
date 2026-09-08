#!/usr/bin/env python3
"""Export the CI-built candidate, exact source identity and measured hashes."""

import hashlib
import json
import os
import shutil
import subprocess
import sys
from pathlib import Path


def main():
    root = Path(__file__).resolve().parents[1]
    output = Path(sys.argv[1])
    output.mkdir(parents=True, exist_ok=False)
    files = {
        "mirra_canister.wasm": root / "target/wasm32-unknown-unknown/release/mirra_canister.wasm",
        "mirra.did": root / "mirra.did",
        "MIRRA_P1_CERTIFICATE.bin": root / "verification/p1/MIRRA_P1_CERTIFICATE.bin",
    }
    hashes = {}
    for name, source in files.items():
        shutil.copyfile(source, output / name)
        hashes[name] = hashlib.sha256(source.read_bytes()).hexdigest()
    git = lambda ref: subprocess.check_output(["git", "rev-parse", ref], cwd=root, text=True).strip()
    record = {
        "status": "STAGED_CANDIDATE_NOT_PRODUCTION_RELEASE",
        "source_commit": git("HEAD"), "source_tree": git("HEAD^{tree}"),
        "workflow_run_id": os.environ.get("GITHUB_RUN_ID"),
        "rustc": subprocess.check_output(["rustc", "--version"], text=True).strip(),
        "sha256": hashes,
        "p1_independent_review": "pending",
        "mainnet_verification": "not_performed",
        "release_attestation": "not_issued",
        "three_clean_path_reproducibility": "pending",
        "rc5_to_new_pin_upgrade": "pending",
        "runtime_upgrade_scope": "same newly built candidate before and after upgrade",
    }
    (output / "BUILD_RECORD.json").write_text(json.dumps(record, indent=2) + "\n")
    (output / "SHA256SUMS").write_text("".join(f"{value}  {name}\n" for name, value in hashes.items()))
    print(json.dumps(record, indent=2))


if __name__ == "__main__":
    main()
