# Jaguars

The jaguars contains everything necessary to solve 2D irregular cutting and packing problems, 
except the combinatorial decisions: which bins to use, where to place items, etc.

It contains:
- All necessary components to create a mutable representation of 2D irregular cutting and packing problems:
  - Items can be placed at any position in any bin.
  - Items can be removed.
  - Snapshots can be taken and used to revert to a previous state.
- Collision detection engine: 
  - Efficiently validate if an item can be placed at a certain position.
- Parser for reading problem instances in a JSON format.
- Set of geometric tools

- Support for:
    - [x] bin- and strip-packing problems.
    - [x] Irregular bins and items
    - [x] Continuous proper rigid transformations (rotation + translation).
    - [x] Support for quality zones.

In summary, `jaguars` can be used to model 2D irregular C\&P problems

It cannot be used on its own, but is designed to be used in combination with an overlying optimization algorithm to solve 2D irregular cutting and packing problems.

## Docs

The code is documented using rustdoc. 
The docs can be build using `cargo doc --open` from the root of the repository.

## How to use

See the [lbf readme](../lbf/README.md) for a reference implementation of how to use the `jaguars`.