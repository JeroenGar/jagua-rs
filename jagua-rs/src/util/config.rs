use serde::{Deserialize, Serialize};

use crate::fsize;

///Configuration of the Collision Detection Engine
#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub struct CDEConfig {
    ///Maximum depth of the quadtree
    pub quadtree_depth: u8,
    ///Target number of cells in the Hazard Proximity Grid
    pub hpg_n_cells: usize,
    ///Configuration of the surrogate generation for items
    pub item_surrogate_config: SPSurrogateConfig,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub struct SPSurrogateConfig {
    ///Poles will stop being generated when the surrogate covers this fraction of the shape's area
    pub pole_coverage_goal: fsize,
    ///Maximum number of poles to generate
    pub max_poles: usize,
    ///Number of poles to test during fail-fast (additional poles are exclusively used in the hazard proximity grid)
    pub n_ff_poles: usize,
    ///number of piers to test during fail-fast
    pub n_ff_piers: usize,
}

impl SPSurrogateConfig {
    pub fn none() -> Self {
        Self {
            pole_coverage_goal: 0.0,
            max_poles: 0,
            n_ff_poles: 0,
            n_ff_piers: 0,
        }
    }
}
