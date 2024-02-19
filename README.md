# Jagua-rs ![workflow](https://github.com/JeroenGar/jagua-rs/actions/workflows/rust.yml/badge.svg)
**An open-source collision detection engine for 2D irregular cutting and packing problems written in Rust 🦀.**

<img src="assets/jaguars_logo.svg" width="50%" alt="Jaguars logo">

# 🚧 Under construction 🚧

## Introduction
2D irregular cutting and packing (C&P) problems are a class of optimization problems that involve placing irregular shaped items into containers in an efficient way.
These problems typically contain two challenges:
 * **Combinatorial** challenge: deciding which items to place where to minimize some objective function.
 * **Geometric** challenge: can we place an item at a certain position without colliding with other items? (feasibility check)

`Jagua-rs` aims to decouple these two challenges by developing a Collision Detection Engine (CDE) that can be used to efficiently deal with the geometric challenges at hand.


## Contents

### Jaguars
The `jaguars` crate contains all required logic for representing 2D irregular C&P problems and also contains the collision detection engine.

**See [jaguars](jaguars) for more information.**

## LBF
The `lbf` crate contains a reference implementation of an optimization algorithm using `jaguars`. \
It is a simple left-bottom-fill heuristic, which places the items one-by-one in the bin each time at the left-bottom most position.
It should provide a good starting point for anyone looking to create a more advanced optimization algorithm using ``jaguars``.

**See [lbf](lbf) for more information.**

## Assets

TODO

## Acknowledgements

This project was funded by [Research Foundation - Flanders (FWO)](https://www.fwo.be/en/) (grant number: 1S71222N)

<img src="https://upload.wikimedia.org/wikipedia/commons/f/fc/Fonds_Wetenschappelijk_Onderzoek_logo.svg" width="10%" alt="FWO logo">

## License

This project is licensed under the Mozilla Public License 2.0 - see the [LICENSE](LICENSE) file for details.