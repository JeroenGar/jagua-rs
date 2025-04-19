use crate::entities::general::{PItemKey, PlacedItem};
use crate::geometry::DTransformation;
use crate::geometry::geo_enums::GeoPosition;
use crate::geometry::primitives::SPolygon;
use std::borrow::Borrow;
use std::sync::Arc;

/// Abstraction around any spatial constraint that affects the feasibility of a placement.
/// See [`HazardEntity`] for the different types of hazards.
#[derive(Clone, Debug)]
pub struct Hazard {
    /// The entity inducing the hazard
    pub entity: HazardEntity,
    /// The shape of the hazard
    pub shape: Arc<SPolygon>,
    /// Hazards can be either active or inactive, inactive hazards are not considered during collision detection
    pub active: bool,
}

impl Hazard {
    pub fn new(entity: HazardEntity, shape: Arc<SPolygon>) -> Self {
        Self {
            entity,
            shape,
            active: true,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
/// Entity inducing a [Hazard]. All entities are uniquely identified.
pub enum HazardEntity {
    /// An item placed in the layout, defined by its id, applied transformation and key
    PlacedItem {
        id: usize,
        dt: DTransformation,
        pk: PItemKey,
    },
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
            HazardEntity::PlacedItem { .. } => GeoPosition::Interior,
            HazardEntity::BinExterior => GeoPosition::Exterior,
            HazardEntity::BinHole { .. } => GeoPosition::Interior,
            HazardEntity::InferiorQualityZone { .. } => GeoPosition::Interior,
        }
    }

    /// Whether the entity is dynamic in nature, i.e. it can be modified in the layout
    pub fn is_dynamic(&self) -> bool {
        match self {
            HazardEntity::PlacedItem { .. } => true,
            HazardEntity::BinExterior => false,
            HazardEntity::BinHole { .. } => false,
            HazardEntity::InferiorQualityZone { .. } => false,
        }
    }

    /// Whether the entity universally applicable, i.e. all items need to be checked against it
    pub fn is_universal(&self) -> bool {
        match self {
            HazardEntity::PlacedItem { .. } => true,
            HazardEntity::BinExterior => true,
            HazardEntity::BinHole { .. } => true,
            HazardEntity::InferiorQualityZone { .. } => false,
        }
    }
}

impl<T> From<(PItemKey, T)> for HazardEntity
where
    T: Borrow<PlacedItem>,
{
    fn from((pk, pi): (PItemKey, T)) -> Self {
        HazardEntity::PlacedItem {
            id: pi.borrow().item_id,
            dt: pi.borrow().d_transf,
            pk,
        }
    }
}
