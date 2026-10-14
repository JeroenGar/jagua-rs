#!/bin/bash
# Usage: ab.sh <label> <rounds> [filter]
# Builds the ci_bench binary for base (/home/user/jagua-rs-base) and variant (/home/user/jagua-rs),
# then alternates runs: base, var, base, var, ... Output: $S/bench/<label>/{base,var}_<i>.txt
set -e
S=/tmp/claude-0/-home-user-sparrow-pro/c78836ad-23ca-593e-bae7-2f110c743d73/scratchpad
LABEL=$1; ROUNDS=${2:-2}; FILTER=${3:-cde_}
export RUSTFLAGS='-C target-cpu=native -Awarnings'
mkdir -p $S/bench/$LABEL
bin() {
  (cd $1 && cargo bench --locked --bench ci_bench --no-run --message-format=json 2>/dev/null) \
    | grep -o '"executable":"[^"]*ci_bench[^"]*"' | tail -1 | cut -d'"' -f4
}
B=$(bin /home/user/jagua-rs-base); V=$(bin /home/user/jagua-rs)
cp $B $S/bench/$LABEL/base_bin; cp $V $S/bench/$LABEL/var_bin
for i in $(seq 1 $ROUNDS); do
  for w in base var; do
    if [ $w = base ]; then D=/home/user/jagua-rs-base/lbf; else D=/home/user/jagua-rs/lbf; fi
    (cd $D && $S/bench/$LABEL/${w}_bin --bench "$FILTER" --output-format bencher 2>/dev/null) | grep '^test' > $S/bench/$LABEL/${w}_$i.txt
    echo "$w $i done (load $(cut -d' ' -f1 /proc/loadavg))" >> $S/bench/$LABEL/log.txt
  done
done
python3 $S/summarize.py $S/bench/$LABEL
