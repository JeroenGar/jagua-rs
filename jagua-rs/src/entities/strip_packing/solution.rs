use crate::entities::general::LayoutSnapshot;
use crate::fsize;
use std::time::Instant;

#[cfg(doc)]
use crate::entities::strip_packing::SPProblem;

/// Snapshot of [`SPProblem`] at a specific moment. Can be used to restore [`SPProblem`] to a previous state.
#[derive(Debug, Clone)]
pub struct SPSolution {
    /// Width of the strip
    pub strip_width: fsize,
    /// Snapshot of the strip
    pub layout_snapshot: LayoutSnapshot,
    /// Instant the solution was created
    pub time_stamp: Instant,
}
