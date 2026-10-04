#!/usr/bin/env bash
set -euo pipefail

# Keep Switch-specific Cargo configuration out of host tests.
cd -- "$(dirname -- "${BASH_SOURCE[0]}")/game_core"
cargo test "$@"
