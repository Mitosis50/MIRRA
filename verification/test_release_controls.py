"""Release-control regressions; no compiler, network, or emulator is used."""

import hashlib
import os
import shutil
import subprocess
import tempfile
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
WITHDRAWN = "b9d1c0d5e0d8144d88f6e1375429fc844967731090d878ebd311072ee456a57b"


class ProofIntakeTests(unittest.TestCase):
    def check_intake(self, expected, payload=None):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            (root / "scripts").mkdir()
            proof = root / "verification/p1"
            proof.mkdir(parents=True)
            shutil.copy2(ROOT / "scripts/verify-p1-certificate.sh", root / "scripts")
            (proof / "CERTIFICATE.sha256").write_text(
                f"{expected}  MIRRA_P1_CERTIFICATE.bin\n"
            )
            if payload is not None:
                (proof / "MIRRA_P1_CERTIFICATE.bin").write_bytes(payload)
            return subprocess.run(
                ["bash", str(root / "scripts/verify-p1-certificate.sh")],
                capture_output=True, text=True, check=False,
            )

    def test_withdrawn_pin_blocks_even_before_bytes_are_supplied(self):
        result = self.check_intake(WITHDRAWN)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("pinned P1 proof is withdrawn", result.stderr)
        self.assertNotIn("PASS", result.stdout)

    def test_replacement_pin_still_requires_bytes(self):
        result = self.check_intake("a" * 64)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("missing or empty", result.stderr)

    def test_mismatched_bytes_cannot_pass_intake(self):
        result = self.check_intake("a" * 64, b"wrong artifact")
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("SHA-256 mismatch", result.stderr)

    def test_nonwithdrawn_hash_match_remains_only_a_byte_check(self):
        # Synthetic data in an isolated fixture, never installed as release evidence.
        payload = b"unit test: not a mathematical proof"
        result = self.check_intake(hashlib.sha256(payload).hexdigest(), payload)
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertIn("certificate bytes match", result.stdout)


class BuildEnvironmentTests(unittest.TestCase):
    def test_dfx_and_wasm_builder_use_same_remaps_with_optional_homes(self):
        for cargo_set, rustup_set in ((False, False), (True, False), (False, True), (True, True)):
            with self.subTest(cargo_home=cargo_set, rustup_home=rustup_set):
                with tempfile.TemporaryDirectory(prefix="mirra build ") as directory:
                    root = Path(directory)
                    scripts = root / "scripts"
                    scripts.mkdir()
                    binaries = root / "tools/bin"
                    binaries.mkdir(parents=True)
                    for name in ("cargo", "dfx"):
                        tool = binaries / name
                        tool.write_text('#!/usr/bin/env bash\nprintf "%s\\n" "$RUSTFLAGS"\n')
                        tool.chmod(0o755)
                    shutil.copy2(ROOT / "scripts/build-wasm.sh", scripts)
                    release = (ROOT / "scripts/verify-release.sh").read_text()
                    definitions = release.split('\nrun_gate "rustc 1.88.0 pin"', 1)[0]
                    self.assertNotEqual(definitions, release)
                    (scripts / "dfx-probe.sh").write_text(definitions + "\nverify_dfx\n")
                    env = dict(os.environ)
                    for name in ("CARGO_HOME", "RUSTUP_HOME", "CARGO", "DFX"):
                        env.pop(name, None)
                    env["PATH"] = str(binaries) + os.pathsep + env["PATH"]
                    for name, enabled in (("CARGO_HOME", cargo_set), ("RUSTUP_HOME", rustup_set)):
                        if enabled:
                            home_path = root / name.lower()
                            home_path.mkdir()
                            env[name] = str(home_path)
                    results = [subprocess.run(
                        ["bash", str(scripts / script)], env=env,
                        capture_output=True, text=True, check=False,
                    ) for script in ("build-wasm.sh", "dfx-probe.sh")]
                    for result in results:
                        self.assertEqual(result.returncode, 0, result.stderr)
                    self.assertEqual(results[0].stdout, results[1].stdout)


if __name__ == "__main__":
    unittest.main()
