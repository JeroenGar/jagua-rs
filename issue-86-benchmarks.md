# Issue 86: M1 Pro benchmark experiment

The direct per-quadrant rectangle checks fix the known false-negative reproducer.
On `ci_bench`, collision collection takes about 4% less time. Detection ranges
from 2.4% less time to 1.1% more time, depending on depth. Update differences are
small. There is no broad performance regression in this workload.

## Candidate

Replace the special edge/bisector classifier with:

```rust
let classified = qs.map(|q| self.collides_with(q));
```

Retain the independent f64 debug assertion. Remove the unused intersection-half
helpers. This experiment uses the existing production f32 rectangle-edge
predicate; it does not move the f64 assertion oracle into production.

The candidate was applied only to the remote scratch checkout. The local
working checkout's classifier is unchanged.

## Method

- Machine: MacBook Pro 14, Apple M1 Pro, 10 CPU cores, 32 GB RAM.
- SSH alias: `poab-m1`.
- All timed runs were on AC power, with Low Power Mode off.
- CPU sampling immediately before timing showed 91.4% idle. No thermal or
  performance warning was reported before or after timing.
- Compiler: Rust 1.98.1 stable, aarch64-apple-darwin.
- CI flags: `RUSTFLAGS="-C target-cpu=native -Awarnings"`.
- Base commit: `824ab31cf8a58eecf5d87527260c92510626661b`, plus the current
  debug-assertion hardening and regression tests. Those changes do not alter
  release classifier behavior.
- Identical Cargo.lock, built with `--locked`. Lockfile SHA-256:
  `fb4323202db8654a4028421f6650a96b7a2246dfdb3cdf45995f273277c47703`.
- Unmodified `lbf/benches/ci_bench.rs`, using its seeded swim workload and
  depths 3, 4, 5.
- Four serial runs: baseline-a, direct-b, direct-c, baseline-d.
- Criterion: 100 samples, 2-second warmup, nominal 5-second measurement.
  Criterion automatically extended measurements where needed.
- Both executables were built before timing. No compilation ran concurrently.
- `caffeinate -i` prevented idle sleep during each benchmark process.

## Results

Times are milliseconds per batch of 1,000 operations. Each column aggregates
the two runs using the geometric mean of Criterion's point estimates: slope
when available, otherwise mean. Negative time change means faster.

| Benchmark | Depth | Baseline ms | Direct ms | Time change |
|---|---:|---:|---:|---:|
| cde_collect_1k | 3 | 1.4212 | 1.3645 | -4.0% |
| cde_collect_1k | 4 | 1.4242 | 1.3690 | -3.9% |
| cde_collect_1k | 5 | 1.4256 | 1.3687 | -4.0% |
| cde_detect_1k | 3 | 0.1651 | 0.1669 | +1.1% |
| cde_detect_1k | 4 | 0.1339 | 0.1314 | -1.9% |
| cde_detect_1k | 5 | 0.1189 | 0.1161 | -2.4% |
| cde_update_1k | 3 | 1.5446 | 1.5481 | +0.2% |
| cde_update_1k | 4 | 2.7630 | 2.7173 | -1.7% |
| cde_update_1k | 5 | 5.5899 | 5.5464 | -0.8% |

Collection improved in both comparisons at every depth. Its paired time
reductions ranged from 3.2% to 4.8%. The depth-3 detection result was less
consistent: one comparison was 0.25% slower and the other 1.99% slower. Treat
that as a small possible cost, not evidence of a universal slowdown or a
precisely established 1% regression.

These are results for one machine and the existing CI workload, not every
geometry or deployment target.

## Correctness checks

- Baseline: the explicit release reproducer failed with
  `missed square collision at depth 3`.
- Candidate: the same reproducer passed in debug and release.
- Candidate: all 22 active workspace tests passed with all features.
- The existing ignored status of the reproducer was bypassed with `--ignored`
  for both comparisons.
- These checks establish the known-case fix, not a proof that every f32
  rectangle-edge calculation is conservative.

## Scratch files and reproducibility

Remote directory: `/Users/jern/jagua-issue-86.Dx2wyj`.

It contains `src/`, the baseline and direct source variants, both benchmark
executables in `bin/`, `run-benchmarks.sh`, and raw logs and Criterion JSON in
`results/`. A local copy of the results is in
`/tmp/jagua-remote-bench.sRLmaA/results`.

Executable SHA-256:

- Baseline: `c3c16c2fe19df9fd6fddbc262fcf54a1ab80e5abe6c54584dcd14e1253023119`
- Direct: `d3d75035acbaa54aead60b0968b7e032c2d7469aeae6deace25c8d3335b78ca7`

The timed command for each saved executable was:

```sh
cd /Users/jern/jagua-issue-86.Dx2wyj/src/lbf
CRITERION_HOME=/Users/jern/jagua-issue-86.Dx2wyj/results/criterion \
  caffeinate -i /Users/jern/jagua-issue-86.Dx2wyj/bin/direct \
  --bench --noplot --warm-up-time 2 --measurement-time 5 \
  --sample-size 100 --save-baseline direct-b
```
