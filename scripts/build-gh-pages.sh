#!/bin/bash
set -e

echo "==> [ Building WASM...] <=="
cd lbf/wasm-demo
./build-lbf-wasm.sh --target web

echo "==> [ Preparing docs folder...] <=="
cd ../..
rm -rf docs
mkdir -p docs/pkg

cp -v lbf/wasm-demo/index.html docs/
cp -v lbf/wasm-demo/index.js docs/
cp -rv lbf/wasm-demo/pkg/* docs/pkg/

echo -e "\n-- ✅ Done. Ready to push to GitHub."
