#!/usr/bin/env python3
"""Check this runner without installing tools or attempting a canister run."""

import json
import shutil
import socket
import subprocess
import sys
import tempfile
from pathlib import Path


def main():
    checks = {}
    with tempfile.TemporaryDirectory(prefix="mirra-preflight-") as temporary:
        try:
            with socket.socket(socket.AF_UNIX, socket.SOCK_STREAM) as endpoint:
                endpoint.bind(str(Path(temporary) / "probe.sock"))
            checks["unix_socket"] = {"ok": True}
        except (AttributeError, OSError) as error:
            checks["unix_socket"] = {"ok": False, "reason": str(error)}
    for name in ("cargo", "rustc", "timeout"):
        path = shutil.which(name)
        checks[name] = {"ok": path is not None, "path": path}
    if checks["rustc"]["ok"]:
        try:
            version = subprocess.run(
                [checks["rustc"]["path"], "--version"],
                capture_output=True, text=True, timeout=30, check=False,
            )
            checks["rustc"]["ok"] = (
                version.returncode == 0 and version.stdout.startswith("rustc 1.88.0 ")
            )
            checks["rustc"]["version"] = version.stdout.strip()
        except (OSError, subprocess.TimeoutExpired) as error:
            checks["rustc"] = {"ok": False, "reason": str(error)}
    ready = all(check["ok"] for check in checks.values())
    print(json.dumps({
        "status": "PREFLIGHT_PASSED" if ready else "BLOCKED",
        "checks": checks,
        "runtime_tests_executed": False,
        "certificate_verified": False,
        "release_ready": False,
    }, indent=2))
    return 0 if ready else 1


if __name__ == "__main__":
    sys.exit(main())
