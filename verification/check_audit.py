#!/usr/bin/env python3
"""Reject vulnerabilities and any informational advisory not explicitly reviewed."""

import datetime as dt
import json
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]


def observed(path: Path) -> set[tuple[str, str, str]]:
    report = json.loads(path.read_text())
    vulnerabilities = report["vulnerabilities"]
    if vulnerabilities["found"] or vulnerabilities["count"]:
        raise SystemExit(f"FAIL: {path.name} contains known vulnerabilities")
    result = set()
    for entries in report.get("warnings", {}).values():
        for entry in entries:
            result.add(
                (
                    entry["advisory"]["id"],
                    entry["package"]["name"],
                    entry["package"]["version"],
                )
            )
    return result


def expected(entries: list[dict[str, str]]) -> set[tuple[str, str, str]]:
    return {(item["id"], item["crate"], item["version"]) for item in entries}


def main() -> int:
    policy = json.loads((ROOT / "verification/dependency-exceptions.json").read_text())
    due = dt.date.fromisoformat(policy["review_due"])
    if dt.date.today() > due:
        print(f"FAIL: dependency exceptions expired on {due}", file=sys.stderr)
        return 1
    checks = {
        "production": ROOT / "security/cargo-audit-main.json",
        "integration": ROOT / "security/cargo-audit-integration.json",
    }
    failed = False
    for scope, path in checks.items():
        actual = observed(path)
        allowed = expected(policy[scope])
        if actual != allowed:
            print(f"FAIL: {scope} advisory set changed", file=sys.stderr)
            print(f"  unexpected: {sorted(actual - allowed)}", file=sys.stderr)
            print(f"  missing: {sorted(allowed - actual)}", file=sys.stderr)
            failed = True
    if failed:
        return 1
    print(f"PASS: vulnerabilities=0; reviewed advisory set exact through {due}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
