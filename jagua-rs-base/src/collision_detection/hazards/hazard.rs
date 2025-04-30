use crate::entities::{PItemKey, PlacedItem};
use crate::geometry::DTransformation;
use crate::geometry::geo_enums::GeoPosition;
use crate::geometry::primitives::SPolygon;
use std::borrow::Borrow;
use std::sync::Arc;

/// Any spatial constraint affecting the feasibility of a placement of an Item.
/// See [`HazardEntity`] for the different entities that can induce a hazard.
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
/// Entity inducing a [`Hazard`].
/// All entities are uniquely identified.
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
    BinHole { idx: usize },
    /// Represents a zone in the bin with a specific quality level that is inferior to the base quality.
    InferiorQualityZone { quality: usize, idx: usize },
}

impl HazardEntity {
    /// Whether the entity induces an 'interior' hazard, meaning everything inside its shape is hazardous.
    /// Or an 'exterior' hazard, meaning everything outside its shape is hazardous.
    pub fn position(&self) -> GeoPosition {
        match self {
            HazardEntity::PlacedItem { .. } => GeoPosition::Interior,
            HazardEntity::BinExterior => GeoPosition::Exterior,
            HazardEntity::BinHole { .. } => GeoPosition::Interior,
            HazardEntity::InferiorQualityZone { .. } => GeoPosition::Interior,
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
