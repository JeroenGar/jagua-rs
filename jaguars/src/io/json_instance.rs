use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct JsonInstance {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Items")]
    pub items: Vec<JsonItem>,
    #[serde(rename = "Objects")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bins: Option<Vec<JsonBin>>,
    #[serde(rename = "Strip")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strip: Option<JsonStrip>,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct JsonBin {
    pub cost: u64,
    pub stock: u64,
    pub shape: JsonPoly,
    pub zones: Option<HashMap<String, JsonZone>>,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct JsonItem {
    pub demand: u64,
    pub base_quality: Option<usize>,
    pub allowed_orientations: Option<Vec<f64>>,
    pub shape: JsonPoly,
    pub zones: Option<HashMap<String, JsonZone>>,
    pub value: Option<u64>
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct JsonPoly {
    pub outer: JsonSimplePoly,
    pub inner: Option<Vec<JsonSimplePoly>>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct JsonSimplePoly(pub Vec<(f64, f64)>);

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct JsonZone {
    pub quality: usize,
    pub shape: JsonPoly,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct JsonStrip {
    pub height: f64,
}