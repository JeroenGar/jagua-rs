//!
//! A fast and fearless Collision Detection Engine for 2D irregular cutting and packing problems.
//!
//! This library is designed to be used in optimization algorithms for solving irregular 2D cutting and packing problems.

/// Everything collision detection engine related
pub mod collision_detection;

/// Entities to model 2D irregular cutting and packing problems
pub mod entities;

/// Geometric primitives and base algorithms
pub mod geometry;

/// Parser and JSON (de)serialization
pub mod io;

/// Helper functions
pub mod util;
