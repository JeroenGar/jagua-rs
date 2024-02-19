# Jaguars

This crate contains everything necessary to solve 2D irregular cutting and packing problems, 
except the combinatorial decisions, i.e. which bins to use, where to place items, etc.

It is not meant to be used as a standalone algorithm, but rather is designed to support an overlying optimization algorithm.
This optimization algorithm would take combinatorial decisions, such as which items to place where, and use `jaguars` for the problem and solution representation and the feasibility check.

See [lbf](../lbf) for a reference implementation of how to use the `jaguars` in a simple left-bottom-fill heuristic.

## Design Goals

- **General purpose** 
  - [x] Bin- and strip-packing problems
  - [x] Irregular shaped items and bins
  - [x] Continuous translation and rotation of items
  - [x] Support for holes and quality zones in the bin
- **Robust**
  - [x] Uses the polygon representations to perform collision detection
  - [x] Mimics the results of a pure trigonometric approach
  - [x] Avoids floating point arithmetic errors by returning false positives (collision) instead of false negatives (no collision) in edge cases
- **Adaptable** 
  - [x] Add constraints affecting the feasibility of a placement by creating new types of `Hazards` (entities and filters)
  - [x] Define new C&P problems by creating a custom `Instance` and accompanying `Problem` variants
- **Fast**
  - [x] Maximum query and update performance
  - [x] Able to resolve millions of collision queries per second
  - [x] Can simplify polygons in preprocessing

## Features
`jaguars` provides the following features:
- Modeling of 2D irregular C&P problems
  - All necessary entities represent 2D irregular cutting and packing problem: 
    - items, bins, layouts, etc.
  - All necessary components to create solutions: 
    - placing and removing items, taking snapshots, etc.
- Collision detection engine:
  - Validate if a certain item can be placed at a certain position without colliding with anything else (feasibility check)
- Parser for reading problem instances in a JSON format.
- A geometric toolbox for working with polygons, rectangles, circles, points, edges, etc.

## Documentation

The code is documented using rustdoc. 
The docs can be build using `cargo doc --open` from the root of the repository.