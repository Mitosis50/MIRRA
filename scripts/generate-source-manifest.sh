#!/usr/bin/env bash
set -euo pipefail

project_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "${project_root}"
find . -type f \
  ! -path './target/*' \
  ! -path './target-*/*' \
  ! -path './integration-tests/target/*' \
  ! -path './.dfx/*' \
  ! -path './.git/*' \
  ! -path '*/__pycache__/*' \
  ! -name '*.pyc' \
  ! -name 'SOURCE_MANIFEST.sha256' \
  -print0 | sort -z | xargs -0 sha256sum > SOURCE_MANIFEST.sha256
echo "wrote SOURCE_MANIFEST.sha256"
