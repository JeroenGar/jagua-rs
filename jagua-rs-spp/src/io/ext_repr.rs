use serde::{Deserialize, Serialize};

/// The JSON representation of a problem instance
#[derive(Serialize, Deserialize, Clone)]
pub struct JsonInstance {
    #[serde(rename = "Name")]
    /// The name of the instance
    pub name: String,
    /// Set of items to be produced
    #[serde(rename = "Items")]
    pub items: Vec<JsonItem>,
    /// Containers for a Bin Packing Problem
    #[serde(rename = "Objects")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bins: Option<Vec<JsonBin>>,
    /// Container for a Strip Packing Problem
    #[serde(rename = "Strip")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strip: Option<JsonStrip>,
}


/// The JSON representation of a strip with fixed height and variable width
#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct JsonStrip {
    pub height: f32,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct JsonSolution {
    /// Sum of the area of the produced items divided by the sum of the area of the containers
    pub density: f32,
    /// The time it took to generate the solution in seconds
    pub run_time_sec: u64,
    /// Layouts which compose the solution
    pub layouts: Vec<JsonLayout>,
}