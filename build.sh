#!/usr/bin/env bash
set -euo pipefail

# Always build from the project folder, even if this script was called elsewhere.
cd -- "$(dirname -- "${BASH_SOURCE[0]}")"

# Cargo and Make already track changes, so a regular build doesn't need a clean.
# Additional Make arguments can be passed through, for example ./build.sh clean.
make -j"$(nproc)" "$@"
