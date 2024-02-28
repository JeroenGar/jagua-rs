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

/// The JSON representation of a strip with fixed height and variable width
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
    /// List of allowed orientations angles (in degrees). If none any orientation is allowed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_orientations: Option<Vec<f64>>,
    /// Polygon shape of the item
    pub shape: JsonShape,
    /// The value of the item (for knapsack problems)
    pub value: Option<u64>,
    /// The quality required for the entire item, if not defined maximum quality is required
    pub base_quality: Option<usize>,
}

/// Different ways to represent a shape
#[derive(Serialize, Deserialize, Clone)]
#[serde(tag = "Type", content = "Data")]
#[serde(rename_all = "PascalCase")]
pub enum JsonShape {
    /// Axis-aligned rectangle
    Rectangle { width: f64, height: f64 },
    /// Polygon with a single outer boundary
    SimplePolygon(JsonSimplePoly),
    /// Polygon with a single outer boundary and a list of holes
    Polygon(JsonPoly),
    /// Multiple disjoint polygons
    MultiPolygon(Vec<JsonPoly>),
}

/// A polygon represented as an outer boundary and a list of holes
#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct JsonPoly {
    /// The outer boundary of the polygon
    pub outer: JsonSimplePoly,
    /// A list of holes in the polygon
    #[serde(default)]
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
