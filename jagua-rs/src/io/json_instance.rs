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
    /// Set of bins where the items are to be placed (for Bin Packing problems)
    #[serde(rename = "Objects")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bins: Option<Vec<JsonBin>>,
    /// A strip where the items are to be placed (for Strip Packing problems)
    #[serde(rename = "Strip")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strip: Option<JsonStrip>,
}

/// The JSON representation of a bin
#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct JsonBin {
    /// The cost of using this bin
    pub cost: u64,
    /// Number of this bin available, if not present, it is assumed to be unlimited
    pub stock: Option<u64>,
    /// Polygon shape of the bin
    pub shape: JsonShape,
    /// A list of zones with different quality levels
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub zones: Vec<JsonQualityZone>,
}

/// The JSON representation of a strip
#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct JsonStrip {
    pub height: f64,
}

/// The JSON representation of an item
#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct JsonItem {
    /// Number of times this item should be produced
    pub demand: u64,
    /// List of allowed orientations angles (in degrees), if not present, any orientation is allowed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_orientations: Option<Vec<f64>>,
    /// Polygon shape of the item
    pub shape: JsonShape,
    /// The value of the item (for knapsack problems)
    pub value: Option<u64>,
    /// The quality required for the entire item
    pub base_quality: Option<usize>,
}

/// An enum containing all the possible possibilities to define a shape
#[derive(Serialize, Deserialize, Clone)]
#[serde(tag = "Type", content = "Data")]
#[serde(rename_all = "PascalCase")]
pub enum JsonShape {
    SimplePolygon(JsonSimplePoly),
    Polygon(JsonPoly),
    MultiPolygon(Vec<JsonPoly>),
}

/// A polygon represented as an outer boundary and a list of holes
#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct JsonPoly {
    /// The outer boundary of the polygon
    pub outer: JsonSimplePoly,
    /// A list of holes in the polygon
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub inner: Vec<JsonSimplePoly>,
}

/// A simple polygon represented as a list of points (x, y)
#[derive(Serialize, Deserialize, Clone)]
pub struct JsonSimplePoly(pub Vec<(f64, f64)>);

/// A zone with a specific quality level
#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct JsonQualityZone {
    /// The quality level of this zone
    pub quality: usize,
    /// The polygon shape of this zone
    pub shape: JsonShape,
}
