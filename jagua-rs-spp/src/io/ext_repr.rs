use serde::{Deserialize, Serialize};

use jagua_rs_base::io::ext_repr::{ExtLayout};

/// External representation of a strip packing problem instance
#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct ExtSPInstance {
    #[serde(rename = "Name")]
    /// The name of the instance
    pub name: String,
    /// Set of items to be produced
    #[serde(rename = "Items")]
    pub items: Vec<ExtItem>,
    /// Fixed height of the strip
    pub strip_height: f32,
}

/// External representation of an item with a demand
#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct ExtItem {
    #[serde(flatten)]
    /// External representation of the item in the base library
    pub base: jagua_rs_base::io::ext_repr::ExtItem,
    /// Amount of times this item needs to be produced
    pub demand: u64
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct ExtSPSolution {
    /// Sum of the area of the produced items divided by the sum of the area of the containers
    pub density: f32,
    /// The time it took to generate the solution in seconds
    pub run_time_sec: u64,
    /// Layouts which compose the solution
    pub layout: ExtLayout,
    /// The strip width of the solution
    pub strip_width: f32,
}