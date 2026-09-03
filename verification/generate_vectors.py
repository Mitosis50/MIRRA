#!/usr/bin/env python3
"""Regenerate or verify the frozen 99-vector MIRRA v1 corpus."""

import argparse
import hashlib
import json
import random
import sys
from pathlib import Path

PROJECT_ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(PROJECT_ROOT))

from verification.reference.mirra_q32 import SCALE, uniform_priors, weights_from_penalties


def generate() -> bytes:
    rng = random.Random(0xD4A)
    vectors = []
    for test_index in range(96):
        k = rng.choice([2, 3, 7, 16, 33, 64])
        style = test_index % 4
        penalties = []
        for index in range(k):
            if style == 0:
                penalties.append(rng.randrange(0, 6 * SCALE * SCALE))
            elif style == 1:
                penalties.append(rng.randrange(0, 25 * SCALE * SCALE))
            elif style == 2:
                penalties.append(0 if index == 0 else rng.randrange(0, 3 * SCALE * SCALE))
            else:
                penalties.append(
                    (17 * SCALE * SCALE if index == 0 else 0)
                    if rng.random() < 0.5
                    else rng.randrange(15 * SCALE * SCALE, 25 * SCALE * SCALE)
                )
        priors = uniform_priors(k)
        vectors.append(
            {
                "k": k,
                "priors": priors,
                "penalties": penalties,
                "bound": 16 * SCALE,
                "floor": 4096,
                "expected": weights_from_penalties(
                    penalties, priors, 16 * SCALE, 4096
                ),
            }
        )
    for penalties in (
        [0, 17 * SCALE * SCALE],
        [20 * SCALE * SCALE, 10 * SCALE * SCALE],
        [0, 0],
    ):
        priors = uniform_priors(2)
        vectors.append(
            {
                "k": 2,
                "priors": priors,
                "penalties": penalties,
                "bound": 16 * SCALE,
                "floor": 4096,
                "expected": weights_from_penalties(
                    penalties, priors, 16 * SCALE, 4096
                ),
            }
        )
    return json.dumps(vectors, separators=(",", ":")).encode()


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--output", type=Path, default=Path("tests/vectors.json"))
    parser.add_argument("--check", action="store_true")
    args = parser.parse_args()
    payload = generate()
    digest = hashlib.sha256(payload).hexdigest()
    if args.check:
        checked_in = args.output.read_bytes()
        if checked_in != payload:
            print("FAIL: generated vectors differ from checked-in vectors")
            print("generated sha256", digest)
            print("checked-in sha256", hashlib.sha256(checked_in).hexdigest())
            return 1
        print("PASS: 99 vectors regenerate byte-for-byte")
    else:
        args.output.parent.mkdir(parents=True, exist_ok=True)
        args.output.write_bytes(payload)
        print(f"wrote vectors to {args.output}")
    print("sha256", digest)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
