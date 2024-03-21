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
    if #[cfg(feature = "double_precision")] {
        pub type fsize = f64;
        pub const PI : fsize = std::f64::consts::PI;
    } else {
        pub type fsize = f32;
        pub const PI: fsize = std::f32::consts::PI;
    }
}
