# Jagua-rs ![workflow](https://github.com/JeroenGar/jagua-rs/actions/workflows/rust.yml/badge.svg)
**An open-source collision detection engine for 2D irregular cutting and packing problems written in Rust 🦀.**

**🏗️ 🚧 Under construction 🚧 🏗️**

<img src="assets/jaguars_logo.svg" width="50%" alt="Jaguars logo">

## Introduction
2D irregular cutting and packing (C&P) problems are a class of optimization problems that involve placing irregular shaped items into containers in an efficient way.
These problems typically contain two challenges:
 * **Combinatorial challenge**: deciding which items to place in which configuration in order to optimize some objective function.
 * **Geometric challenge**: determining if it is possible to place an item at a certain position feasibly? (without colliding with other items, the bin or anything else)

`jagua-rs` aims to decouple these two challenges by providing a Collision Detection Engine (CDE) that can be used to efficiently deal with the geometric challenges at hand.
This allows for the separation of concerns and the development of optimization algorithms that can focus on the combinatorial challenge, while `jagua-rs` handles the geometric challenge.
Thereby, lowering the barrier to entry for researchers and practitioners to develop and test new optimization algorithms and enable independent improvements in the geometric challenge.

## Contents

### Jagua-rs
The **[`jagua-rs`](jagua-rs)** crate contains everything necessary to solve 2D irregular cutting and packing problems without the combinatorial decision-making (i.e. which items to place where). It provides all necessary entities and components to create a dynamic model of a 2D irregular C&P instance and provide a collision detection engine to check the feasibility of a placement.

## LBF
The **[`lbf`](lbf)** crate contains a reference implementation of an optimization algorithm using `jagua-rs`.
It is a simple left-bottom-fill heuristic, which places the items one-by-one in the bin each time at the left-bottom most position.
It should provide a good starting point for anyone looking to create a more advanced optimization algorithm using ``jagua-rs``.

## Assets

TODO

## Acknowledgements

This project was funded by [Research Foundation - Flanders (FWO)](https://www.fwo.be/en/) (grant number: 1S71222N)

<img src="https://upload.wikimedia.org/wikipedia/commons/f/fc/Fonds_Wetenschappelijk_Onderzoek_logo.svg" width="10%" alt="FWO logo">

## License

This project is licensed under the Mozilla Public License 2.0 - see the [LICENSE](LICENSE) file for details.