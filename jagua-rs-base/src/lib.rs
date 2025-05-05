#![doc = document_features::document_features!()]
//! The base `jagua-rs` module containing all shared components (problem-type agnostic)

/// Everything related to the Collision Detection Engine
pub mod collision_detection;

/// Entities to model 2D Irregular Cutting and Packing Problems
pub mod entities;

/// Geometric primitives and base algorithms
pub mod geometry;

/// Importing problem instances into and exporting solutions out of this library
pub mod io;

/// Helper functions which do not belong to any specific module
pub mod util;
