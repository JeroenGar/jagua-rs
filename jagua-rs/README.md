# Jagua-rs 🐆

This crate provides the all essential building blocks for solve 2D irregular Cutting and Packing (C&P) problems except the combinatorial decision-making aspects, such as where to place each of the items.

`jagua-rs` is a fundamental building block to be used to build by optimization algorithms.

`jagua-rs` incorporates all components required to create an **easily manipulable internal representation** of 2D irregular C&P problems.
It also boasts a powerful **Collision Detection Engine (CDE)** that can efficiently determine whether an item can fit at a specific position without *colliding* with anything.
The speed at which these *feasibility checks* can be resolved is of paramount importance, defining the design constraints of the optimization algorithms that rely on it.

See [lbf crate](../lbf) for a reference implementation of an optimization algorithm making use of `jagua-rs`.

## Design Goals

- **Built-in support for:** 
  - [x] Bin- and strip-packing problems
  - [x] Both irregular shaped items and bins
  - [x] Continuous rotation and translation
  - [x] Holes and quality zones in the bin
- **Robustness:**
  - [x] Uses the polygonal representation of shapes
  - [x] Mimics the results of a simple trigonometric approach, albeit much faster
  - [x] Implementation handles floating-point arithmetic edge cases by erring on the side of caution
  - [x] Fully written in Rust 🦀
- **Adaptabilty:**
  - [x] Define custom C&P problem variants by adding new `Instance` and `Problem` implementations
  - [x] Add new constraints by creating new `Hazards` and `HazardFilters`
    - [x] `Hazards` consolidate all spatial constraints into a single model
    - [x] `HazardFilters` enable customized collision checks by selectively excluding specific `Hazards`
- **Performance:**
  - [x] Focus on maximum `query` and `update` performance
  - [x] Able to resolve millions of collision queries per second
  - [x] Custom polygon simplification procedures in preprocessing

## Documentation

Documentation of this repo is automatically deployed to [GitHub Pages](https://jeroengar.github.io/jagua-rs-docs/jagua_rs/).

Or alternatively, use `cargo doc --open` to compile locally and view the documentation in your browser.

## Testing

`jagua-rs` contains a suite of assertions which are enabled by default in debug builds to ensure the correctness of the engine.
These checks are sprinkled throughout the codebase and aim to verify correctness of many of the datastructures.
In release builds, these `debug_assert`s are omitted by default to maximize performance.

Additionally, `lbf` contains some basic integration tests to validate the correctness of the engine on a macro level.
See [lbf crate](../lbf#Testing) for more information.
