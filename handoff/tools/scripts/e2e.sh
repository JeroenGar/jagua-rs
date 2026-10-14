#!/bin/bash
# Usage: e2e.sh <tag> <runner-binary> <instance> [seconds]
# Runs seeds 1-3 single-threaded in parallel; prints evals/s (thousands) and widths.
S=/tmp/claude-0/-home-user-sparrow-pro/c78836ad-23ca-593e-bae7-2f110c743d73/scratchpad
TAG=$1; BIN=$2; INST=$3; SECS=${4:-60}
pids=()
for seed in 1 2 3; do
  d=$S/e2e/$TAG-$INST-$seed; rm -rf $d; mkdir -p $d
  (cd $d && exec $BIN -i /home/user/sparrow-pro/data/input/$INST.json -e $SECS -c 1 -s $seed --workers 1 > log.txt 2>&1) &
  pids+=($!)
done
wait "${pids[@]}"
line=""
for seed in 1 2 3; do
  l=$(grep -h 'EXPL\] finished' $S/e2e/$TAG-$INST-$seed/log.txt | sed -E 's/.*width: ([0-9.]+).*evals\/s: ([0-9.]+ ?[KM]?),.*/\2 (w\1)/')
  line="$line $l |"
done
echo "$TAG $INST:$line"
