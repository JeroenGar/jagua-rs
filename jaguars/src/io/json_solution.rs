use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct JsonSolution {
    pub usage: f64,
    pub run_time_sec: u64,
    pub layouts: Vec<JsonLayout>,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct JsonLayout {
    pub object_type: JsonObjectType,
    pub placed_items: Vec<JsonPlacedItem>,
    pub statistics: JsonLayoutStats,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct JsonPlacedItem {
    pub item_index: usize,
    pub transformation: JsonTransformation,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct JsonTransformation {
    pub rotation: f64,
    pub translation: (f64, f64),
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct JsonLayoutStats {
    //
    pub usage: f64,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub enum JsonObjectType {
    Object { id: usize },
    Strip {
        #[serde(rename = "Length")]
        width: f64,
        height: f64,
    },
}