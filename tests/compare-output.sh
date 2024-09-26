#!/bin/bash
set -euo pipefail
command=$1
expected_file=$2
output=$(cargo ${command} 2>&1)
echo "$output" > output.txt
if ! diff -u ${expected_file} output.txt; then
echo "Cargo ${command} output does not match expected output"
exit 1
fi
