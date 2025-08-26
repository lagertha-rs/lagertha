#!/usr/bin/env bash
set -euo pipefail

if ! javac --version 2>&1 | grep -q '^javac 24\.0\.2'; then
  echo "Error: expected javac 24.0.2, got: $(javac --version 2>&1)"
  exit 1
fi

OUT_DIR="target/test-classes"
mkdir -p "$OUT_DIR"

find testdata/java -name '*.java' -print0 | xargs -0 javac -g -d "$OUT_DIR"

echo "Compiled fixtures to $OUT_DIR"
