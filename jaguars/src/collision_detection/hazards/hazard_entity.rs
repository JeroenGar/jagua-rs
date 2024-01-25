use std::hash::Hash;

use crate::entities::placed_item_uid::PlacedItemUID;
use crate::geometry::geo_enums::GeoPosition;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
// Entity inducing the hazard, every hazard entity must be unique
pub enum HazardEntity {
    Item(PlacedItemUID),
    BinOuter,
    BinHole { id: usize },
    QualityZoneInferior { quality: usize, id: usize },
}

impl HazardEntity {
    pub fn presence(&self) -> GeoPosition {
        match self {
            HazardEntity::Item(_) => GeoPosition::Interior,
            HazardEntity::BinOuter => GeoPosition::Exterior,
            HazardEntity::BinHole { .. } => GeoPosition::Interior,
            HazardEntity::QualityZoneInferior { .. } => GeoPosition::Interior,
        }
    }

    pub fn dynamic(&self) -> bool {
        match self {
            HazardEntity::Item(_) => true,
            HazardEntity::BinOuter => false,
            HazardEntity::BinHole { .. } => false,
            HazardEntity::QualityZoneInferior { .. } => false,
        }
    }
}

impl From<PlacedItemUID> for HazardEntity {
    fn from(uid: PlacedItemUID) -> Self {
        HazardEntity::Item(uid)
    }
}