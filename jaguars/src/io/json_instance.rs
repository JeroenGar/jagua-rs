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
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub zones: Vec<JsonQualityZone>,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct JsonStrip {
    pub height: f64,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct JsonItem {
    /// Number of times this item should be produced
    pub demand: u64,
    /// List of allowed orientations angles (in degrees).
    /// If Some(), only the specified angles are allowed
    /// If None, continuous rotation is allowed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_orientations: Option<Vec<f64>>,
    pub shape: JsonPoly,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub zones: Vec<JsonQualityZone>,
    pub value: Option<u64>,
    pub base_quality: Option<usize>,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct JsonPoly {
    pub outer: JsonSimplePoly,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub inner: Vec<JsonSimplePoly>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct JsonSimplePoly(pub Vec<(f64, f64)>);

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct JsonQualityZone {
    pub quality: usize,
    pub shape: JsonPoly,
}