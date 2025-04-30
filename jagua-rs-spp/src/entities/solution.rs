use std::time::Instant;
use jagua_rs_base::entities::LayoutSnapshot;
use crate::entities::SPInstance;
use crate::entities::strip::Strip;

/// Snapshot of [`SPProblem`](crate::entities::SPProblem) at a specific moment. Can be used to restore to a previous state.
#[derive(Debug, Clone)]
pub struct SPSolution {
    /// Width of the strip
    pub strip: Strip,
    /// Snapshot of the strip
    pub layout_snapshot: LayoutSnapshot,
    /// Instant the solution was created
    pub time_stamp: Instant,
}

impl SPSolution {
    pub fn density(&self, instance: &SPInstance) -> f32 {
        self.layout_snapshot.density(instance)
    }
}
