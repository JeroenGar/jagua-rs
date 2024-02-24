use serde::{Deserialize, Serialize};

/// Representation of a solution
#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct JsonSolution {
    /// Sum of the area of the produced items divided by the sum of the area of the containers
    pub usage: f64,
    /// The time it took to generate the solution in seconds
    pub run_time_sec: u64,
    /// Layouts which compose the solution
    pub layouts: Vec<JsonLayout>,
}

/// Representation how a set of items are placed in a certain container
#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct JsonLayout {
    /// The container that was used
    pub container: JsonContainer,
    /// The items placed in the container and where they were placed
    pub placed_items: Vec<JsonPlacedItem>,
    /// Some statistics about the layout
    pub statistics: JsonLayoutStats,
}

/// Represents an item placed in a container
#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct JsonPlacedItem {
    /// The index of the item in the instance
    pub index: usize,
    /// The transformation applied to the item to place it in the container
    pub transformation: JsonTransformation,
}

/// Represents a proper rigid transformation defined as a rotation followed by translation
#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct JsonTransformation {
    /// The rotation angle in radians
    pub rotation: f64,
    /// The translation vector (x, y)
    pub translation: (f64, f64),
}

/// Some statistics about the layout
#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct JsonLayoutStats {
    /// The percentage of the container that is packed with items
    pub usage: f64,
}

/// Type of container that was used
#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
#[serde(tag = "Type", content = "Params")]
pub enum JsonContainer {
    Bin {
        /// The index of the object in the instance
        #[serde(rename = "Index")]
        index: usize,
    },
    Strip {
        /// The length of the strip (variable)
        #[serde(rename = "Length")]
        width: f64,
        /// The height of the strip (fixed)
        #[serde(rename = "Height")]
        height: f64,
    },
}
