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
    Disjoint
}