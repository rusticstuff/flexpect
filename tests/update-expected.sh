#!/bin/bash
set -euo pipefail

cargo check --all-features --quiet --color=never --test simple> check-expect-supported-simple.out 2>&1
cargo check --all-features --quiet --color=never --test submod> check-expect-supported-submod.out 2>&1
cargo clippy --all-features --quiet --color=never --test simple> clippy-expect-supported-simple.out 2>&1
cargo clippy --all-features --quiet --color=never --test submod> clippy-expect-supported-submod.out 2>&1
