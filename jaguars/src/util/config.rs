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
    Number(usize),
    Density, //TODO
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub struct SPSurrogateConfig {
    pub pole_coverage_goal: f64,
    pub max_poles: usize,
    pub n_ff_poles: usize, //number of poles to test during fail fast
    pub n_ff_piers: usize, //number of piers to test during fail fast
}