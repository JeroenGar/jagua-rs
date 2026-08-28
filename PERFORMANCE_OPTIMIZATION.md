# Performance optimization guide

This guide defines how to run an unattended performance campaign on `jagua-rs`.
The target is higher throughput in `lbf/benches/ci_bench.rs` without making the
collision detector less reliable or harder to maintain.

Use `$profile-guided-optimization` to run the campaign. Use `$jeroens-rust` for
every Rust implementation, review, and invariant check.

## Contract

- `ci_bench` is the performance oracle. Keep its workload representative of
  production collision detection, collection, and update operations.
- Quadtree depth 4 is the primary performance target. Optimize and rank
  candidates by depth-4 query throughput; use depths 3 and 5 as correctness and
  meaningful-regression guardrails rather than co-equal optimization targets.
- Correctness takes priority over throughput. A candidate must not introduce a
  false negative.
- Conservative false positives are acceptable only at genuine numerical edge
  cases. Extra false positives in clearly separated geometry reduce packing
  quality and are regressions.
- Preserve validation, floating-point semantics, and error handling.
- Keep debug assertions when replacing data structures, indexes, caches, or
  control flow. Recompute their expected state from authoritative data.
- Do not introduce unsafe code for optimization.
- A change must earn its complexity. Reject a small gain when the permanent code
  obscures the algorithm or creates fragile state.

## Authority and campaign state

An unattended campaign needs explicit authority to commit, push, create or edit
a pull request, and keep working until a stated deadline. It never implies
authority to merge, release, upgrade dependencies, add unsafe code, or change
collision semantics.

Before editing, record:

- Starting commit and branch
- Clean or dirty worktree state
- `rustc -Vv`
- `Cargo.lock` hash, even though the lockfile is ignored
- Cargo features, build profile, benchmark fixture, and seed
- Machine power state and relevant background load
- Benchmark, profiling, correctness, test, and CI commands
- Authorized Git and GitHub actions
- Deadline and stop conditions

Keep restart-safe working notes under the ignored directory
`.agent-notes/performance-optimization/`:

- `WORKFLOW.md` contains commands, environment, baseline, latest accepted commit,
  authority, and deadline.
- `EXPERIMENTS.md` contains every accepted and rejected experiment.
- `BACKLOG.md` contains profiler-backed hypotheses, not commitments.

Read all three files after a restart or context compaction and before repeating
an idea.

## Freeze the benchmark

Audit `ci_bench` before taking the baseline. Make every measured result observable
through `criterion::black_box` so the compiler cannot discard the work.

Treat benchmark-harness changes as measurement changes. Re-establish the baseline
after one, and never report the resulting difference as a library speedup.

Once the baseline is established, freeze:

- Benchmark source and fixture
- Compiler and dependency resolution
- Features and build profile
- Target directory
- Query seed and configuration
- Machine and power conditions

Compare each candidate with its immediate parent. Report cumulative improvement
separately against the frozen starting commit.

## Correctness oracle

Do not record verification output inside the timed Criterion loop. Generate a
fixed query corpus outside the benchmark and run it through an untimed verifier.

Each query record contains:

- Stable query identifier
- Query kind
- Item identifier and transformation
- Final collision result
- Sorted colliding `HazardEntity` values for collection queries

Compare stable hazard entities, not `HazKey` values. SlotMap keys may change after
a representation change.

The corpus uses the same representative scene and deterministic sampler as
`ci_bench`. It also contains boundary cases created with small ULP perturbations
of translations and rotations.

### Parent comparison

Run the exact same corpus on the immediate parent and candidate. An exact match
is preferred, but neither side is assumed to be geometrically correct merely
because it is the parent.

Any difference in the collision boolean or collected hazard set blocks automatic
acceptance and starts an independent mismatch investigation.

### Depth-zero traversal comparison

Build a depth-zero CDE with the same hazards. It scans the complete hazard edge
set without spatial pruning. Run the same queries at depth zero and at every
optimized depth used by `ci_bench`.

A collision or hazard found at depth zero must not disappear at an optimized
depth. A difference blocks acceptance and enters the same independent
investigation.

For an unfiltered query, also verify:

```text
detect_result == !collected_hazards.is_empty()
```

### Mutation comparison

Exercise the fixed remove and place sequence used by the update benchmark. After
each mutation, compare fixed probes against:

1. The incrementally updated CDE.
2. A freshly rebuilt CDE containing the same hazards.

Keep the existing `layout_qt_matches_fresh_qt` debug assertions. Add a similarly
independent debug assertion for any new cache, index, compact representation, or
derived state.

## Investigate every behavior difference

Give every non-exact result to an independent subagent. The candidate stays
blocked while the investigation runs.

The subagent receives the exact parent and candidate commits plus the saved query
geometry. It must not repair the candidate or assume that either result is
correct. It should:

1. Bypass the optimized traversal.
2. Recompute the disputed collision with slower independent logic.
3. Use `f64` or robust predicates when ordinary `f32` arithmetic is inconclusive.
4. Measure distance from the geometric boundary.
5. Repeat the query with small ULP perturbations.
6. Group repeated mismatches by predicate, hazard type, and numerical signature.
7. Record a representative proof and count for every group.

Classify each difference:

| Classification | Decision |
|---|---|
| Candidate misses a clear collision | Reject |
| Candidate returns free inside the numerical uncertainty band | Reject |
| Candidate removes a clear parent false positive | Acceptable improvement |
| Candidate adds a collision for clearly disjoint geometry | Reject |
| Candidate adds a conservative positive at touching or ambiguous geometry | Acceptable and recorded |
| Investigation cannot decide | Reject unattended |

Ambiguity resolves toward collision. Clearly separated geometry should remain
free. A widespread or unexplained behavior change is rejected.

Do not build a second full collision engine preemptively. Add slower independent
geometry only when a real mismatch needs adjudication.

## Discovery loop

Always start from the latest accepted commit.

On macOS, launch profiled binaries from a scratch directory outside protected
folders such as `Documents`. Copy the fixed benchmark inputs into that directory
with the same relative layout first. This prevents Instruments or the benchmark
process from opening interactive file-access permission dialogs during an
unattended run.

1. Profile the optimized `ci_bench` workload with symbols and frame pointers.
2. Inspect inclusive call paths, then leaf arithmetic, branching, allocation,
   cloning, dropping, memory movement, and traversal.
3. Add temporary `#[inline(never)]` markers only when inlining hides useful phase
   boundaries. Remove them before measuring a candidate.
4. Form one narrow hypothesis from the profile.
5. Implement the smallest isolated experiment with `$jeroens-rust`.
6. Run the cheapest relevant compile check.
7. Compare the targeted Criterion cases sequentially with the immediate parent.
8. Reject a clear loss immediately and record it.
9. Reject added complexity when the result is flat.
10. Send a promising candidate to the acceptance lane.

Do not stack experiments on an unaccepted candidate. Re-profile after every
accepted change because the hot path may have moved.

## Performance decision rule

Run the complete `ci_bench` suite before accepting a candidate. A targeted gain
must not hide a meaningful regression in another query or update workload.

Use this as a judgment guide, not a substitute for Criterion's interval and
significance result:

| Result | Default decision |
|---|---|
| Clear regression during warm-up | Stop and reject |
| Flat with added complexity | Reject |
| Flat with meaningful code deletion | May proceed to correctness checks |
| Less than 1% faster | Reject unless simpler |
| 1% to 3% faster | Keep only when the diff is tiny and local |
| Hundreds of added lines for about 2% | Reject |
| Larger reproducible gain | Proceed if correctness and readability hold |

Do not average unrelated benchmark cases into a flattering aggregate. Report
depth 4 first, followed by the worst guardrail result elsewhere in `ci_bench`.

## Acceptance lane

Snapshot a promising candidate as an exact temporary commit. Give an independent
subagent a separate worktree containing only that candidate and its parent.

The subagent must:

1. Audit callers, semantics, floating-point behavior, invariants, and mutation
   paths.
2. Run the fixed parent and candidate query corpus.
3. Run depth-zero and fresh-CDE comparisons.
4. Investigate every result-set difference.
5. Exercise debug assertions on a short representative run.
6. Repeat the relevant parent and candidate Criterion runs in isolation.
7. Run formatting, Clippy, all-feature and no-feature builds, tests, and the
   README example.
8. Return a pass or a precise rejection without quietly changing the experiment.

A correctness repair is a new experiment. It returns to the discovery benchmark
gate.

The primary agent may profile or prepare a separate hypothesis during acceptance,
but final performance runs must not overlap. Keep one writer for shared ledgers
and the pull request.

## Integrate or discard

If acceptance fails:

1. Drop the candidate.
2. Record its parent, evidence, diff, measurements, behavior result, decision,
   and lesson.
3. Return to the accepted head.

If acceptance passes and Git actions are authorized:

1. Integrate one focused commit.
2. Re-run the complete correctness oracle and `ci_bench` suite.
3. Push the accepted commit.
4. Update the pull request.
5. Verify CI on the exact pushed revision.
6. Make that commit the new accepted head.
7. Re-profile before choosing the next experiment.

## Pull request bookkeeping

Keep the pull request useful to someone returning after several hours. Its
description should contain:

- Frozen baseline and environment
- Verification method
- Accepted-change overview
- Compact rejected-experiment overview
- Current cumulative throughput checkpoint
- Behavior status and mismatch classifications
- One short numbered section per accepted change

For every experiment, record:

- Exact parent commit
- Profiler evidence and hypothesis
- Files or representation boundary changed
- Immediate-parent Criterion result and interval
- Worst `ci_bench` guardrail result
- Exact or changed behavior result
- Mismatch investigation result when applicable
- Independent subagent verdict
- Decision and lesson

Keep raw traces and verbose rejected notes in `.agent-notes`. Archive the final
rejected ledger in one pull-request comment and link it from the description.

## Stop and wrap up

At the stated deadline:

1. Start no new experiment.
2. Let acceptance checks already in flight finish, or reject them if they cannot
   finish safely.
3. Integrate only candidates that passed every gate against the correct parent.
4. Remove temporary no-inline markers, profiling files, traces, and disposable
   worktrees without touching user-owned changes.
5. Run the final build, tests, correctness oracle, and `ci_bench` suite.
6. Push and verify the exact final revision only when authorized.
7. Reconcile the pull request, experiment ledger, backlog, and accepted head.
8. Report the final commit, cumulative result, behavior status, CI state,
   rejected count, and remaining profiler-backed ideas.

Do not extend the deadline because one more idea looks easy.
