use std::array;
use std::borrow::Borrow;
use std::sync::Arc;

use crate::collision_detection::hazards::Hazard;
use crate::collision_detection::hazards::HazardEntity;
use crate::collision_detection::quadtree::qt_partial_hazard::{PartialQTHaz, RelevantEdges};
use crate::geometry::geo_enums::{GeoPosition, GeoRelation};
use crate::geometry::geo_traits::{CollidesWith, Shape};
use crate::geometry::primitives::AARectangle;
use crate::geometry::primitives::SimplePolygon;
use crate::util::assertions;

/// Represents the manifestation of a [Hazard] in a [QTNode](crate::collision_detection::quadtree::qt_node::QTNode)
#[derive(Clone, Debug)]
pub struct QTHazard {
    pub entity: HazardEntity,
    pub presence: QTHazPresence,
    pub active: bool,
}

/// How a [Hazard] is present in a [QTNode](crate::collision_detection::quadtree::qt_node::QTNode)
#[derive(Clone, Debug)]
pub enum QTHazPresence {
    /// The hazard is entirely absent from the node
    None,
    /// The hazard is present in the node, but only partially, defined by a [PartialQTHaz]
    Partial(PartialQTHaz),
    /// The hazard is entirely present in the node
    Entire,
}
impl QTHazard {
    fn new(entity: HazardEntity, presence: QTHazPresence, active: bool) -> Option<Self> {
        match presence {
            QTHazPresence::None => None,
            _ => Some(Self {
                entity,
                presence,
                active,
            }),
        }
    }

    /// Returns the resulting QTHazards after constricting to the given quadrants.
    /// The quadrants should be ordered according to [AARectangle::QUADRANT_NEIGHBOR_LAYOUT]
    /// and should all be inside the bounds from which `self` was created.
    pub fn constrict(&self, quadrants: [&AARectangle; 4]) -> [Option<Self>; 4] {
        debug_assert!(assertions::quadrants_have_valid_layout(&quadrants));

        match &self.presence {
            QTHazPresence::None => array::from_fn(|_| None),
            QTHazPresence::Entire => array::from_fn(|_| Some(self.clone())),
            QTHazPresence::Partial(partial_haz) => {
                //If the hazard is partially present, it may produce different hazards for each quadrant

                //check the bbox of the hazard with the bboxes of the quadrants
                let haz_bbox = partial_haz.shape_arc().bbox();
                let haz_q_rels = quadrants.map(|q| haz_bbox.relation_to(q));

                //find the presence of the hazard in each quadrant, initially set to None (not yet determined)
                let mut q_presences = array::from_fn(|_| None);

                //Check if one of the quadrants entirely contains the hazard
                let enclosed_haz_quad_index =
                    haz_q_rels.iter().position(|r| r == &GeoRelation::Enclosed);

                if let Some(quad_index) = enclosed_haz_quad_index {
                    //If the hazard is entirely enclosed within a quadrant,
                    //it is entirely present in that quadrant and not present in the others
                    for (i, q_presence) in q_presences.iter_mut().enumerate() {
                        if i == quad_index {
                            *q_presence = Some(self.presence.clone());
                        } else {
                            *q_presence = Some(QTHazPresence::None);
                        }
                    }
                } else {
                    //The hazard is partially active in multiple quadrants, find them
                    let shape = partial_haz.shape_arc();

                    //Add the relevant edges to the presences in the quadrants
                    match &partial_haz.edges {
                        RelevantEdges::All => {
                            for edge_i in 0..shape.number_of_points() {
                                q_presences = Self::add_edge_to_q_presences(
                                    edge_i,
                                    &shape,
                                    quadrants,
                                    q_presences,
                                );
                            }
                        }
                        RelevantEdges::Some(indices) => {
                            for &edge_i in indices {
                                q_presences = Self::add_edge_to_q_presences(
                                    edge_i,
                                    &shape,
                                    quadrants,
                                    q_presences,
                                );
                            }
                        }
                    };

                    //At this point, all partial presences are determined
                    //For those without any intersecting edges, determine if they are entirely inside or outside the hazard
                    for i in 0..4 {
                        if q_presences[i].is_none() {
                            //Check if a neighbor is already resolved, if so this quadrant will have the same presence
                            //Nodes with Entire and None are never neighboring (they are always separated by a node with Partial),
                            let [n_0, n_1] = AARectangle::QUADRANT_NEIGHBOR_LAYOUT[i];
                            q_presences[i] = match (&q_presences[n_0], &q_presences[n_1]) {
                                (Some(QTHazPresence::Entire), _) => Some(QTHazPresence::Entire),
                                (_, Some(QTHazPresence::Entire)) => Some(QTHazPresence::Entire),
                                (Some(QTHazPresence::None), _) => Some(QTHazPresence::None),
                                (_, Some(QTHazPresence::None)) => Some(QTHazPresence::None),
                                _ => {
                                    //no neighbor is resolved, check its position.
                                    let haz_pos = self.entity.position();
                                    let colliding = shape.collides_with(&quadrants[i].centroid());
                                    match (haz_pos, colliding) {
                                        (GeoPosition::Interior, true) => {
                                            Some(QTHazPresence::Entire)
                                        }
                                        (GeoPosition::Exterior, false) => {
                                            Some(QTHazPresence::Entire)
                                        }
                                        _ => Some(QTHazPresence::None),
                                    }
                                }
                            }
                        }
                    }
                }

                //convert to QTHazards
                q_presences.map(|hp| match hp {
                    Some(hp) => Self::new(self.entity, hp, self.active),
                    None => unreachable!("all quadrants should have a determined presence"),
                })
            }
        }
    }

    fn add_edge_to_q_presences(
        edge_index: usize,
        shape: &Arc<SimplePolygon>,
        quadrants: [&AARectangle; 4],
        mut q_presences: [Option<QTHazPresence>; 4],
    ) -> [Option<QTHazPresence>; 4] {
        let edge = shape.get_edge(edge_index);
        //check for which quadrants the edge is relevant
        for (q_index, quad) in quadrants.iter().enumerate() {
            if quad.collides_with(&edge) {
                //relevant, add it to the constricted presence
                match &mut q_presences[q_index] {
                    None => {
                        //create a new partial hazard
                        q_presences[q_index] = Some(QTHazPresence::Partial(PartialQTHaz::new(
                            shape.clone(),
                            edge_index.into(),
                        )));
                    }
                    Some(QTHazPresence::Partial(ch)) => {
                        //add the edge to the existing partial hazard
                        ch.add_edge_index(edge_index);
                    }
                    Some(_) => {
                        unreachable!("constricted presences should be None or of type partial")
                    }
                }
            }
        }
        q_presences
    }
}

impl Into<u8> for &QTHazPresence {
    fn into(self) -> u8 {
        match self {
            QTHazPresence::None => 0,
            QTHazPresence::Partial(_) => 1,
            QTHazPresence::Entire => 2,
        }
    }
}

impl<T> From<T> for QTHazard
where
    T: Borrow<Hazard>,
{
    fn from(hazard: T) -> Self {
        Self {
            entity: hazard.borrow().entity,
            presence: QTHazPresence::Partial(hazard.borrow().into()),
            active: true,
        }
    }
}
