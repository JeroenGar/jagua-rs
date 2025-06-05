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
TARGET="nodejs"

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

echo -e "${BLUE}${INFO} Entering LBF target...${RESET}"
cd lbf || { echo -e "${RED}${FAILURE} Failed to enter lbf directory!${RESET}"; exit 1; }

echo -e "${YELLOW}${STEP} Compiling using wasm-pack with target '${TARGET}'...${RESET}"
RUSTFLAGS='--cfg getrandom_backend="wasm_js"' ~/.cargo/bin/wasm-pack build --target "$TARGET"

if [ $? -eq 0 ]; then
  echo -e "\n${GREEN}${SUCCESS} Build successful for target '${RED}${TARGET}${GREEN}'!${RESET} ${BLUE}Check the 'lbf' directory for output.${RESET}"
else
  echo -e "\n${RED}${FAILURE} Build failed!${RESET}"
  exit 1
fi
