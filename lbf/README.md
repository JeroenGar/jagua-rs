# LBF
A left-bottom-fill heuristic for 2D irregular cutting and packing problems using the `jagua-rs` library.

This heuristic serves as a reference implementation of how to use the collision detection engine.
It is a very simple constructive heuristic that places items in the bin from left to right and bottom to top.

## How to run
General usage:
```bash
    cargo run --release \
      -i <input file> \
      -c <config file (optional)> \
      -s <solution folder>
```

Concrete example:
```bash
    cargo run --release \
      -i ../assets/swim.json \
      -c ../assets/config_lbf.json \
      -s ../solutions
```

## Input JSON

TODO

## Output JSON

TODO

## Output SVG

In the solution folder, for every layout in the solution, and SVG file will be created to visualize the layout.

## Config JSON

```json
{
  "cde_config": { // Collision detection engine configuration
    "quadtree_depth": 5, // Maximum depth of the quadtree is 5
    "hpg_n_cells": 2000, // The hazard proximity grid contains 2000 cells
    "item_surrogate_config": {
      "pole_coverage_goal": 0.9, // The surrogate will stop generating poles when 90% of the item is covered
      "max_poles": 10, // The surrogate will at most generate 10 poles
      "n_ff_poles": 2, // 2 poles will be used for fail-fast collision detection
      "n_ff_piers": 0 // 0 piers will be used for fail-fast collision detection
    }
  },
  "poly_simpl_config": { // Polygon simplification configuration
    "mode": "enabled", // [enabled, disabled]
    "params": {
      "tolerance": 0.001 // The polygons will be simplified until they deviate at most 0.1% from their original area.
    }
  },
  "deterministic_mode": true, // The heuristic will always produce the same solution for the same input and configuration
  "n_samples_per_item": 5000, // It will sample in total 5000 placements for each item
  "ls_samples_fraction": 0.2 // Of those 5000, 80% will be random samples, 20% will be local search samples
}
```

