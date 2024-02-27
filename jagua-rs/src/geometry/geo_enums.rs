#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum GeoPosition {
    Exterior,
    Interior,
}

impl GeoPosition {
    pub fn inverse(&self) -> Self {
        match self {
            GeoPosition::Exterior => GeoPosition::Interior,
            GeoPosition::Interior => GeoPosition::Exterior,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum GeoRelation {
    Intersecting,
    Enclosed,
    Surrounding,
    Disjoint,
}

#[derive(Clone, Debug, PartialEq)]
pub enum AllowedRotation {
    /// No rotation is allowed
    None,
    /// Any rotation is allowed
    Continuous,
    /// Only a limited set of rotations is allowed
    Discrete(Vec<f64>),
}
