#!/bin/bash
set -euxo pipefail

command=$1
expected_file=$2
output=$(cargo ${command} --color=never --quiet 2>&1)
echo "$output" > /tmp/output.txt
if ! diff -u ${expected_file} /tmp/output.txt; then
echo "Cargo ${command} output does not match expected output"
exit 1
fi
