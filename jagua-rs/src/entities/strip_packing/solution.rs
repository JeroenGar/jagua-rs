use std::time::Instant;
use crate::entities::general::layout::LayoutSnapshot;
use crate::fsize;

/// Represents a snapshot of a `SPProblem` at a specific moment.
/// Solutions can be used to restore the state of a `SPProblem` to a previous state.
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
