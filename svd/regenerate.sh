#!/bin/bash

SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )

TARGET="cortex-m"

SVD="${SCRIPT_DIR}/r7fa6m5bh_fixed.svd"
BUILD="${SCRIPT_DIR}/tmp"
SVD2RUST_OPTIONS="--impl-debug --atomics --target ${TARGET}"

# Clean previous content
rm -rf "${BUILD}"
mkdir -p "${BUILD}"
rm -rf "${SCRIPT_DIR}/../src"
mkdir -p "${SCRIPT_DIR}/../src"

echo "SVD Path ${SCRIPT_DIR}/r7fa6m5bh_fixed.svd"s
svd2rust -i "$SVD" -o "$BUILD" $SVD2RUST_OPTIONS
form -i "${BUILD}/lib.rs" -o "${SCRIPT_DIR}/../src" --format-output