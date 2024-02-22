# Jagua-rs [![Rust CI](https://github.com/JeroenGar/jagua-rs/actions/workflows/rust.yml/badge.svg?branch=main)](https://github.com/JeroenGar/jagua-rs/actions/workflows/rust.yml)
### A fast and fearless Collision Detection Engine for 2D irregular Cutting and Packing problems

**🏗️ 🚧 Under construction 🚧 🏗️**

<img src="assets/jaguars_logo.svg" width="100%" height="300px" alt="jagua-rs logo">

## Introduction
2D irregular cutting and packing (C&P) problems are a class of optimization problems that involve placing irregular shaped items into containers in an efficient way.
These problems typically contain two challenges:
 * **Combinatorial**: deciding which items to place in which configuration in order to optimize some objective function.
 * **Geometric**: determining if it is possible to place an item at a certain position feasibly? (without colliding with other items, the bin or anything else)

`jagua-rs` aims to decouple these two challenges by providing a Collision Detection Engine (CDE) that can be used to efficiently deal with the geometric challenges at hand.
This allows for the separation of concerns and the development of optimization algorithms that can focus on the combinatorial challenge, while `jagua-rs` handles the geometric challenge.
Thereby, lowering the barrier to entry for researchers and practitioners to develop and test new optimization algorithms and enable independent improvements in the geometric challenge.

## Contents

### [Jagua-rs](jagua-rs)
The `jagua-rs` crate contains everything necessary to solve 2D irregular cutting and packing problems except the combinatorial aspect (i.e. deciding which items to place where). It contains all necessary components and entities to model a 2D irregular C&P instance and a collision detection engine.
The purpose of the collision detection engine to validate the feasibility of a potential item placements as fast as possible.

### [LBF](lbf)
The `lbf` crate contains a reference implementation of an optimization algorithm built on top of `jagua-rs`.
It is a simple left-bottom-fill heuristic, which places the items one-by-one in the bin, each time at the left-bottom most position.
It should provide a good starting point for anyone interested building their own optimization algorithm on top of `jagua-rs`.

### Assets

*TODO*

## Acknowledgements

This project was funded by [Research Foundation - Flanders (FWO)](https://www.fwo.be/en/) (grant number: 1S71222N)

<img src="https://upload.wikimedia.org/wikipedia/commons/f/fc/Fonds_Wetenschappelijk_Onderzoek_logo.svg" width="100px" alt="FWO logo">

## License

This project is licensed under the Mozilla Public License 2.0 - see the [LICENSE](LICENSE) file for details.
