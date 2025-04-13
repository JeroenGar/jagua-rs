use std::time::Instant;
use crate::entities::general::layout::LayoutSnapshot;
use crate::fsize;

#[cfg(doc)]
use crate::entities::strip_packing::problem::SPProblem;

/// Snapshot of [`SPProblem`] at a specific moment. Can be used to restore [`SPProblem`] to a previous state.
#[derive(Debug, Clone)]
pub struct SPSolution {
    /// Width of the strip
    pub strip_width: fsize,
    /// Snapshot of the strip
    pub layout_snapshot: LayoutSnapshot,
    /// Usage of the strip
    pub usage: fsize,
    /// Instant the solution was created
    pub time_stamp: Instant,
}
