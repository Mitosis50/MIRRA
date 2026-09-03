#!/usr/bin/env python3
"""Independent MIRRA public-snapshot commitment verifier."""

import hashlib
import json
import sys


def field(value: bytes) -> bytes:
    return len(value).to_bytes(4, "big") + value


def commitment(snapshot: dict) -> bytes:
    h = hashlib.sha256()
    h.update(b"mirra.snapshot.v1\0")
    h.update(field(snapshot["protocol"].encode()))
    h.update(field(bytes.fromhex(snapshot["protocol_fingerprint"])))
    h.update(int(snapshot["schema_version"]).to_bytes(4, "big"))
    h.update(int(snapshot["seq"]).to_bytes(8, "big"))
    h.update(int(snapshot["expert_count"]).to_bytes(8, "big"))
    h.update(field(bytes.fromhex(snapshot["state_root"])))
    h.update(field(snapshot["vector_sha256"].encode()))
    h.update(field(snapshot["p1_cert_sha256"].encode()))
    return h.digest()


def main() -> int:
    document = json.load(sys.stdin)
    actual = commitment(document["snapshot"]).hex()
    expected = document["snapshot_commitment"].lower()
    if actual != expected:
        print(f"FAIL expected={expected} actual={actual}", file=sys.stderr)
        return 1
    print(f"OK {actual}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
