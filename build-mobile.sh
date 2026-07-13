#!/usr/bin/env bash
# Copyright (c) 2026 Edison Lepiten / AIEONYX
# License: Apache-2.0
#
# build-mobile.sh — Phase M1 build script.
# Run from the root of the edisondb repo on your Prague workstation.
# Prereqs: cargo-ndk installed, Android NDK path set.

set -euo pipefail

AISTOP_DIR="${AISTOP_DIR:-../aistop}"
JNI_LIBS="${AISTOP_DIR}/app/src/main/jniLibs"

echo "=== EdisonDB Mobile Build ==="
echo "Output: ${JNI_LIBS}"

# 1. Ensure Android targets are registered
rustup target add aarch64-linux-android x86_64-linux-android

# 2. Cross-compile with mobile feature, no server stack
cargo ndk \
  -t arm64-v8a \
  -t x86_64 \
  -o "${JNI_LIBS}" \
  build --release \
  --features mobile \
  --no-default-features

echo ""
echo "=== Build complete ==="
ls -lh "${JNI_LIBS}/arm64-v8a/libeditsondb.so" \
        "${JNI_LIBS}/x86_64/libeditsondb.so" 2>/dev/null || true

# 3. Commit to edisondb repo
echo ""
echo "=== Committing to edisondb ==="
git add -A
git commit -m "feat(mobile): Phase M1 Android FFI bridge + cargo-ndk build

- Add mobile feature flag (strips gRPC/tonic)
- C-ABI exports: edisondb_open/insert/query/delete/close/free_string
- JNI bridge for com.aieonyx.edisondb.EdisonDbAndroid
- ARPi header validation in MobileDb::insert
- BLAKE3 content hash verification on every write
- Cross-compiled to arm64-v8a + x86_64 via cargo-ndk
- libeditsondb.so output to aistop jniLibs

Copyright (c) 2026 Edison Lepiten / AIEONYX"

git push origin main

echo ""
echo "=== EdisonDB mobile commit pushed ==="
echo ""
echo "Next: copy Android SDK files to aistop and run Phase M3 integration."
