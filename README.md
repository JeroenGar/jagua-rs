# Jagua-rs [![Rust CI](https://github.com/JeroenGar/jagua-rs/actions/workflows/rust.yml/badge.svg?branch=main)](https://github.com/JeroenGar/jagua-rs/actions/workflows/rust.yml)[![Docs](https://github.com/JeroenGar/jagua-rs/actions/workflows/doc.yml/badge.svg)](https://github.com/JeroenGar/jagua-rs/actions/workflows/doc.yml)
### A fast and fearless Collision Detection Engine for 2D irregular Cutting and Packing problems

**🏗️ 🚧 Under construction 🚧 🏗️**

<img src="assets/jaguars_logo.svg" width="100%" height="300px" alt="jagua-rs logo">

## Preamble
2D irregular cutting and packing (C&P) problems are a class of optimization problems that involve placing irregular shaped items into containers in an efficient way.
These problems contain two distinct challenges:
 * **Combinatorial**: deciding which items to place in which configuration in order to optimize some objective function.
 * **Geometric**: determining the feasibility of a placement. Does the item fit in the container? Does it not collide with other items?

`jagua-rs` decouples these challenges by providing a Collision Detection Engine (CDE) that efficiently deals with the geometric challenge.
By making use of `jagua-rs`, you can confidently focus on the combinatorial aspects without having to worry about sufficiently fast feasibility checks.

A reference implementation of an optimization algorithm built on top of `jagua-rs` is also provided in the form of the `lbf` crate.

## `jagua-rs` 🐆
`jagua-rs` incorporates all components required to create an **easily manipulable internal representation** of 2D irregular C&P problems.
It also boasts a powerful **Collision Detection Engine (CDE)** that can efficiently determine whether an item can fit at a specific position without *colliding* with anything.
The speed at which these *feasibility checks* can be resolved is of paramount importance, defining the design constraints of the optimization algorithms that rely on it.

### Design Goals
- **Currently supports:**
    - [x] Bin- and strip-packing problems
    - [x] Both irregular shaped items and bins
    - [x] Continuous rotation and translation
    - [x] Holes and quality zones in the bin
- **Robust:**
    - [x] Uses the polygon representation of items and bins, and mimics the results of a simple trigonometric approach
    - [x] Special care is taken to handle floating-point arithmetic edge cases
    - [x] Written in pure Rust 🦀
- **Adaptable:**
    - [x] Define custom C&P problem variants by adding new `Instance` and `Problem` implementations
    - [x] Add extra constraints by creating new `Hazards` and `HazardFilters`
        - [x] `Hazards`: consolidating all spatial constraints into a single model
        - [x] `HazardFilters`: excluding specific `Hazards` on a per-query basis
- **Performant:**
    - [x] Focus on maximum `query` and `update` performance
    - [x] Able to resolve millions of collision queries per second
    - [x] Contains preprocessor to simplify polygons

## `lbf` ↙️
The `lbf` crate contains a reference implementation of an optimization algorithm built on top of `jagua-rs`.
It is a simple left-bottom-fill heuristic, which places the items one-by-one in the bin, each time at the left-bottom most position.
It should provide a good starting point for anyone interested building their own optimization algorithm on top of `jagua-rs`.

### How to run
General usage:
```bash
cd lbf
cargo run --release -- \
  -i <input file> \
  -c <config file (optional)> \
  -s <solution folder> \
  -l <log level (optional)>
```

Concrete example:
```bash
cd lbf
cargo run --release -- \
  -i ../assets/swim.json \
  -c ../assets/config_lbf.json \
  -s ../solutions
```

### Input

The [assets](assets) folder contains a set of input files from the academic literature that were converted to the
same JSON structure.

The files are also available in the [OR-Datasets repository](https://github.com/Oscar-Oliveira/OR-Datasets/tree/master/Cutting-and-Packing/2D-Irregular) by Oscar Oliveira.

### Solution

Two types of files are written in the solution folder: the solution in JSON format and an SVG file per layout to visualize the solution.

#### JSON

*TODO*

#### SVG

*TODO*

*Note: Unfortunately, the SVG standard does not support strokes drawn purely inside (or outside) shapes.
Items might therefore sometimes falsely appear to be (very slightly) colliding in the SVG visualizations.*

### Config JSON

If no config file is provided, the default configuration is used.

The configuration file is a JSON file with the following structure:
```javascript
{
  "cde_config": { //Configuration of the collision detection engine
    "quadtree_depth": 5, //Maximum depth of the quadtree is 5
    "hpg_n_cells": 2000, //The hazard proximity grid contains 2000 cells
    "item_surrogate_config": {
      "pole_coverage_goal": 0.9, //The surrogate will stop generating poles when 90% of the item is covered
      "max_poles": 10, //The surrogate will at most generate 10 poles
      "n_ff_poles": 2, //Two poles will be used for fail-fast collision detection
      "n_ff_piers": 0 //Zero piers will be used for fail-fast collision detection
    }
  },
  "poly_simpl_config": { // Polygon simplification configuration
    "mode": "enabled", //[enabled/disabled]
    "params": {
      "tolerance": 0.001 //Polygons will be simplified until they deviate at most 0.1% from their original area.
    }
  },
  "deterministic_mode": true, //The heuristic will always produce the same solution for the same input and configuration
  "n_samples_per_item": 5000, //5000 placement samples will be queried per item.
  "ls_samples_fraction": 0.2 //Of those 5000, 80% will be sampled at uniformly at random, 20% will be local search samples
}
```

An example config file is provided [here](assets/config_lbf.json).

### Important note

Due to `lbf` being a one-pass constructive heuristic, the final solution quality is extremely *chaotic*. \
Meaning that minute changes in the flow (sorting of the items, configuration, prng seed...) lead to solutions with drastically different quality. \
Seemingly superior configurations (such as increased `n_samples_per_item`), for example, can result in worse solutions and vice versa. \
Testing with `deterministic_mode` set to `false` will demonstrate this spread in solution quality.

**Once again, this heuristic should only serve as a reference implementation of how to use `jagua-rs` and not as a reliable optimization algorithm for any real-world problems.**

## Testing

The `jagua-rs` codebase contains a suite of assertion checks which verify the correctness of the engine.
These `debug_asserts` are enabled by default in debug builds and tests but are omitted in release builds to maximize performance.

Additionally, `lbf` contains some basic integration tests to validate the correctness of the engine on a macro level.
It basically runs the heuristic on a set of input files with multiple configurations with assertions enabled.

To run the tests, use:
```bash
cd lbf
cargo test
``` 

## Documentation

Documentation of this repo is written with rustdoc and is automatically deployed to GitHub pages.

[Link to `jagua-rs` docs](https://jeroengar.github.io/jagua-rs-docs/jagua_rs/).

[Link to `lbf` docs](https://jeroengar.github.io/jagua-rs-docs/lbf/).

Alternatively use `cargo doc --open` to compile and view the documentation in your browser.

## Acknowledgements

This project was funded by [Research Foundation - Flanders (FWO)](https://www.fwo.be/en/) (grant number: 1S71222N)

<img src="https://upload.wikimedia.org/wikipedia/commons/f/fc/Fonds_Wetenschappelijk_Onderzoek_logo.svg" width="100px" alt="FWO logo">

## License

This project is licensed under the Mozilla Public License 2.0 - see the [LICENSE](LICENSE) file for details.
