use serde::{Deserialize, Serialize};

use jaguars::io::json_instance::JsonInstance;
use jaguars::io::json_solution::JsonSolution;

use crate::config::Config;

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct JsonOutput {
    #[serde(flatten)]
    pub instance: JsonInstance,
    pub solution: JsonSolution,
    pub config: Config,
}