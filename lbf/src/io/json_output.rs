use serde::{Deserialize, Serialize};

use crate::config::LBFConfig;
use jagua_rs_bpp::io::ext_repr::{ExtBPInstance, ExtBPSolution};
use jagua_rs_spp::io::ext_repr::{ExtSPInstance, ExtSPSolution};

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct SPOutput {
    #[serde(flatten)]
    pub instance: ExtSPInstance,
    pub solution: ExtSPSolution,
    pub config: LBFConfig,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct BPOutput {
    #[serde(flatten)]
    pub instance: ExtBPInstance,
    pub solution: ExtBPSolution,
    pub config: LBFConfig,
}
