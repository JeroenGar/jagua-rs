# jagua-rs [![Rust CI](https://github.com/JeroenGar/jagua-rs/actions/workflows/rust.yml/badge.svg?branch=main)](https://github.com/JeroenGar/jagua-rs/actions/workflows/rust.yml)[![Docs](https://github.com/JeroenGar/jagua-rs/actions/workflows/doc.yml/badge.svg)](https://jeroengar.github.io/jagua-rs-docs/jagua_rs/)

### A fast and fearless collision detection engine for 2D irregular cutting and packing problems.

<img src="img/jaguars_logo.svg" width="100%" height="300px" alt="jagua-rs logo">

## Preamble

2D irregular cutting and packing (C&P) problems are a class of combinatorial optimization problems that involve placing irregular
shaped items into containers in an efficient way.
These problems contain two distinct challenges:

* **Optimization**: deciding which items to place in which configuration in order to optimize some objective function.
* **Geometric**: ensuring a placement is feasible. Does the item fit in the container? Does it not collide
  with other items?

Previously, those tackling these problems have had to address both challenges simultaneously.
This is particulary demanding given that it requires two distinct sets of expertise and lots of research & development effort.

**This project aims to decouple the two challenges by providing a Collision Detection Engine (CDE) that can efficiently handle the
geometric aspects of 2D irregular C&P problems.**
The CDE's main responsibility is determining if an item can be placed at a certain location without causing any *collisions*, which would render a solution infeasible.
The CDE embedded in `jagua-rs` is powerful enough to resolve millions of these collision queries every second.

`jagua-rs` enables you to confidently focus on the combinatorial aspects of the optimization challenge at hand, without
having to worry about the underlying geometry.

In addition, a reference implementation of a basic optimization algorithm built on top of `jagua-rs` is provided in the `lbf` crate.

## `jagua-rs` 🐆

`jagua-rs` includes all components required to create an **easily manipulable internal representation** of 2D
irregular C&P problems.
It also boasts a powerful **Collision Detection Engine (CDE)** which determines whether an item can fit at a specific
position without causing any *collisions*.

### Design Goals

- **Performant:**
  - [x] Focus on maximum performance, both in terms of query resolution and update speed
  - [x] Can resolve millions of collision queries per second
  - [x] Integrated preprocessor to simplify polygons
- **Robust:**
  - [x] Designed to mimic the exact results of a naive trigonometric approach
  - [x] Special care is taken to handle edge cases caused by floating-point arithmetic
  - [x] Written in pure Rust 🦀
- **Adaptable:**
  - [x] Define custom C&P problem variants by creating new `Instance` and accompanying `Problem` implementations
  - [x] Add extra constraints by creating new `Hazards` and `HazardFilters`
    - [x] `Hazards`: consolidation of all spatial constraints into a single model
    - [x] `HazardFilters`: excluding specific `Hazards` from consideration on a per-query basis
- **Currently supports:**
  - [x] Bin- & strip-packing problems
  - [x] Irregular-shaped items & bins
  - [x] Continuous rotation & translation (double precision)
  - [x] Holes and quality zones in the bin

## `lbf` ↙️

The `lbf` crate contains a reference implementation of an optimization algorithm built on top of `jagua-rs`.
It is a simple left-bottom-fill heuristic, which sequentially places the items into the bin, each time at the left-bottom
most position.

The code is thoroughly documented and should provide a good starting point for anyone interested building their own optimization algorithm on top
of `jagua-rs`.

### How to run LBF

Ensure [Rust and Cargo](https://www.rust-lang.org/learn/get-started) are installed and up to date.

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

The [assets](assets) folder contains a set of problem instances from the academic literature that were converted to the
same JSON structure.

The files are also available in Oscar Oliveira's
[OR-Datasets repository](https://github.com/Oscar-Oliveira/OR-Datasets/tree/master/Cutting-and-Packing/2D-Irregular).

### Solution

At the end of the optimization, the solution is written to the specified folder.
Two types of files are written:

#### JSON

The solution JSON is similar to the input JSON, but with the addition of the `Solution` key at the top level.
It contains all information required to recreate the solution, such as the bins used, how the items are placed inside and some additional statistics.

#### SVG

A visual representation of every layout of the solution is created as an SVG file.
By default, only the bin and the items placed inside it are drawn.
Optionally the quadtree, hazard proximity grid and fail-fast surrogates can be drawn on top.
A custom color theme can also be defined.

All visual options be configured in the config file, see [docs](https://jeroengar.github.io/jagua-rs-docs/lbf/io/svg_util/struct.SvgDrawOptions.html) for all available
options.

Some examples of layout SVGs created by `lbf`:
<p align="center">
  <img src="img/sp_example.svg" width="22%" alt="strip packing example">
  <img src="img/leather_example.svg" width="20%" alt="leather example">
  <img src="img/bp_example.svg" width="50%" alt="bin packing example">
</p>

*Note: Unfortunately, the SVG standard does not support strokes drawn purely inside (or outside) of polygons.
Items might therefore sometimes falsely appear to be (very slightly) colliding in the SVG visualizations.*

### Config JSON

Configuration of `jagua-rs` and the `lbf` heuristic is done through a JSON file.
An example config file is provided [here](assets/config_lbf.json).
If no config file is provided, the default configuration is used.

The configuration file has the following structure:
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
  "poly_simpl_tolerance": 0.001, //Polygons will be simplified until at most a 0.1% deviation in area from the original
  "prng_seed": 0, //Seed for the pseudo-random number generator. If undefined the outcome will be non-deterministic
  "n_samples": 5000, //5000 placement samples will be queried per item per layout
  "ls_frac": 0.2 //Of those 5000 samples, 80% will be sampled at uniformly at random, 20% will be local search samples
}
```

See [docs](https://jeroengar.github.io/jagua-rs-docs/lbf/lbf_config/struct.LBFConfig.html) for a detailed description of all available configuration options.

### Important note

Due to `lbf` being a one-pass constructive heuristic, the final solution quality is very *chaotic*.
Tiny changes in the operation of the algorithm (sorting of the items, configuration, prng seed...)
will lead to solutions with drastically different quality. \
Seemingly superior configurations (such as increased `n_samples`), for example, may result in worse solutions and vice versa. \
Omitting `prng_seed` in the config file disables the deterministic behavior and will demonstrate this variation in solution quality.

**This heuristic merely serves as a reference implementation of how to use `jagua-rs` 
and should probably not be used as an optimization algorithm for any real-world use case.**

## Documentation

Documentation of this repo is written in rustdoc and the most recent version is automatically deployed and hosted on GitHub Pages:

- `jagua-rs` docs: [https://jeroengar.github.io/jagua-rs-docs/jagua_rs/](https://jeroengar.github.io/jagua-rs-docs/jagua_rs/)
- `lbf` docs: [https://jeroengar.github.io/jagua-rs-docs/lbf/](https://jeroengar.github.io/jagua-rs-docs/lbf/)

Alternatively, you can compile and view the docs of older versions locally by using: `cargo doc --open`.

## Testing

These `debug_asserts` are enabled by default in debug and test builds, but are omitted in release builds to maximize performance.

Additionally, `lbf` contains some basic integration tests to validate the general correctness of the engine.
These tests essentially run the heuristic on a set of input files, using multiple configurations and with assertions enabled.

The coverage and granularity of the tests needs to be expanded in the future.

## Development

Contributions to `jagua-rs` are more than welcome!
To submit code contributions: [fork](https://help.github.com/articles/fork-a-repo/) the repository,
commit your changes, and [submit a pull request](https://help.github.com/articles/creating-a-pull-request-from-a-fork/).

## License

This project is licensed under Mozilla Public License 2.0 - see the [LICENSE](LICENSE) file for details.

## Acknowledgements

This project began development at [KU Leuven](https://www.kuleuven.be/english/) and was funded by [Research Foundation - Flanders (FWO)](https://www.fwo.be/en/) (grant number: 1S71222N).

<img src="https://upload.wikimedia.org/wikipedia/commons/9/97/Fonds_Wetenschappelijk_Onderzoek_logo_2024.svg" height="50px" alt="FWO logo">
&nbsp;
<img src="https://upload.wikimedia.org/wikipedia/commons/4/49/KU_Leuven_logo.svg" height="50px" alt="KU Leuven logo">
