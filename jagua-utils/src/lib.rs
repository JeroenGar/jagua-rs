//! Utility functions for jagua-rs
//!
//! This crate provides utility functions for working with jagua-rs,
//! including SVG nesting utilities.

pub mod svg_nesting;

pub use svg_nesting::{
    AdaptiveConfig, NestingResult, SubResultHandler, nest_svg_parts, nest_svg_parts_adaptive,
};
