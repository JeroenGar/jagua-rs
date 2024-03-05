use std::sync::Arc;

use crate::entities::placed_item::PlacedItemUID;
use crate::geometry::geo_enums::GeoPosition;
use crate::geometry::primitives::simple_polygon::SimplePolygon;

/// Defines a certain spatial constraint that affects the feasibility of a placement.
#[derive(Clone, Debug)]
pub struct Hazard {
    /// The entity inducing the hazard
    pub entity: HazardEntity,
    /// The shape of the hazard
    pub shape: Arc<SimplePolygon>,
    /// Hazards can be either active or inactive, inactive hazards are not considered during collision detection
    pub active: bool,
}

impl Hazard {
    pub fn new(entity: HazardEntity, shape: Arc<SimplePolygon>) -> Self {
        Self {
            entity,
            shape,
            active: true,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
/// Entity inducing the `Hazard`. All entities are uniquely identified.
pub enum HazardEntity {
    /// An item placed in the layout.
    PlacedItem(PlacedItemUID),
    /// Represents all regions outside the bin
    BinExterior,
    /// Represents a hole in the bin.
    BinHole { id: usize },
    /// Represents a zone in the bin with a specific quality level that is inferior to the base quality.
    InferiorQualityZone { quality: usize, id: usize },
}

impl HazardEntity {
    /// Whether the entity induces an `Interior` or `Exterior` hazard
    pub fn position(&self) -> GeoPosition {
        match self {
            HazardEntity::PlacedItem(_) => GeoPosition::Interior,
            HazardEntity::BinExterior => GeoPosition::Exterior,
            HazardEntity::BinHole { .. } => GeoPosition::Interior,
            HazardEntity::InferiorQualityZone { .. } => GeoPosition::Interior,
        }
    }

    /// Whether the entity is dynamic in nature, i.e. it can be modified in the layout
    pub fn dynamic(&self) -> bool {
        match self {
            HazardEntity::PlacedItem(_) => true,
            HazardEntity::BinExterior => false,
            HazardEntity::BinHole { .. } => false,
            HazardEntity::InferiorQualityZone { .. } => false,
        }
    }

    /// Whether the entity universally applicable, i.e. all items need to be checked against it
    pub fn universal(&self) -> bool {
        match self {
            HazardEntity::PlacedItem(_) => true,
            HazardEntity::BinExterior => true,
            HazardEntity::BinHole { .. } => true,
            HazardEntity::InferiorQualityZone { .. } => false,
        }
    }
}

impl From<PlacedItemUID> for HazardEntity {
    fn from(p_uid: PlacedItemUID) -> Self {
        HazardEntity::PlacedItem(p_uid)
    }
}
