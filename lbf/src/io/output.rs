use crate::config::LBFConfig;
use jagua_rs::probs::bpp::io::ext_repr::{ExtBPInstance, ExtBPSolution};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct BPOutput {
    #[serde(flatten)]
    pub instance: ExtBPInstance,
    pub solution: ExtBPSolution,
    pub config: LBFConfig,
}
