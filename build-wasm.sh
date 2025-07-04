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

# Default values
TARGET="web"
BUILD_TARGET="lbf"
BUILD_TARGET_UNDERSCORE="lbf"
USE_WASM_OPT=true
WASM_OPT_LEVEL="-O4"
WASM_PATH=""
OUT_FILE="optimized.wasm"
BINARYEN_GH_URL="https://github.com/WebAssembly/binaryen"

print_help() {
  echo -e "${YELLOW}Usage: $0 [options]${RESET}"
  echo
  echo -e "${BLUE}Description:${RESET}"
  echo "  This is a clean, optimized WebAssembly build script for the"
  echo -e "  ${GREEN}jagua-rs${RESET} project using ${GREEN}wasm-pack${RESET} and ${GREEN}wasm-opt${RESET}."
  echo
  echo -e "  -> It supports optional ${GREEN}optimization${RESET}, ${GREEN}rayon multithreading${RESET}" 
  echo -e "  -> ${RED}Supports only web due to SAB!${RED}"
  echo
  echo -e "${BLUE}Options:${RESET}"
  echo "  --target <wasm-target>         wasm-pack target (default: web)"
  echo "  --opt <opt-level>              wasm-opt optimization level (default: -O4)"
  echo "  --no-opt                       skip wasm-opt optimization step entirely"
  echo "  --wasm-path <path>             override .wasm input path"
  echo "  -h, --help                     display this help message"
  echo
  echo -e "${BLUE}Example:${RESET}"
  echo "  ./build.sh --target web --opt -O3"
  echo
  echo -e "${YELLOW}Output:${RESET}"
  echo "  - Compiled and optimized .wasm"
  echo "  - Patched rayon helper JS (for thread pool support)"
}

# Parse flags
while [[ "$#" -gt 0 ]]; do
  case "$1" in
    --target)
      TARGET="$2"
      shift 2
      ;;
    --opt)
      WASM_OPT_LEVEL="$2"
      shift 2
      ;;
    --no-opt)
      USE_WASM_OPT=false
      shift
      ;;
    --wasm-path)
      WASM_PATH="$2"
      shift 2
      ;;
    -h|--help)
      print_help
      exit 0
      ;;
    *)
      echo -e "${RED}${FAILURE} Unknown option: $1${RESET}"
      print_help
      exit 1
      ;;
  esac
done

# Check wasm-opt
if ! command -v wasm-opt &> /dev/null; then
  echo -e "${RED}${FAILURE} wasm-opt is not installed or not in PATH!${RESET}"
  echo -e "${BLUE}${INFO} Install it from: ${BINARYEN_GH_URL}${RESET}"
  exit 1
fi

# Detect wasm-pack
if command -v wasm-pack &> /dev/null; then
  WASM_PACK=$(command -v wasm-pack)
elif [ -x "$HOME/.cargo/bin/wasm-pack" ]; then
  WASM_PACK="$HOME/.cargo/bin/wasm-pack"
else
  echo -e "${RED}${FAILURE} wasm-pack not found in PATH or ~/.cargo/bin!${RESET}"
  echo -e "${BLUE}${INFO} Install it via: cargo install wasm-pack${RESET}"
  exit 1
fi

echo -e "${GREEN}${INFO} Using wasm-pack at: ${WASM_PACK}${RESET}"

# Build
echo -e "${BLUE}${INFO} Entering project directory '${BUILD_TARGET}'...${RESET}"
cd "$BUILD_TARGET" || { echo -e "${RED}${FAILURE} Failed to enter ${BUILD_TARGET} directory!${RESET}"; exit 1; }

echo -e "${YELLOW}${STEP} Compiling using wasm-pack with target '${TARGET}'...${RESET}"
"$WASM_PACK" build --target "$TARGET" --release

if [ $? -eq 0 ]; then
  echo -e "\n${GREEN}${SUCCESS} Build successful for target '${RED}${TARGET}${GREEN}'!${RESET} ${BLUE}Check the '$BUILD_TARGET/pkg' directory for output.${RESET}"
else
  echo -e "\n${RED}${FAILURE} Build failed!${RESET}"
  exit 1
fi

# Default wasm path
if [ -z "$WASM_PATH" ]; then
  WASM_PATH="pkg/${BUILD_TARGET_UNDERSCORE}_bg.wasm"
fi

if [ "$USE_WASM_OPT" = true ]; then
  # Show full wasm-opt args
  WASM_OPT_ARGS="$WASM_OPT_LEVEL --dce --enable-simd --enable-bulk-memory --enable-threads -ifwl -ffm --memory-packing --enable-gc"
  echo -e "${YELLOW}${STEP} Running wasm-opt with the following arguments:${RESET}"
  echo -e "${BLUE}wasm-opt $WASM_OPT_ARGS -o \"$OUT_FILE\" \"$WASM_PATH\"${RESET}"

  # Run wasm-opt
  wasm-opt $WASM_OPT_ARGS -o "$OUT_FILE" "$WASM_PATH"

  if [ $? -eq 0 ]; then
    mv -v "$OUT_FILE" "$WASM_PATH"
    echo -e "${GREEN}${SUCCESS} wasm-opt optimization applied and saved to ${WASM_PATH}.${RESET}"
  else
    echo -e "${RED}${FAILURE} wasm-opt optimization failed.${RESET}"
    exit 1
  fi
else
  echo -e "${YELLOW}${INFO} Skipping wasm-opt as per --no-opt flag.${RESET}"
fi

echo -e "${BLUE}${STEP} Present directory: $(pwd)${RESET}"

#################################################
#
# This patch is needed to ensure that during 
# browser run time, pkg is resolved to an 
# actual file '$BUILD_TARGET.js'
#
#################################################

WORKER_FILE=$(find pkg/snippets/ -type f -name workerHelpers.js | head -n 1)

if [ -z "$WORKER_FILE" ]; then
  echo -e "${RED}${FAILURE} workerHelpers.js not found!${RESET}"
  exit 1
fi

echo -e "${YELLOW}${STEP} Patching import in ${WORKER_FILE}...${RESET}"
sed -i "s|const pkg = await import('../../..');|const pkg = await import('../../../${BUILD_TARGET_UNDERSCORE}.js');|" "$WORKER_FILE"

echo -e "${GREEN}${SUCCESS} Patch applied.${RESET}"
