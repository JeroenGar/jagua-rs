//!
//! A fast and fearless Collision Detection Engine for 2D irregular cutting and packing problems.
//!
//!
//! This crate can be configured to use single or double precision for floating points (see [fsize]).

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

/// The floating point type used in jagua-rs.
cfg_if::cfg_if! {
    if #[cfg(feature = "double-precision")] {
        /// The floating point type used in jagua-rs.
        /// ```f32``` by default, ```f64``` with feature **double-precision**.
        pub type fsize = f64;
        /// The constant PI as [fsize].
        pub const PI : fsize = std::f64::consts::PI;
    } else {
        /// The floating point type used in jagua-rs.
        /// ```f32``` by default, ```f64``` with feature **double-precision**.
        pub type fsize = f32;
        /// The constant PI as [fsize].
        pub const PI: fsize = std::f32::consts::PI;
    }
}
