#!/bin/bash
set -euo pipefail

cargo +1.38.0 check --quiet --color=never --test simple >check-expect-ignored-simple.out 2>&1
