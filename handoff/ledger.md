# Quadtree speedup ledger (jagua-rs, branch `quadtree-layout`)

## Working rules (from the user, re-read before every step)
- Only collision **collection** matters: bench `cde_collect_1k/4` only (depth 4). Ignore detect/update/other depths.
- Fail fast: implement → compile → bench immediately. Not faster → toss, no correctness work. Only winners get the equivalence harness + `cargo test`, then a commit.
- Free to think out of the box and change a lot, but deliver it as small, separately reviewable steps. No giant diffs.
- Every kept step: one commit, pushed, with a ledger entry: what changed, numbers, and **IDENTICAL** (bit-identical harness output) or **EQUIVALENT** (differences only conservative: extra false collisions, never a missed one).
- Every tossed idea: one ledger line with its number.
- Don't waste time on broad benchmarks.

## Setup
- Baseline: `main` @ 3becd0c (worktree /home/user/jagua-rs-base).
- Equivalence harness: scratchpad/eqh (variant) and eqh-base (baseline); ~102k query lines, depths 3–5, with moves, save/restore, width changes, filters.
- A/B: scratchpad/ab.sh <label> <rounds> cde_collect_1k/4 (alternating base/variant).

## Baseline
- run 1 (full ci_bench): cde_collect_1k/4 = 3,429 µs/iter (1000 queries)

- A/A noise (4 alternating rounds, collect/4): base 3521±61 µs vs identical variant 3556±75 µs (+1.0%). Run-to-run sd ≈ 2%; require ≳4% improvement to count.

## Profile (cachegrind, base, collect/4, 20k queries, setup subtracted)
- ~23.4k instructions and ~210 branch mispredicts per query; D1 miss rate ≈0.4% → not memory-bound, instruction count + branches.
- `collect_collisions_until` (leaf scan + recursion): 32% Ir, 50% mispredicts (collector `contains_key` alone 22% of mispredicts).
- `Edge::collides_with_quadrants`: 29% Ir, 22% mispredicts.
- `detect_containment_collision`: 7% Ir. Leaf edge–edge tests: ~1.3% Ir → SIMD leaf edge tests ruled out.

## Steps
| # | Change | collect/4 base → var | Output | Commit | Kept |
|---|--------|----------------------|--------|--------|------|
| t1 | Branchless `Edge::collides_with_quadrants` prefilter (shared column/row range tests, `&`/`|`) | 3469 → 4181 µs (+20.5%) | – | – | tossed |
| t2 | Leaf loop: test partial hazard ff_bbox before collector `contains_key` | 3523 → 3807 µs (+8.1%) | – | – | tossed |
| t3 | Skip internal node if all its hazards are already collected/filtered | 3524 → 3636 µs (+3.2%) | – | – | tossed |
| t4 | Fast path in `Edge::collides_with_quadrants` for edges strictly inside one quadrant | 3535 → 3540 µs (+0.2%) | – | – | tossed (neutral) |

### Traversal statistics (temporary counters, base, collect/4, per query; patch in scratchpad/instrumentation.patch)
19.2 shape edges · 27.9 internal-node classifications (`collides_with_quadrants`) · 19.7 leaf scans · 145.9 leaf hazards (52.8 already collected, 83.3 rejected by ff_bbox, ≤49 edge tests) · 0.8 insertions · 30.6 containment checks · surrogate: 4.4 internal + 2.9 leaf visits, 21.5 leaf hazards.
→ `collides_with_quadrants` costs ~246 instructions per call; `[0,1,2,3].map(..)` → `[Option<bool>;4]` goes through non-inlined `array::Drain` code.
| t5 | `collides_with_quadrants`: per-quadrant closure calls instead of `[0,1,2,3].map` | 3536 → 3501 µs (−1.0%) | – | – | tossed (within noise) |
- Distinct leaves per query: 2.1 (vs 19.7 leaf scans) → each leaf is rescanned ~9× because the tree is queried edge by edge.

## Phase 2: hazard-first prototype (branch `hazard-first-proto`)
| # | Change | collect/4 base → var | Output | Commit | Kept |
|---|--------|----------------------|--------|--------|------|
| p1 | Hazard-first collect: SoA bbox scan of all interior hazards + per-candidate bbox-culled edge pairs + one containment check; exterior handled explicitly; surrogate poles: edge distance + center-in-polygon when the pole's bbox lies inside the hazard's | 3526 → 2598 µs (−26.3%) | EQUIVALENT | cd03aae | prototype |

p1 equivalence (eqh, ~102k queries, depths 3–5, moves/save/restore/width changes/filters), checked against an f64 oracle (exact segment intersection + containment):
- missed collisions: 0. False positives: 0 in both engines.
- final collect (`a=`): 331 extra hazards on 325 lines, **all real collisions** that the quadtree collect misses (e.g. query 1971: base collect empty while base detect reports a collision).
- surrogate pass (`s=`): extra hazards on 11,338 lines, mostly the exterior hazard (poles outside the container) and poles lying inside a hazard; all real collisions.
- `cargo test --locked` passes (test profile has debug assertions).
| q1 | 64-wide branch-free bbox mask scan; surrogate pass scans once with the poles' union bbox, then per pole over candidates | 3497 → 1803 µs (−48.4% vs main; p1 was 2598) | IDENTICAL to p1 | 8da149f | ✅ |
| q2 | Cohen–Sutherland outcodes over vertex arrays for edge culling (pairs and circles), stack buffers for both sides | 3567 → 1556 µs (−56.4% vs main; q1 was 1803) | IDENTICAL to p1 | 9d736c1 | ✅ |
| q3 | Edge buffers as u16 vertex indices (avoid 2 KB memset per pair) | 3538 → 1546 µs (q2: 1556; −0.6%) | – | – | tossed (neutral) |
| q4 | Surrogate pass: drop the pole-inside-hazard point-in-polygon check | 3508 → 1524 µs (q2: 1556; −2%) | (s= weaker) | – | tossed (below bar) |
| q5 | Select-based (branchless) circle–edge squared distance in the pole test | 3520 → 1557 µs (q2: 1556) | – | – | tossed (neutral) |

Prototype counters at q2 (per query; scratchpad/proto_instrumentation.patch): 4.1 poly candidates → 2.55 narrow-phase pairs (16.8 query edges × 6.6 hazard edges surviving the cull, 24.9 edge-pair tests) · 2.46 containment checks (0.63 point-in-polygon) · 2.05 ff poles → 3.15 pole candidates, 4.9 circle–hazard edge scans over 67.7 vertices, 0.08 pole point-in-polygon. Culling loops dominate now.
| q6 | Per-hazard edge bounding boxes cached in flat arrays at registration; hazard-edge and pole culling via 64-wide masks | 3504 → 1390 µs (−60.3% vs main; q2 was 1556) | IDENTICAL to p1 | de38d66 | ✅ |
| q7 | Skip exterior tests (edges, containment, point-in-polygon) for shapes/poles strictly inside a rectangular container | 3484 → 1175 µs (−66.3% vs main; q6 was 1390) | IDENTICAL to p1 | ddcaa50 | ✅ |
| q8 | Pad box arrays to multiples of 16 and build masks in fixed 16-lane chunks | 3559 → 1322 µs (q7: 1175; +12.5%) | – | – | tossed (slower) |
| q9 | Query-shape edge boxes on the stack once per query; per hazard edge, mask query edges by edge-bbox overlap (replaces per-pair outcode loop + buffer) | 3529 → 1147 µs (q7: 1175; −2.4%) | – | – | tossed (below 4% bar; likely a small real gain) |
- Floor (both collect calls stubbed): 141 µs → CDE-only time: main ≈ 3400 µs, q7 ≈ 1034 µs (3.3×).
| q10 | Mask building in blocks of 8 flag bytes packed by multiply | 3505 → 1543 µs (q7: 1175) | – | – | tossed (slower) |
- Scaling (prof driver, µs/query incl. transform; quadtree d4/d5[/d6] vs proto q7): swim(48) 4.54/4.17 vs 1.21 · trousers(64) 1.95/1.93 vs 0.54 · shirts(99) 3.10/2.99 vs 0.79 · gardeyn3(100) 5.65/5.17 vs 1.03 · gardeyn6(161) 4.56/4.57 vs 1.14 · gardeyn7(160) 5.23/5.38 vs 1.18 · gardeyn7×4(640) 5.04/4.69/4.38 vs 1.80 · gardeyn7×8(1280) 4.91/4.59/4.20 vs 3.03 (linear scan starts to show).
| q11 | Hazard bboxes sorted by x_min; binary-search the query x-window (skipped when ≤64 hazards) | swim 3523 → 1214 µs (q7: 1175, +3% ≈ noise); 1280 items 3.03 → 1.71 µs/query, 640 items 1.80 → 1.23 | IDENTICAL to p1 on swim; g7×4: 0 missed, 1 extra real hit | 9484dd3 | ✅ (scalability) |
| q12 | Retry of q9 on top of q11 (stack query edge boxes; pair culling by edge-box overlap), 5 rounds | 3501 → 1173 µs (q11: 1214; −3.4%, sd 0.6%) | – | – | tossed (under 4% bar; likely real — revisit on real hardware) |
| q13 | Pole–pole quick positive test (query ff poles vs hazard ff poles) before edge scans, both passes | 3583 → 1243 µs (q11: 1214; +2.4%) | – | – | tossed (slower) |
| q14 | Store entity + shape Arc + edge boxes in the index (no hazards_map lookups in the hot loop) | 3596 → 1204 µs (q11: 1214) | – | – | tossed (neutral) |

### End-to-end: Sparrow Pro runner (local uncommitted [patch] copies in scratchpad; base = jagua-rs main 3becd0c, proto = 9484dd3)
3 seeds single-threaded in parallel (4 vCPUs), 60 s exploration, `--workers 1`. Evals/s per seed:
- swim: base 284 / 281 / 288 (mean 284) → proto 331 / 336 / 342 (mean 336): **+18%**. Widths: seeds 1–2 identical layouts (5987.8, 6005.1); seed 3 6000.7 → 5964.8.
- gardeyn7: base 197 / 199 / 199 (mean 198) → proto 225 / 233 / 248 (mean 235): **+19%**. Widths 5544/5519/5549 → 5589/5547/5477 (independent samples; too short to judge quality).
| q15 | Detect queries (poly + surrogate poles/piers) from the hazard index; fix: shape bbox disjoint from the container bbox now collides with the exterior in polygon queries | detect/4: 280 → 185 µs (−34%); collect/4 1230 µs (unchanged) | EQUIVALENT (oracle: 0 missed by detect, 0 false positives) | 760a1ea | ✅ |
- E2E with q15 (detect via index too), evals/s: swim 322/317/326 (q11: 331/336/342), gardeyn7 203/209/223 (q11: 225/233/248) → detect via index is slower inside Sparrow than the quadtree detect; investigating.
| q16 | Detect back on the quadtree (end-to-end evidence below); keep the collect exterior fix from q15 | collect/4 unchanged (~1214 µs) | EQUIVALENT (oracle: 0 missed, 0 FP, 331 extra real) | f84524d | ✅ |

### End-to-end, same batch (B3), evals/s per seed in thousands (3 seeds × 60 s, 1 worker each, 3 in parallel)
| variant | swim | gardeyn7 |
|---|---|---|
| main | 286 / 287 / 294 (mean 289K) | 193 / 195 / 193 (194K) |
| q11 collect via index | 329 / 331 / 330 (330K, **+14%**) | 220 / 230 / 244 (231K, **+19%**) |
| q15 + detect via index | 329 / 326 / 336 (330K, +14%) | 207 / 218 / 228 (218K, +12%) |
| + only detect_poly via index | 334 / 334 / 334 (334K, +16%) | 211 / 219 / 230 (220K, +13%) |
Batches drift ~5% (main gardeyn7: 198 → 189 → 194K across batches); only same-batch numbers are compared.

## Phase 3 (fastest prototype; surrogate config; quantification) — instructions_phase3.md

### Production build (`--features simd`, nightly) — earlier end-to-end runs were scalar builds
Same batch S1, evals/s per thread (seeds 1–3, 60 s): swim main 308/308/306K → proto q16 430/428/433K (**+40%**); gardeyn7 main 206/196/222K → proto 269/268/279K (**+31%**).

### Quantification (sparrow-pro branch `quantify-proto`; microbench scratchpad/spq/qbench: 2000 overlapping swim pairs, avg 237 pole pairs, best of 7)
Baseline SIMD quantification ≈ 398 ns (411.6 / 384.9), scalar ≈ 914 ns.
| # | Change | quantify ns (base → var) | e2e evals/s (same batch) | Output | Commit | Kept |
|---|--------|---------------------------|--------------------------|--------|--------|------|
| Q2 | Pad pole SoA to whole SIMD chunks with zero-radius poles; widths 4 / 8 / 16 | 325 → 325 / 313 / 417 | – | EQUIVALENT | – | tossed (neutral; 16 slower) |
| Q1 | SIMD kernel: accumulate lanes, one `reduce_sum` per hazard pole (was per chunk) | ≈398 → ≈325 (−18%) | gardeyn7 +7%, swim −1% (paths diverge; noisy) | EQUIVALENT (summation order; rel. err ~1e-7) | e9d1f59 | ✅ |
| Q3 | APPROXIMATE: quantify only hazard poles reaching the shape bbox + margin k·ε (outer loop) | colliding pairs: 395 → 313 (k=10, −21%) / 322 (k=2) / 301 (k=0, −24%) | – | APPROXIMATE: Spearman all 0.9993 / 0.9988 / 0.9986; pairs without overlapping poles (6%) 0.96 / 0.91 / 0.86 | ac90f58 (k=10) | ✅ APPROXIMATE |
| Q4 | APPROXIMATE: prune both sides (query poles near the near-hazard-poles bbox, stack buffers) | 515–567 (slower) | – | – | – | tossed |

### Surrogate config sweep (end to end on Q1+Q3, batch C1, evals/s per thread, seeds 1–3, 60 s; widths indicative)
| config | swim | gardeyn7 |
|---|---|---|
| ref `[(64,0),(16,.8),(8,.9)]`, ff 0.5, piers 0 | 413/405/414 (411K) w 5893/5981/5959 | 281/271/288 (280K) w 5435/5437/5434 |
| A `[(32,0),(12,.8),(6,.9)]` | 467/470/469 (469K, +14%) w 5994/6059/6025 | 286/292/300 (293K, +5%) w 5440/5519/5484 |
| B `[(24,0),(8,.75),(4,.85)]` | 503/510/491 (501K, +22%) w 5869/6059/5995 | 333/335/335 (334K, +19%) w 5429/5413/5438 |
| F3 ff ratio 0.3 | 431/428/433 (431K, +5%) | 293/288/303 (295K, +5%) |
| F7 ff ratio 0.7 | 408/403/403 (405K, −1%) | 282/273/287 (281K, 0%) |
| P1 1 ff pier | 424/422/423 (423K, +3%) | 295/287/306 (296K, +6%) |
| Q5 | APPROXIMATE: two-sided pruning with reusable evaluator buffers (no zeroing) | 330 → 640 ns (slower: pruned sets rarely fill SIMD chunks) | – | – | – | tossed |

### Surrogate config combinations (batch C2, on Q1+Q3)
| config | swim | gardeyn7 |
|---|---|---|
| ref | 432/423/431 (429K) w 5893/5981/5959 | 269/265/274 (269K) w 5435/5437/5434 |
| B | 492/505/494 (497K, +16%) w 5869/6059/5995 | 334/337/335 (335K, +25%) w 5429/5413/5438 |
| B+F3 | 492/495/489 (492K, +15%) w 5869/5969/5923 | 330/338/338 (335K, +25%) w 5435/5430/5433 |
| **B+F3+P1** | 499/517/505 (507K, **+18%**) w 5869/5969/5923 | 338/336/339 (338K, **+26%**) w 5435/5430/5433 |
| C `[(16,0),(6,.7),(3,.8)]` | 529/529/523 (527K, +23%) w 5976/5993/5923 | 321/332/338 (330K, +23%) w 5486/5435/5473 (worse) |
| C+F3 | 548/550/544 (547K, +28%) w 6030/6102/5995 (worse) | 325/335/341 (334K, +24%) w 5437/5501/5473 (worse) |
Recommendation: B+F3+P1 = `n_pole_limits: [(24, 0.0), (8, 0.75), (4, 0.85)]`, `ff_pole_area_ratio: 0.3`, `n_ff_piers: 1`. C variants look too coarse (widths worse). Quality needs proper long runs.

Committed on sparrow-pro `quantify-proto` (pushed): e9d1f59 Q1 · 44ef9d6 surrogate config B+F3+P1 · ac90f58 Q3 (k=10). Tests: 39 passed with and without simd.

### Final end-to-end, same batch F1 (SIMD builds, evals/s per thread, seeds 1–3, 60 s exploration)
| instance | main (both repos at main) | q16 (jagua-rs hazard-first collect only) | final (q16 + quantify-proto: Q1, Q3, config B+F3+P1) |
|---|---|---|---|
| swim | 305/306/308 (306K) w 6030/6005/5935 | 424/421/423 (423K, +38%) | **514/526/512 (517K, +69%)** w 5869/5969/5923 |
| gardeyn7 | 205/194/222 (207K) w 5556/5437/5439 | 260/264/277 (267K, +29%) | **341/343/349 (344K, +66%)** w 5435/5430/5433 |
| gardeyn8 | 240/245/243 (243K) w 107355/107241/106664 | 316/311/317 (315K, +30%) | **384/389/384 (386K, +59%)** w 106052/106973/105663 |

## Summary (night of 13–14 October 2026)

### Where the time went
- jagua-rs quadtree collection was bound by repetition and branches, not memory: a query's ~19 edges each walked the tree, rescanning the same ~2 leaves ~9× (146 hazard checks per query where ~15 suffice). Five layout/traversal tweaks (t1–t5) were neutral or slower.
- In Sparrow Pro's exploration (SIMD build, setup subtracted), quantification was the largest single cost (~25%), then collision collection (~20%), sample evaluation overhead, transforms, allocations.

### What was built
**jagua-rs `hazard-first-proto`** (public API unchanged), commits in order:
cd03aae p1 hazard-first collection · 8da149f q1 64-wide mask scan · 9d736c1 q2 outcode culling · de38d66 q6 cached edge boxes · ddcaa50 q7 rectangular-container shortcut · 9484dd3 q11 sorted x-window (scalability) · 760a1ea q15 detect via index + exterior fix · f84524d q16 detect back on the quadtree (Sparrow end-to-end), exterior fix kept · 3fd57f7 q17+q18 fast exterior tests for irregular containers (cached exterior edge boxes; point-in-exterior via the quadtree).
- Collection: broad phase = hazard bboxes in flat arrays sorted by x_min, 64-wide branch-free masks; narrow phase per candidate = cached edge boxes, outcode culling, one containment check. Detection stays on the quadtree (faster inside Sparrow: deep pole collisions are proved by whole-node presence).
- `cde_collect_1k/4`: ~3500 → ~1210 µs (2.9×; collision work only ~3400 → ~1050 µs, 3.3×). Scales: 1280 items 1.7 µs/query vs quadtree 4.2–4.9.
- EQUIVALENT: across ~100k harness queries checked with an f64 oracle, 0 missed collisions, 0 false positives; it finds 331 real collisions that main's quadtree collection misses.

**sparrow-pro `quantify-proto`** (on top of the jagua-rs prototype via a local patch): e9d1f59 Q1 SIMD reduce once per pole (EQUIVALENT) · 44ef9d6 surrogate config `[(24,0),(8,.75),(4,.85)]`, ff ratio 0.3, 1 ff pier · ac90f58 Q3 quantify only hazard poles near the shape (APPROXIMATE; Spearman 0.9993 vs exact).

### End to end (final batch F2, SIMD builds, evals/s per thread, seeds 1–3, 60 s; main = both repos at main, final = jagua-rs 3fd57f7 + sparrow-pro quantify-proto ac90f58)
| instance | main | final | change |
|---|---|---|---|
| swim | 302/297/297 (299K) | 501/497/500 (499K) | **+67%** |
| gardeyn7 | 204/197/221 (207K) | 341/342/345 (343K) | **+65%** |
| gardeyn8 | 251/255/252 (253K) | 389/390/394 (391K) | **+55%** |
| baldacci1 (irregular bins) | 244/245/242 (244K) | 352/364/354 (357K) | **+46%** |
| frames | 739/790/748 (759K) | 766/808/741 (772K) | +2% (neutral) |
The jagua-rs collection prototype alone gives +29–38% on the strip instances (batch F1). 300 s full runs (batch L1) reached higher final densities with the final build on swim, gardeyn7 and gardeyn8 (3 seeds, indicative).

### What to re-measure on real hardware / test properly
1. `cde_collect_1k/4` on the M3 Max (prompt drafted in chat), and q12 (−3.4%, tossed under the 4% bar here).
2. Solution quality of the new surrogate config and the approximate quantification with long runs over many seeds and instances (this VM only allowed short indicative runs; see the long-run batch below).
3. Hazard registration cost: the index builds edge boxes and the quadtree is still maintained (only detection uses it); a detection that beats the quadtree inside Sparrow would let the quadtree go.
4. The quadtree's collection code is now dead code in the prototype.
5. Irregular containers: now checked with a bin-packing harness (scratchpad/eqh/src/bin/bpp.rs, jagua-rs baldacci1, 8 LBF layouts, 60k queries at depths 3–5, f64 oracle over every hazard): collected sets IDENTICAL to main, 0 missed, 0 false positives, detect misses nothing. Holes/defects in bins are still untested.
6. frames (very fast already) gains nothing; check what dominates there.
7. ⚠ baldacci1 (irregular bins) ended less dense in 300 s runs (66.3% vs 67.4%, 3 seeds), also with the jagua-rs prototype alone. Possibly noise; test with many seeds on bin instances first.

### Long runs (batch L1): final density after 300 s full runs (exploration + compression), seeds 1–3; evals/s per thread in parentheses
| instance | main | final |
|---|---|---|
| swim | 75.99 / 74.42 / 75.29% (mean 75.23%; ~311K) | 76.94 / 75.20 / 75.59% (**75.91%**; ~493K) |
| gardeyn7 | 80.20 / 81.18 / 81.17% (80.85%; ~212K) | 82.40 / 82.94 / 82.18% (**82.51%**; ~361K) |
| gardeyn8 | 83.93 / 85.14 / 84.53% (84.53%; ~267K) | 85.14 / 85.43 / 86.75% (**85.77%**; ~401K) |
Indicative only (3 seeds, one VM), but every instance's mean improves at equal time.

### Irregular containers (bin packing, baldacci1) — regression found and fixed
Batch B1–B4, evals/s per thread (seeds 1–3, 60 s, SIMD):
- main 242/242/241K; final (q16 + quantify-proto) 206/212/203K (−15%); q16 alone 155/154/153K vs main 239/239/234K (−35%): the exterior (irregular sheet outline) was tested with all its vertices per query and per pole, plus point-in-polygon.
- frames: main 682/716/684K → final 756/793/730K (+9%).
| q17 | Cache the exterior's edge boxes; masked culling like other hazards | baldacci1 final 180K → 205K (+14%) | EQUIVALENT | 3fd57f7 (with q18) | ✅ |
| q18 | Point-in-exterior via the quadtree's exterior presence in the node containing the point; point-in-polygon only in partial leaves | baldacci1 205K → 347K (main 238K: **+46%**) | EQUIVALENT on swim (oracle unchanged); irregular containers not covered by the harness | 3fd57f7 | ✅ |

### Bin-packing harness (after q17/q18, jagua-rs 3fd57f7)
baldacci1 (jagua-rs asset, irregular bins), 60k queries: main and proto both collect 189,114 real collisions, 0 false positives, 0 missed (oracle over all hazards), detect never misses; collected sets identical per query.

### Bins with holes/defects (batch H1, evals/s per thread, seeds 1–3, 60 s)
- baldacci1_no_holes_mixed_holes: main 265/266/266K (w 44064/43820/43005) → final 357/362/363K (**+36%**, w 40005/40692/43357).
- baldacci1_no_holes_mixed_holes_solid: main 301/289/294K (w 48546/47632/48546) → final 420/409/413K (**+40%**, w 48015/47871/48063). No errors or panics in the logs.

### ⚠ Quality on irregular bins (batch L2, 300 s full runs, seeds 1–3)
baldacci1: main 66.50 / 68.03 / 67.73% (mean 67.42%, ~253K) vs final 65.84 / 67.56 / 65.42% (mean **66.27%**, ~356K). Faster, but less dense: unlike the strip instances, the final build loses quality here.
- Control (batch L3): jagua-rs 3fd57f7 with sparrow-pro main (exact quantification, old surrogate config): 65.74 / 65.62 / 67.34% (mean 66.23%, ~269K). So the drop does not come from quantify-proto. Either the collision changes (extra real collisions found; exterior presence from the quadtree) change the search on irregular bins, or it is seed noise (spread ~1 point per variant; 3 seeds cannot separate a 1.2-point difference). **Open: run many seeds on baldacci1 and the other bin instances before adopting.**

### Quality status (user feedback, 14 Oct): UNVALIDATED
All density/width comparisons above use 3 seeds per variant and are not evidence either way, including "denser on strips" and the baldacci1 drop.
The new pole limits change the quantification itself, so their quality effect must be measured separately.
Protocol: per instance, ≥10–20 seeds per variant at the same fixed time and parallelism, final density compared as independent groups (mean ± SE, permutation test).
Variants: main; jagua-rs prototype only; + Q1 (old config); + Q3; + new pole limits. Scripts: scratchpad/quality/run.sh and analyze.py.
Prepared binaries in scratchpad/cfgbin (SIMD, native): main (both repos main) · jagua_only (jagua-rs 3fd57f7, sparrow-pro main) · q1_oldcfg (+Q1 e9d1f59) · q1q3_oldcfg (+Q3, old pole limits) · final18 (+ new pole limits, quantify-proto ac90f58).
Smoke-tested: run.sh (5 s, 2 seeds) + analyze.py work. Example full run on this VM (3 in parallel, 20 seeds, 300 s):
  quality/run.sh quality/out 300 3 1 20 <instances...> -- main=cfgbin/main jagua_only=cfgbin/jagua_only q1=cfgbin/q1_oldcfg q1q3=cfgbin/q1q3_oldcfg final=cfgbin/final18
  python3 quality/analyze.py quality/out main
- Started (04:53 UTC): baldacci1, seeds 1–10, 300 s, 3 parallel, variants main/jagua_only/q1/q1q3/final → quality/out_baldacci1, result in quality/result_baldacci1.txt (~85 min).

### Design notes for the real version (user, 14 Oct)
- Keep a quadtree-like structure, but split static from dynamic: container hazards (exterior, holes, defects) are static per bin, so their structure is built once (inside/outside/boundary cells, boundary cells holding their edges) with no update logic.
- Open choice for placed items: (1) keep a dynamic quadtree only for item detection, or (2) move item detection to the hazard index with a cheap "pole deep inside an item" proof (item poles first, point-in-polygon fallback), removing the dynamic quadtree. Option 2 is simplest if it matches the quadtree inside Sparrow.

### Correction (14 Oct): the q16 rationale was wrong
Sparrow Pro calls detection only in `CollisionQuery::is_free` (collision.rs), used by `LBFEvaluator` in the initial LBF construction (optimizer/lbf.rs) and the placeability check (optimizer/placeability.rs). Exploration (the separator) never detects; it only collects and quantifies. Exploration evals/s therefore cannot measure detection speed. The q15 vs q11 difference most likely came from changed detect results altering the initial LBF layout and with it the search path. The "deep poles inside parts" explanation was wrong.
Consequence: no measured reason to keep movable items in the quadtree. Compare detection variants by timing LBF construction and the placeability check, not exploration throughput.

### Quality, statistical (baldacci1, 10 seeds per variant, 300 s full runs, 3 in parallel, interleaved)
| variant | density mean ± SE | Δ vs main | permutation p |
|---|---|---|---|
| main | 66.748% ± 0.288 | | |
| jagua_only (3fd57f7) | 66.679% ± 0.301 | −0.069 | 0.87 |
| q1 (old pole limits) | 66.455% ± 0.230 | −0.293 | 0.44 |
| q1q3 (old pole limits) | 66.657% ± 0.152 | −0.091 | 0.79 |
| final (new pole limits) | 67.049% ± 0.320 | +0.301 | 0.49 |
No significant difference; the 3-seed "drop" was noise. Detectable difference at this n ≈ 0.8 pts, so small effects are not excluded. Strip instances still to test.

### Phase 4: exterior as y-monotone edge chains (uncommitted), batch K1 (evals/s per thread, seeds 1–3, 60 s; whole batch ~18% slower than earlier batches: host load)
| instance | main | q18 (quadtree point-in-exterior) | chains |
|---|---|---|---|
| baldacci1 | 202/200/196 (199K) | 275/276/272 (274K) | 269/274/270 (271K, −1%) |
| baldacci1 mixed holes | 233/231/230 (231K) | 314/324/324 (321K) | 317/318/320 (318K, −1%) |
| swim | 269/262/264 (265K) | 426/427/422 (425K) | 407/405/400 (404K, −5%) |
Bin-packing oracle harness: chains collect exactly the same hazards as main (0 FP, 0 missed). Chains replace the quadtree for the exterior at no cost on bins; swim −5% to investigate.
- First chain version iterated edges through vertex indices with modulo: swim −3–4% vs q18 (two reruns, identical widths). Contiguous edges per chain fixed it (batch K3): swim q18 395/405/393 → chains 396/409/403 (even); baldacci1 267/277/270 → 293/291/282 (+6%).
| p4-1 | Exterior edges in y-monotone chains (≤16 edges, contiguous, bbox per chain); point-in-exterior by chain ray casting instead of the quadtree | baldacci1 +6% vs q18, swim even | bpp harness identical to main; swim harness unchanged | 739c61b | ✅ |

### User's M1 swim races (14 Oct; 5 rounds × 300 s, permutation tests; results in ~/bench/swim_race_* on the M1)
- jagua-only (hazard-first-proto 3fd57f7 on pro ae30161): 1.22× evals/s, every run faster; density −0.03 pts, p=0.91 → equal.
- quant (quantify-proto ac90f58): 1.13× evals/s; density −0.33 pts, p=0.17; all 5 runs ≤ 77.60% (pro: 3 of 5 ≥ 77.9%).
- quantpl (quant with old n_pole_limits): 0.98× evals/s, density equal.
Conclusion: the quantify-proto speedup comes from the smaller pole limits and likely costs quality → dropped.
- Commit 5404eb7 on quantify-proto restores all three old surrogate defaults; the branch now isolates Q1 (SIMD reduction) + Q3 (approximate pole pruning) on top of ae30161, for the next race against pro.
- Note: the interrupted surrogate-off experiment had partly edited collision.rs in the scratch worktree; it was never committed and has been reverted.

### Current branch state
- jagua-rs hazard-first-proto @ 739c61b (3fd57f7 + exterior edge chains, +6% on irregular bins, swim even).
- sparrow-pro quantify-proto @ 5404eb7 (Q1 + Q3 only, original surrogate defaults).

### Phase 5
- Step 1 (batch QO, evals/s per thread, seeds 1–3, 60 s; host busier than earlier): quantify-proto 5404eb7 standalone (jagua-rs 9e00fd8, original surrogate defaults) vs main: swim 259/256/255 → 277/266/275 (+6%); gardeyn7 172/161/181 → 177/174/180 (+3.5%).
- Step 2 (batch SO; build 5404eb7 + jagua-rs 739c61b): Sparrow's surrogate collect pass on vs off (all 3 call sites in collect_until): swim 339/334/339 → 316/317/313 (−7%); gardeyn7 236/231/244 → 232/225/228 (−4%); baldacci1 245/242/241 → 266/262/269 (+9%). Widths differ (paths diverge). Surrogate step helps strips, costs on irregular bins.
- Step 2b (batch SI): surrogate on for items, off for the container collect ("sur_items"): swim on 344/342/342 · off 324/321/321 · items-only 372/374/370 (+8% vs on); baldacci1 on 243/245/240 · off 265/255/266 · items-only 251/248/244 (+2% vs on). Container surrogate screening costs everywhere; item surrogate screening helps strips (+15% vs off) and costs bins (~−5%).
| p5-1 | Sparrow: no surrogate screening for container collisions (collision.rs) | swim +8%, baldacci1 +2% | EQUIVALENT (same collisions; order/early stop may differ) | quantify-proto (see git log) | ✅ |
- Step 3: Sparrow only detects during setup (~1.2 s on baldacci1 incl. IO). The real gain of index detection is dropping quadtree maintenance. Batch NQ (evals/s per thread, seeds 1–3, 60 s; baseline sur_items = quantify-proto 3a82dd6 + jagua-rs 739c61b): swim 345/343/348 → 366/365/365 (+6%); gardeyn7 228/222/236 → 279/270/285 (+22%); baldacci1 256/256/251 → 318/318/312 (+24%). Setup baldacci1 1.14/1.20/1.13 s → 1.04/0.98/1.11 s.
| p5-2 | Detect on the hazard index; quadtree no longer maintained (code kept, unused) | +6% / +22% / +24% (swim / gardeyn7 / baldacci1) | swim: detect never misses, no FP; bpp: identical to main incl. detect | 0616a23 | ✅ |
- Step 5 (batch IC): y-monotone chains cached per hazard; point-in-polygon for item containment and pole-in-item via chains: swim 383/375/375 → 392/384/388 (+2.7%), gardeyn7 281/271/284 → 288/280/291 (+2.6%), baldacci1 306/308/303 → 304/295/302 (−1.5%). Below the 4% bar (registration now also builds chains per move) → tossed; candidate for the real version.

### Current stack vs main (14 Oct, host busier than earlier; ratios only)
- jagua-rs `cde_collect_1k/4`, head 0616a23 vs main (3 alternating rounds): 3836 ± 233 → 1296 ± 62 µs (**3.0×**).
- Sparrow end to end (batch CU, evals/s per thread, seeds 1–3, 60 s); current = jagua-rs 0616a23 + sparrow-pro quantify-proto 3a82dd6 (original pole limits):
| instance | main | jagua-rs 3fd57f7 only | current |
|---|---|---|---|
| swim | 253/245/252 (250K) | 342/337/335 (338K, +35%) | 369/364/365 (366K, **+46%**) |
| gardeyn7 | 177/166/191 (178K) | 230/233/242 (235K, +32%) | 268/261/270 (266K, **+50%**) |
| gardeyn8 | 212/211/207 (210K) | 254/257/253 (255K, +21%) | 335/346/342 (341K, **+62%**) |
| baldacci1 | 203/200/196 (200K) | 233/231/228 (231K, +16%) | 309/297/309 (305K, **+53%**) |

### Quantification signal test (14 Oct; scratch benches spq/qbench/src/bin/qsignal.rs and qkern.rs; artifact https://claude.ai/artifact/FG25PzjBcHkRWeBFhfAT8Q)
- Test: moving item slightly overlapping 1–3 items (0.2–3% of diameter), 24 perturbed poses, weighted loss sum over colliding pairs; agreement of each variant's best pick with the original proxy. 2000 trials × {near contact, medium, wide} × {swim, shirts, gardeyn7}. Raw output in scratchpad/qsig/*.txt, JSON in qsig/signal.json.
- Near contact, 92–94% of the proxy comes from non-overlapping pole pairs; pairs beyond 10ε carry 0.39–0.46 of the candidate-to-candidate spread. In 59–83% of trials the best pose has no overlapping pole pair. The user confirmed the tail is intended: poles don't cover the whole part, and it spreads the pole sets apart.
- Same-pick rates near contact: C (rsqrt+rcp, 1 Newton) 99.5%; C0 (crude) 93–94%; Q3 (k=10, committed ac90f58) 91% (84–91% across modes); T50 94–95%; B25 84–85%; A10 77%; A1 45–50%.
- Kernel (Xeon VM, f32x4 loop shape, swim, ns per collision, median of 3 rounds of best-of-7): original 289; hinge without sqrt or division 288; crude rsqrt/rcp 298; rsqrt/rcp + Newton 515; original formula with 8 lanes × 2 hazard poles per pass 164 (1.76×, max rel. err 4.8e-7).
- Proposal (awaiting sign-off): revert Q3; 8×2 kernel with zero-radius padding (EQUIVALENT); measure qbench, e2e, M3.
- Step 4 (x-window sizes; counters in an isolated jagua-rs 0616a23 worktree scratchpad/jr-win, prof driver eqh-win, collect depth 4, 20k queries on the LBF layout, counters reset after construction). Per index call:
| instance | hazards | scanned | x-overlap | bbox overlap | µs/query |
|---|---|---|---|---|---|
| swim | 48 | 48 (≤64: no window) | 15.1 | 3.54 | 1.68 |
| shirts | 99 | 26.3 | 20.9 | 2.23 | 1.15 |
| gardeyn7 | 160 | 22.2 | 12.0 | 1.12 | 1.37 |
| gardeyn7 ×4 | 640 | 22.6 | 11.9 | 0.27 | 2.25 |
| gardeyn7 ×8 | 1280 | 22.4 | 12.1 | 0.13 | 2.77 |
  The x-window keeps scans at ~22 hazards (under one 64-wide mask chunk) at any layout size. A second axis would save at most that one branch-free chunk, a few percent of a query. The query time growth with layout size comes from elsewhere (binary search and memory). Decision: no second axis.

### M3 candidates (batch M3, 14 Oct ~09:12 UTC; evals/s per thread, seeds 1–3, 60 s, one batch, order rotated per instance)
All built from branches pinned to jagua-rs hazard-first-proto 0616a23 (no local patch), native SIMD.
| instance | main | m3/q3 (current stack) | exact (Q3 reverted) | exact-k8 | q3-k8 |
|---|---|---|---|---|---|
| swim | 249/241/245 (245K) | 374/363/365 (367K) | 367/366/371 (368K, +0%) | 399/394/396 (396K, **+8%**) | 379/378/385 (381K, +4%) |
| gardeyn7 | 168/161/181 (170K) | 279/267/277 (274K) | 273/273/275 (274K, 0%) | 284/295/311 (297K, **+8%**) | 287/282/295 (288K, +5%) |
| baldacci1 | 204/202/202 (203K) | 292/290/290 (291K) | 297/302/291 (297K, +2%) | 313/315/307 (312K, **+7%**) | 292/290/290 (291K, 0%) |
- qbench (swim colliding pairs, ns/quantification, 3 rounds; noisy): q3 395/383/486, exact 363/412/475, exact-k8 390/291/298, q3-k8 356/358/365.
- Q3 gives no end-to-end speed (its filter costs what it saves). exact-k8 is fastest everywhere and exact. q3-k8 is dominated by exact-k8 (slower and approximate), and exact alone is not faster than the current stack: both dropped (local branches deleted, never pushed).
| p6-1 | 8 lanes, two hazard poles per pass, zero-radius padding of the shape's poles | +7–8% e2e vs current stack | EQUIVALENT (summation order; kernel rel. err ≤ 5e-7); 39 tests pass with debug assertions (SIMD vs sequential proxy on every quantification), with and without simd | 6c9cc8b (m3/exact-k8) | ✅ |
| p6-2 | Revert Q3 (approximate pole cull) | ±0–2% e2e; restores the exact signal (Q3 changed 9–16% of best picks) | IDENTICAL to the pre-Q3 quantification | 3600ebb (m3/exact-k8) | ✅ |
- Pushed: sparrow-pro `m3/q3` (9ab29cd = quantify-proto 3a82dd6 + jagua-rs pin) as the M3 baseline, and `m3/exact-k8` (+ 3600ebb + 6c9cc8b) as the candidate. quantify-proto itself is unchanged.

## Final summary (14 Oct)
**Best configuration:** jagua-rs `hazard-first-proto` 0616a23 + sparrow-pro `m3/exact-k8` 6c9cc8b (builds with `--locked`, jagua-rs pinned).
**Gains vs main (this VM, batch M3, evals/s per thread):** swim 245K → 396K (**+62%**), gardeyn7 170K → 297K (**+75%**), baldacci1 203K → 312K (**+54%**). jagua-rs `cde_collect_1k/4`: 3836 → 1296 µs (**3.0×**).
**What it consists of:**
- jagua-rs (collision collection and detection on a hazard-first index, quadtree no longer maintained): cd03aae, 8da149f, 9d736c1, de38d66, ddcaa50, 9484dd3, 760a1ea, f84524d, 3fd57f7, 739c61b, 0616a23.
- sparrow-pro: e9d1f59 (SIMD reduction once per pole), 3a82dd6 (no surrogate screening for container collisions), 6c9cc8b (two hazard poles per pass, 8 lanes), 9ab29cd (jagua-rs pin). Tried and undone: 44ef9d6 surrogate config (cost quality on the M1, reverted by 5404eb7), ac90f58 Q3 (approximate, no e2e gain, reverted by 3600ebb).
- Quantification is unchanged in value (EQUIVALENT); collision results are EQUIVALENT per the oracle harnesses (no missed collisions; extras are real).
**Re-measure on real hardware (M3):**
1. evals/s: main vs `m3/q3` vs `m3/exact-k8`, 5 seeds × 60 s single-threaded per variant, one batch. The 8-lane kernel runs as two 4-lane halves on NEON, so its gain may differ.
2. Quality: main vs `m3/q3` vs `m3/exact-k8`, many seeds, unpaired permutation tests (adopt exact-k8 if it is not worse).
3. `cde_collect_1k/4` on jagua-rs 0616a23 vs main (prompt drafted earlier); q12 (−3.4% here, below the bar) and item chains (+2.5%) are worth a retry there.
**Not pursued, with evidence:** a second bbox axis (x-window already ~22 hazards at any layout size); compact-support or division-free quantification (no kernel gain, and they cut the long-range tail that steers near-contact decisions).
- quantify-proto now also carries the kept steps: 28e7f3e (Q3 revert, = 3600ebb) and 9ce0478 (8-lane kernel, = 6c9cc8b), pushed; it builds with and without simd. quantify-proto = m3/exact-k8 minus the jagua-rs pin commit.
- Phase 5 status: steps 1–5 done (step 4: measured, no second axis; step 5: item chains tossed at +2.5%).

### Exploration profile of m3/exact-k8 (cachegrind, swim, seed 1, AVX2 build with line tables, 30 s − 1 s exploration diff; scratchpad/cgk8)
Instruction counts (Ir), inlined code attributed to its caller; 3.08 G Ir for ~300K evaluations (~10K Ir per evaluation).
- Collision collection ≈ 50%: edges_intersect_cached 14.2%, circle_touches_edges_cached (surrogate pass) 9.4%, SPolygon point-in-polygon 7.0%, index scans/collector/hazard map inlined in evaluate_sample ≈ 18%, containment detection 1.9%.
- Quantification (CollisionLossEvaluator::add) 18.9%: kernel math, loads and loop ≈ 70% of it (SIMD ops 46%, bounds-checked slicing 11%, loop 13%); per-collision bookkeeping ≈ 25% (hazard dispatch and bound logic 6%, Option/Vec/pointer 10%, GLS weights 2%, penalty 1%, loss bound 1%, SoA load 2%).
- Transforms ≈ 7% (transform_from 4.2%, bounding box 2.4%, compose, sincos). Allocation and copying ≈ 7% (malloc/free/realloc 4.8%, memset 1.4%, memcpy 1.2%).
- Runner output ≈ 3–5%: SVG export of every improvement (string formatting) and container rebuilds when the strip shrinks (SPolygon::new computes the POI).
- Memory: 27 last-level misses in total, ~18 L1 data misses per evaluation: not memory-bound (my earlier memory guess for the quantification overhead was wrong).
- Caveat: instructions, not cycles; sqrt/div and branch mispredicts weigh more in time.

### Profile comparison: 3600ebb (exact, original 4-lane kernel) vs 6c9cc8b (m3/exact-k8); same settings, categories by scratchpad/cg_split.py
| share of exploration instructions | 3600ebb | 6c9cc8b |
|---|---|---|
| collision collection | 44.6% | 50.7% |
| quantification path | 26.7% | 18.9% |
| allocation and copying | 7.9% | 8.3% |
| runner output, container rebuilds | 7.6% | 7.3% |
| transforms | 6.5% | 7.4% |
| layout updates / tracker / sampling / other | 6.9% | 7.3% |
- Per call (callgrind, 10 s runs, exploration callers): CollisionLossEvaluator::add 1,762 → 1,085 Ir (−38%); edges_intersect_cached 1,159 → 1,108; circle_touches_edges_cached 350 → 345 (collection unchanged, as expected).
- The 30 s runs diverged (3600ebb reached 68.5% density with 70 improvements, 6c9cc8b 67.1% with 55), so per-evaluation totals differ by search state; shares and per-call costs are the comparable numbers.

### Surrogate and allocation variants of m3/exact-k8 (batch SUR, 14 Oct ~12:25 UTC; only_final_svg from now on; evals/s per thread, 3 seeds × 60 s)
| variant | swim | gardeyn7 | baldacci1 |
|---|---|---|---|
| s_base (6c9cc8b) | 401/395/405 (400K) | 299/314/333 (315K, noisy) | 312/313/310 (312K) |
| s_sur_off (no surrogate pass against items) | 372K (−7%) | 277K (−12%) | 368K (**+18%**) |
| s_ff03 (ff_pole_area_ratio 0.3) | 413K (+3%) | 294K (−7%) | 351K (**+13%**) |
| s_ff07 (0.7) | 378K (−6%) | 282K (−11%) | 303K (−3%) |
| s_p1 (1 fail-fast pier) | 376K (−6%) | 291K (−8%) | 324K (+4%) |
| s_alloc (jagua-rs: edge boxes with_capacity, behind Arc) | 415K (+4%) | 307K (−3%) | 333K (+7%) |
- Allocation profile (callgrind, only_final_svg): allocation ≈ 5% of exploration Ir; mostly EdgeBoxes built by push on every HazardIndex::insert (place_item, swap_container, restore), deep-copied on CDEngine clone (~9.6K clones / 10 s), freed on remove. Also memset of the edge buffer in edges_intersect_cached (~0.9%, q3 already found removing it neutral).
- Surrogate pass: still pays on strips, costs on baldacci1. Wider checks (ff 0.7, piers) lose everywhere. Confirmation batch SUR2 running (base, ff03, alloc, ff03+alloc, sur_off).
- Confirmation batch SUR2 (~12:45 UTC; load avg logged per run, ~3 throughout):
| variant | swim | gardeyn7 | baldacci1 |
|---|---|---|---|
| s_base | 441/445/441 (442K) | 282/294/309 (295K) | 336/344/340 (340K) |
| s_ff03 | 419K (−5%) | 325K (+10%) | 356K (+5%) |
| s_alloc | 402K (−9%) | 312K (+6%) | 350K (+3%) |
| s_ff03_alloc | 425K (−4%) | 324K (+10%) | 343K (+1%) |
| s_sur_off | 360K (−19%) | 293K (−1%) | 381K (+12%) |
- Verdict: the same binary moved ±10% between batches (s_base swim 400K → 442K), so single-batch differences under ~10% are not reliable here. Consistent across both batches: skipping the surrogate pass helps baldacci1 (+12–18%) and hurts swim (−7 to −19%); ff 0.3 helps baldacci1 (+5–13%) only; wider checks lose. The allocation fix is inconsistent (−9% to +7%): no measurable gain, not committed (patch saved as scratchpad/alloc_edgeboxes.patch). No new M3 branch.
- Measurement lesson: run A/B pairs interleaved per seed (ABAB…) with repeats, not one 60 s block per variant.

## Final state (14 Oct, afternoon)
- Candidate: sparrow-pro m3/exact-k8 6c9cc8b (= quantify-proto 9ce0478 + jagua-rs pin), jagua-rs hazard-first-proto 0616a23. Baseline for the M3: m3/q3 9ab29cd. Build with --features simd,only_final_svg.
- vs main (batch M3, with per-improvement SVG export): swim +62%, gardeyn7 +75%, baldacci1 +54%; m3/exact-k8 vs m3/q3 +7–8% (kernel: 1.76× kernel, −38% instructions per quantification call; the e2e size is within this VM's noise, re-measure on the M3).
- Quality: the user's M3 race so far shows m3/exact-k8 behaving very similarly to the original.
- Open on the M3: throughput (interleaved, only_final_svg), the quality race to completion, cde_collect_1k/4 and an early-exit collection bench.
