use serde::{Deserialize, Serialize};

use crate::config::LBFConfig;
use jagua_rs::io::json_instance::JsonInstance;
use jagua_rs::io::json_solution::JsonSolution;

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct JsonOutput {
    #[serde(flatten)]
    pub instance: JsonInstance,
    pub solution: JsonSolution,
    pub config: LBFConfig,
}
