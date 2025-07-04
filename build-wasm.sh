#!/bin/bash

# ANSI color codes
GREEN="\033[1;32m"
RED="\033[1;31m"
YELLOW="\033[1;33m"
BLUE="\033[1;34m"
RESET="\033[0m"

# ASCII markers
INFO="[*]"
SUCCESS="[+]"
FAILURE="[!]"
STEP="[>]"

# Default target
TARGET="web"
BUILD_TARGET="lbf"
BUILD_TARGET_UNDERSCORE="lbf"

if ! command -v wasm-opt &> /dev/null; then
  echo -e "${RED}${FAILURE} wasm-opt is not installed or not in PATH!${RESET}"
  echo -e "${BLUE}${INFO} Install it from: https://github.com/WebAssembly/binaryen${RESET}"
  exit 1
fi

# Parse flags
while [[ "$#" -gt 0 ]]; do
  case "$1" in
    --target)
      TARGET="$2"
      shift 2
      ;;
    *)
      echo -e "${RED}${FAILURE} Unknown option: $1${RESET}"
      exit 1
      ;;
  esac
done

echo -e "${BLUE}${INFO} Entering Genetic-Algorithm target...${RESET}"
cd $BUILD_TARGET || { echo -e "${RED}${FAILURE} Failed to enter ${BUILD_TARGET} directory!${RESET}"; exit 1; }

echo -e "${YELLOW}${STEP} Compiling using wasm-pack with target '${TARGET}'...${RESET}"
~/.cargo/bin/wasm-pack build --target "$TARGET" --release

if [ $? -eq 0 ]; then
  echo -e "\n${GREEN}${SUCCESS} Build successful for target '${RED}${TARGET}${GREEN}'!${RESET} ${BLUE}Check the '$BUILD_TARGET/pkg' directory for output.${RESET}"
else
  echo -e "\n${RED}${FAILURE} Build failed!${RESET}"
  exit 1
fi

WASM_PATH="pkg/${BUILD_TARGET_UNDERSCORE}_bg.wasm"
OPTIMIZED="optimized.wasm"

echo -e "${YELLOW}${STEP} Running wasm-opt on ${WASM_PATH}...${RESET}"
wasm-opt -O4 --dce --enable-simd --enable-bulk-memory --enable-threads -ifwl -ffm --memory-packing --enable-gc -o "$OPTIMIZED" "$WASM_PATH"

if [ $? -eq 0 ]; then
  mv -v "$OPTIMIZED" "$WASM_PATH"
  echo -e "${GREEN}${SUCCESS} wasm-opt optimization applied and saved to ${WASM_PATH}.${RESET}"
else
  echo -e "${RED}${FAILURE} wasm-opt optimization failed.${RESET}"
  exit 1
fi

echo -e "${BLUE}${STEP} Present directory: $(pwd)${RESET}"

#################################################
#
# This patch is needed to ensure that during 
# browser run time, pkg is resolved to an 
# actual file '$BUILD_TARGET.js'
#
#################################################

# Post-build patch: fix import path in workerHelpers.js
WORKER_FILE=$(find pkg/snippets/ -type f -name workerHelpers.js | head -n 1)
ORIGINAL_LINE="const pkg = await import(\'../../..\');"

if [ -z "$WORKER_FILE" ]; then
  echo "workerHelpers.js not found!"
  exit 1
fi

echo "${STEP} Patching $WORKER_FILE..."

# Use sed to replace the import line
sed -i "s|const pkg = await import('../../..');|const pkg = await import('../../../${BUILD_TARGET_UNDERSCORE}.js');|" "$WORKER_FILE"

echo -e "${GREEN}${SUCCESS} Patch applied.${RESET}"
