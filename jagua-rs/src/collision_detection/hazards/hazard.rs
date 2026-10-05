use crate::geometry::DTransformation;
use crate::geometry::geo_enums::GeoPosition;
use crate::geometry::primitives::SPolygon;
use slotmap::new_key_type;
use std::fmt::Debug;
use std::hash::Hash;
use std::sync::Arc;

new_key_type! {
    /// Key to identify hazards inside the CDE.
    pub struct HazKey;

    /// Key of a placed item, identifying it in [`BasicHazardEntity::PlacedItem`].
    pub struct PItemKey;
}

/// Any spatial constraint affecting the feasibility of a placement of an Item.
/// The entity type `E` defines what can induce a hazard; see [`HazardEntity`].
#[derive(Clone, Debug)]
pub struct Hazard<E = BasicHazardEntity> {
    /// The entity inducing the hazard
    pub entity: E,
    /// The shape of the hazard
    pub shape: Arc<SPolygon>,
}

impl<E: HazardEntity> Hazard<E> {
    #[must_use]
    pub fn new(entity: E, shape: Arc<SPolygon>) -> Self {
        Self { entity, shape }
    }
}

/// Entity inducing a [`Hazard`]: everything the collision engine needs to know about it.
/// Each entity registered in a [`CDEngine`](crate::collision_detection::CDEngine) must be unique.
///
/// [`BasicHazardEntity`] covers the entities of this crate's layouts. Applications with other
/// kinds of hazards can define their own.
pub trait HazardEntity: Copy + Eq + Hash + Debug {
    /// Whether the hazard covers the interior or the exterior of its shape.
    /// An engine needs exactly one exterior hazard.
    fn scope(&self) -> GeoPosition;

    /// Whether the hazard can be registered and deregistered over time, such as a placed item.
    /// Only dynamic hazards are saved and restored by
    /// [`CDEngine::save`](crate::collision_detection::CDEngine::save) and
    /// [`CDEngine::restore`](crate::collision_detection::CDEngine::restore).
    fn is_dynamic(&self) -> bool;
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
/// Entity inducing a [`Hazard`] in this crate's layouts.
/// All entities are uniquely identified.
pub enum BasicHazardEntity {
    /// An item placed in the layout, defined by its id, applied transformation and key
    PlacedItem {
        id: usize,
        dt: DTransformation,
        pk: PItemKey,
    },
    /// Represents all regions outside the container
    Exterior,
    /// Represents a hole in the container.
    Hole { idx: usize },
    /// Represents a zone in the container with a specific quality level that is inferior to the base quality.
    InferiorQualityZone { quality: usize, idx: usize },
}

impl HazardEntity for BasicHazardEntity {
    fn scope(&self) -> GeoPosition {
        match self {
            BasicHazardEntity::PlacedItem { .. }
            | BasicHazardEntity::Hole { .. }
            | BasicHazardEntity::InferiorQualityZone { .. } => GeoPosition::Interior,
            BasicHazardEntity::Exterior => GeoPosition::Exterior,
        }
    }

    fn is_dynamic(&self) -> bool {
        matches!(self, BasicHazardEntity::PlacedItem { .. })
    }
}
