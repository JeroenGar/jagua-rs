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

# Post-build patch: fix import path in workerHelpers.js
WORKER_FILE=$(find lbf/pkg/snippets/ -type f -name workerHelpers.js | head -n 1)
PATCH_LINE='const pkg = await import("../../../lbf.js");'
ORIGINAL_LINE="const pkg = await import(\'../../..\');"

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
~/.cargo/bin/wasm-pack build --target "$TARGET" --release

if [ $? -eq 0 ]; then
  echo -e "\n${GREEN}${SUCCESS} Build successful for target '${RED}${TARGET}${GREEN}'!${RESET} ${BLUE}Check the 'lbf/pkg' directory for output.${RESET}"
else
  echo -e "\n${RED}${FAILURE} Build failed!${RESET}"
  exit 1
fi

if [ -z "$WORKER_FILE" ]; then
  echo "workerHelpers.js not found!"
  exit 1
fi

echo -e "${BLUE}${STEP} Checking out of lbf..${RESET}"

cd ..

#################################################
#
# This patch is needed to ensure that during 
# browser run time, pkg is resolved to an 
# actual file 'lbf.js'
#
#################################################

echo "${STEP} Patching $WORKER_FILE..."

# Use sed to replace the import line
sed -i "s|const pkg = await import('../../..');|const pkg = await import('../../../lbf.js');|" "$WORKER_FILE"

echo -e "${GREEN}${SUCCESS} Patch applied.${RESET}"
