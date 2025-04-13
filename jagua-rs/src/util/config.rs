use serde::{Deserialize, Serialize};

use crate::fsize;

///Configuration of the [`CDEngine`](crate::collision_detection::CDEngine)
#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub struct CDEConfig {
    ///Maximum depth of the quadtree
    pub quadtree_depth: u8,
    ///Target number of cells in the Hazard Proximity Grid (set to 0 to disable)
    pub hpg_n_cells: usize,
    ///Configuration of the surrogate generation for items
    pub item_surrogate_config: SPSurrogateConfig,
}

/// maximum number of definable pole limits, increase if needed
const N_POLE_LIMITS: usize = 3;

/// Configuration of the [`SPSurrogate`](crate::geometry::fail_fast::SPSurrogate) generation
#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub struct SPSurrogateConfig {
    ///Limits on the number of poles to be generated at different coverage levels.
    ///For example: [(100, 0.0), (20, 0.75), (10, 0.90)]:
    ///While the coverage is below 75% the generation will stop at 100 poles.
    ///If 75% coverage with 20 or more poles the generation will stop.
    ///If 90% coverage with 10 or more poles the generation will stop.
    pub n_pole_limits: [(usize, fsize); N_POLE_LIMITS],
    ///Number of poles to test during fail-fast (additional poles are exclusively used in the hazard proximity grid)
    pub n_ff_poles: usize,
    ///number of piers to test during fail-fast
    pub n_ff_piers: usize,
}

impl SPSurrogateConfig {
    pub fn none() -> Self {
        Self {
            n_pole_limits: [(0, 0.0); N_POLE_LIMITS],
            n_ff_poles: 0,
            n_ff_piers: 0,
        }
    }
}
