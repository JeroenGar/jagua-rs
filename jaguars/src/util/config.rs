use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct CDEConfig {
    pub quadtree: QuadTreeConfig,
    pub haz_prox: HazProxConfig,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum QuadTreeConfig {
    FixedDepth(u8),
    Auto,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum HazProxConfig {
    Number(usize),
    Density, //TODO
}