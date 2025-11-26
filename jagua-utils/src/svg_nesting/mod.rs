//! SVG nesting module

mod parsing;
mod strategy;
mod svg_generation;

pub use parsing::*;
pub use strategy::{NestingStrategy, SimpleNestingStrategy};
pub use svg_generation::NestingResult;

use anyhow::Result;

/// Nest SVG parts into a bin using the simple single-run strategy
///
/// # Arguments
/// * `bin_width` - Width of the bin
/// * `bin_height` - Height of the bin
/// * `spacing` - Minimum spacing between parts
/// * `svg_part_bytes` - SVG part as bytes (should contain a path element)
/// * `amount_of_parts` - Number of parts to place
/// * `amount_of_rotations` - Number of discrete rotations to allow (0 = no rotation, 1 = 0°, 2 = 0° and 180°, etc.)
/// * `loops` - Number of optimization loops to run (ignored, kept for compatibility)
/// * `placements` - Number of placement samples per loop (ignored, kept for compatibility)
///
/// # Returns
/// * [`NestingResult`] containing SVG bytes and placement metadata
pub fn nest_svg_parts(
    bin_width: f32,
    bin_height: f32,
    spacing: f32,
    svg_part_bytes: &[u8],
    amount_of_parts: usize,
    amount_of_rotations: usize,
    _loops: usize,
    _placements: usize,
) -> Result<NestingResult> {
    let strategy = SimpleNestingStrategy::new();
    strategy.nest(
        bin_width,
        bin_height,
        spacing,
        svg_part_bytes,
        amount_of_parts,
        amount_of_rotations,
    )
}
