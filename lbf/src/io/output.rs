use serde::{Deserialize, Serialize};
use jagua_rs::prob_variants::bpp::io::ext_repr::{ExtBPInstance, ExtBPSolution};
use jagua_rs::prob_variants::spp::io::ext_repr::{ExtSPInstance, ExtSPSolution};
use crate::config::LBFConfig;

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
