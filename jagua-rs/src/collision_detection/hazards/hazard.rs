use crate::collision_detection::edges::{EdgeBoxes, EdgeChains, EdgeSet};
use crate::geometry::DTransformation;
use crate::geometry::geo_enums::GeoPosition;
use crate::geometry::primitives::{Edge, Rect, SPolygon};
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
/// Cloning is cheap: the shape and the geometry derived from it are shared.
#[derive(Clone, Debug)]
pub struct Hazard<E = BasicHazardEntity> {
    entity: E,
    shape: Arc<SPolygon>,
    geometry: HazardGeometry,
}

impl<E: HazardEntity> Hazard<E> {
    #[must_use]
    pub fn new(entity: E, shape: Arc<SPolygon>) -> Self {
        let geometry = HazardGeometry::new(&shape, entity.scope());
        Self {
            entity,
            shape,
            geometry,
        }
    }
}

impl<E: Copy> Hazard<E> {
    /// The entity inducing the hazard
    #[must_use]
    pub fn entity(&self) -> E {
        self.entity
    }

    /// The shape of the hazard
    #[must_use]
    pub fn shape(&self) -> &Arc<SPolygon> {
        &self.shape
    }

    pub(crate) fn geometry(&self) -> &HazardGeometry {
        &self.geometry
    }
}

impl<E> EdgeSet for Hazard<E> {
    #[inline(always)]
    #[allow(clippy::inline_always)] // part of the query loops
    fn any_edge_overlapping(&self, r: Rect, f: impl FnMut(&Edge) -> bool) -> bool {
        match &self.geometry {
            HazardGeometry::Interior(boxes) => boxes.any_edge_overlapping(&self.shape, r, f),
            HazardGeometry::Exterior { chains, .. } => chains.any_edge_overlapping(r, f),
        }
    }
}

/// Geometry derived from a hazard's shape to speed up collision queries.
/// Cloning shares the derived data.
#[derive(Clone, Debug)]
pub(crate) enum HazardGeometry {
    Interior(EdgeBoxes),
    /// The container's outline
    Exterior {
        chains: Arc<EdgeChains>,
        /// Whether the outline is its own bounding box
        is_rect: bool,
    },
}

impl HazardGeometry {
    #[allow(clippy::float_cmp)] // the bounding box's sides are vertex coordinates
    fn new(shape: &SPolygon, scope: GeoPosition) -> Self {
        match scope {
            GeoPosition::Interior => HazardGeometry::Interior(EdgeBoxes::new(shape)),
            GeoPosition::Exterior => {
                let bbox = shape.bbox;
                let is_rect = shape.n_vertices() == 4
                    && shape.vertices.iter().all(|p| {
                        (p.0 == bbox.x_min || p.0 == bbox.x_max)
                            && (p.1 == bbox.y_min || p.1 == bbox.y_max)
                    });
                HazardGeometry::Exterior {
                    chains: Arc::new(EdgeChains::new(shape)),
                    is_rect,
                }
            }
        }
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
}
