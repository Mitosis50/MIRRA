"""Exercise proof/source/pin mismatches and pending-review rejection."""

import copy
import hashlib
import importlib.util
import json
import re
import shutil
import subprocess
import sys
import tempfile
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(ROOT / "verification/p1"))
from check_candidate import PIN_PATTERN, canonical, check  # noqa: E402
from derive_candidate import compute  # noqa: E402


class CandidateTests(unittest.TestCase):
    def fixture(self):
        directory = tempfile.TemporaryDirectory()
        self.addCleanup(directory.cleanup)
        root = Path(directory.name)
        shutil.copytree(ROOT / "src", root / "src")
        shutil.copytree(ROOT / "verification/p1", root / "verification/p1",
                        ignore=shutil.ignore_patterns("__pycache__"))
        return root

    def repin(self, root, payload):
        value = hashlib.sha256(payload).hexdigest()
        (root / "verification/p1/MIRRA_P1_CERTIFICATE.bin").write_bytes(payload)
        (root / "verification/p1/CERTIFICATE.sha256").write_text(value + "  MIRRA_P1_CERTIFICATE.bin\n")
        lib = root / "src/lib.rs"
        lib.write_text(re.sub(PIN_PATTERN, lambda m: m[1] + value + m[3], lib.read_text()))

    def test_adapter_preserves_archived_exact_derivation(self):
        actual = compute(ROOT)
        archived = json.loads((ROOT / "verification/p1/review/MIRRA_P1_CORRECTED_BUDGET_2026-09-03.json").read_text())
        for key in ("constants", "range", "witness", "new_derivation",
                    "mechanical_old_formula_with_true_tmax_not_new_proof"):
            self.assertEqual(actual[key], archived[key], key)
        check(ROOT)

    def test_rehashed_false_claims_cannot_pass(self):
        original = json.loads((ROOT / "verification/p1/MIRRA_P1_CERTIFICATE.bin").read_text())
        mutations = [
            lambda p: p["claim"].update(absolute_error_output_ulps_le="0.500000042"),
            lambda p: p["derivation"]["range"].update(safe_analytic_t={"exact_fraction": "1/5"}),
            lambda p: p["derivation"]["new_derivation"]["k_cases"].pop(),
            lambda p: p["claim"].update(universal_correct_rounding_claimed=True),
        ]
        for index, mutate in enumerate(mutations):
            with self.subTest(index=index):
                root = self.fixture()
                payload = copy.deepcopy(original)
                mutate(payload)
                self.repin(root, canonical(payload))
                with self.assertRaisesRegex(ValueError, "candidate bytes differ"):
                    check(root)

    def test_arithmetic_drift_is_rejected_before_candidate_comparison(self):
        root = self.fixture()
        core = root / "src/core.rs"
        core.write_text(core.read_text().replace("const N_SER: usize = 14", "const N_SER: usize = 13"))
        with self.assertRaisesRegex(ValueError, "Rust arithmetic source drift"):
            check(root)

    def test_compiled_pin_drift_is_rejected(self):
        root = self.fixture()
        lib = root / "src/lib.rs"
        lib.write_text(re.sub(PIN_PATTERN, lambda m: m[1] + "a" * 64 + m[3], lib.read_text()))
        with self.assertRaisesRegex(ValueError, "compiled proof pin mismatch"):
            check(root)

    def test_candidate_pass_does_not_close_pending_review(self):
        check(ROOT)
        with self.assertRaisesRegex(ValueError, "independent P1 review is pending"):
            check(ROOT, release=True)
        result = subprocess.run(["bash", str(ROOT / "scripts/verify-p1-certificate.sh")],
                                capture_output=True, text=True, check=False)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("independent P1 review is pending", result.stderr)


if __name__ == "__main__":
    unittest.main()
