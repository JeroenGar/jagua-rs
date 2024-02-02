use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub struct CDEConfig {
    pub quadtree: QuadTreeConfig,
    pub haz_prox: HazProxConfig,
    pub item_surrogate_config: SPSurrogateConfig,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum QuadTreeConfig {
    FixedDepth(u8),
    Auto,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum HazProxConfig {
    Disabled,
    Enabled { n_cells: usize },
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub struct SPSurrogateConfig {
    /// poles will be generated until this percentage of the shape is covered
    pub pole_coverage_goal: f64,
    /// maximum number of poles to generate
    pub max_poles: usize,
    ///number of poles to test during fail fast
    pub n_ff_poles: usize,
    ///number of piers to test during fail fast
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