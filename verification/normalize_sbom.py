#!/usr/bin/env python3
"""Remove build-host source paths from a cargo-cyclonedx JSON document."""

from __future__ import annotations

import argparse
from pathlib import Path


CANONICAL_SOURCE_URI = "file:///mirra/source"


def normalize(sbom: Path, project_root: Path) -> None:
    source_uri = project_root.resolve().as_uri()
    document = sbom.read_text(encoding="utf-8")
    normalized = document.replace(source_uri, CANONICAL_SOURCE_URI)

    if normalized == document:
        raise SystemExit(f"no source URI found in {sbom}: {source_uri}")
    if "path+file:///workspace/" in normalized:
        raise SystemExit(f"unnormalized workspace path remains in {sbom}")

    sbom.write_text(normalized, encoding="utf-8")


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("sbom", type=Path)
    parser.add_argument("project_root", type=Path)
    args = parser.parse_args()
    normalize(args.sbom, args.project_root)


if __name__ == "__main__":
    main()
