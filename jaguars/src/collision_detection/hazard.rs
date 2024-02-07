use std::sync::Arc;

use crate::entities::placed_item::PlacedItemUID;
use crate::geometry::geo_enums::GeoPosition;
use crate::geometry::primitives::simple_polygon::SimplePolygon;

/// Defines a certain spatial constraint that affects the feasibility of placed items
/// Hazards are defined by a certain entity, have a shape and can be active or inactive
#[derive(Clone, Debug)]
pub struct Hazard {
    /// The entity inducing the hazard
    pub entity: HazardEntity,
    /// The shape of the hazard
    pub shape: Arc<SimplePolygon>,
    /// Whether the hazard is currently active or not
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
/// Entity inducing a hazard, every hazard entity must be unique
pub enum HazardEntity {
    PlacedItem(PlacedItemUID),
    BinExterior,
    BinHole { id: usize },
    QualityZoneInferior { quality: usize, id: usize },
}

impl HazardEntity {

    /// Whether the entity induces an Interior or Exterior hazard
    pub fn position(&self) -> GeoPosition {
        match self {
            HazardEntity::PlacedItem(_) => GeoPosition::Interior,
            HazardEntity::BinExterior => GeoPosition::Exterior,
            HazardEntity::BinHole { .. } => GeoPosition::Interior,
            HazardEntity::QualityZoneInferior { .. } => GeoPosition::Interior,
        }
    }

    /// True if the hazard is dynamic in nature, i.e. it can be modified by the optimizer
    pub fn dynamic(&self) -> bool {
        match self {
            HazardEntity::PlacedItem(_) => true,
            HazardEntity::BinExterior => false,
            HazardEntity::BinHole { .. } => false,
            HazardEntity::QualityZoneInferior { .. } => false,
        }
    }

    /// Returns true if the hazard is universally applicable, i.e. all items are affected by it
    pub fn universal(&self) -> bool {
        match self {
            HazardEntity::PlacedItem(_) => true,
            HazardEntity::BinExterior => true,
            HazardEntity::BinHole { .. } => true,
            HazardEntity::QualityZoneInferior { .. } => false,
        }
    }
}

impl From<PlacedItemUID> for HazardEntity {
    fn from(p_uid: PlacedItemUID) -> Self {
        HazardEntity::PlacedItem(p_uid)
    }
}