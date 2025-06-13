use std::ops::RangeInclusive;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum GeoPosition {
    Exterior,
    Interior,
}

#[derive(Debug, PartialEq)]
/// Possible relations between two geometric entities A and B.
/// A is `GeoRelation` to B
pub enum GeoRelation {
    /// A ∩ B ≠ ∅ and neither A ⊆ B nor B ⊆ A
    Intersecting,
    /// A ⊆ B
    Enclosed,
    /// B ⊆ A
    Surrounding,
    /// A ∩ B = ∅
    Disjoint,
}

#[derive(Clone, Debug, PartialEq)]
pub enum AllowedRotation {
    /// A single allowed rotation angle (in radians)
    Fixed(f32),
    /// A range of allowed rotation angles (in radians)
    Range(RangeInclusive<f32>),
}
