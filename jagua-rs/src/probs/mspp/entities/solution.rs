use slotmap::SecondaryMap;
use crate::Instant;
use crate::entities::LayoutSnapshot;
use crate::probs::mspp::entities::instance::MSPInstance;
use crate::probs::mspp::entities::problem::LayKey;
use crate::probs::mspp::entities::strip::MStrip;

/// Snapshot of [`SPProblem`](crate::probs::spp::entities::SPProblem) at a specific moment. Can be used to restore to a previous state.
#[derive(Debug, Clone)]
pub struct MSPSolution {
    /// A map of the bin layout snapshots, identified by the same keys as in the problem
    pub bin_layout_snapshots: SecondaryMap<LayKey, LayoutSnapshot>,
    /// Snapshot of the strip layout
    pub strip_layout_snapshot: (MStrip, LayoutSnapshot),
    /// Instant the solution was created
    pub time_stamp: Instant,
}

impl MSPSolution {
    pub fn density(&self, instance: &MSPInstance) -> f32 {
        self.layout_snapshot.density(instance)
    }
    pub fn strip_width(&self) -> f32 {
        self.strip.width
    }
}
