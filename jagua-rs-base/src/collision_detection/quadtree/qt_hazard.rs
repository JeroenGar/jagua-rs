use crate::collision_detection::hazards::Hazard;
use crate::collision_detection::hazards::HazardEntity;
use crate::collision_detection::quadtree::qt_partial_hazard::{QTHazPartial, RelevantEdges};
use crate::geometry::geo_enums::{GeoPosition, GeoRelation};
use crate::geometry::geo_traits::{CollidesWith, Shape};
use crate::geometry::primitives::Rect;
use crate::geometry::primitives::SPolygon;
use crate::util::assertions;
use std::array;
use std::sync::Arc;

/// Representation of a [`Hazard`] in a [`QTNode`](crate::collision_detection::quadtree::QTNode)
#[derive(Clone, Debug)]
pub struct QTHazard {
    /// The bounding box of the quadtree node
    pub qt_bbox: Rect,
    /// Entity inducing the hazard
    pub entity: HazardEntity,
    /// How the hazard is present in the node
    pub presence: QTHazPresence,
    /// Whether the hazard is active or not
    pub active: bool,
}

/// Presence of a [`Hazard`] in a [`QTNode`](crate::collision_detection::quadtree::QTNode)
#[derive(Clone, Debug)]
pub enum QTHazPresence {
    /// The hazard is entirely absent from the node
    None,
    /// The hazard is only partially present in the node
    Partial(QTHazPartial),
    /// The hazard is present in the entire node.
    Entire,
}
impl QTHazard {
    /// Converts a [`Hazard`] into a [`QTHazard`], assuming it is for the root of the quadtree.
    pub fn from_qt_root(qt_root_bbox: Rect, haz: &Hazard) -> Self {
        Self {
            qt_bbox: qt_root_bbox,
            entity: haz.entity,
            presence: QTHazPresence::Partial(QTHazPartial {
                shape: Arc::downgrade(&haz.shape),
                edges: RelevantEdges::All,
            }),
            active: haz.active,
        }
    }

    /// Returns the resulting QTHazards after constricting to the provided quadrants.
    /// The quadrants should be ordered according to the [Cartesian system](https://en.wikipedia.org/wiki/Quadrant_(plane_geometry))
    /// and should all be inside the bounds from which `self` was created.
    pub fn constrict(&self, quadrants: [Rect; 4]) -> Option<[Self; 4]> {
        debug_assert!(
            quadrants
                .iter()
                .all(|q| self.qt_bbox.relation_to(*q) == GeoRelation::Surrounding)
        );
        debug_assert!(assertions::quadrants_have_valid_layout(&quadrants));

        match &self.presence {
            QTHazPresence::None => None,
            QTHazPresence::Entire => Some(array::from_fn(|_| self.clone())),
            QTHazPresence::Partial(partial_haz) => {
                //If the hazard is partially present, it may produce different hazards for each quadrant

                //check the bbox of the hazard with the bboxes of the quadrants
                let haz_bbox = partial_haz.shape_arc().bbox;

                //Check if one of the quadrants entirely contains the hazard
                let enclosed_hazard_quadrant = quadrants
                    .iter()
                    .map(|q| haz_bbox.relation_to(*q))
                    .position(|r| r == GeoRelation::Enclosed);

                if let Some(quad_index) = enclosed_hazard_quadrant {
                    //The hazard is entirely enclosed within one quadrant,
                    //For this quadrant the QTHazard is equivalent to the original hazard, the rest are None
                    let hazards = array::from_fn(|i| {
                        let presence = match i {
                            i if i == quad_index => QTHazPresence::Partial(partial_haz.clone()),
                            _ => QTHazPresence::None,
                        };
                        Self {
                            qt_bbox: quadrants[i],
                            entity: self.entity,
                            presence,
                            active: self.active,
                        }
                    });
                    Some(hazards)
                } else {
                    //The hazard is partially active in multiple quadrants, find which ones
                    let shape = partial_haz.shape_arc();

                    //For every quadrant, check which of the edges of the hazard are relevant
                    let mut constricted_hazards = quadrants.map(|q| {
                        compute_edge_collisions_in_quadrant(q, &partial_haz.edges, &shape).map(
                            |p_haz| {
                                //The quadrant has collisions with the hazard edges
                                //For these quadrants we can be sure they will be of Partial presence
                                QTHazard {
                                    qt_bbox: q,
                                    entity: self.entity,
                                    presence: QTHazPresence::Partial(p_haz),
                                    active: self.active,
                                }
                            },
                        )
                    });

                    debug_assert!(constricted_hazards.iter().filter(|h| h.is_some()).count() > 0);

                    //At this point, we know which quadrants have collisions with which edges.
                    //What remain are the quadrants without any intersecting edges.
                    //These can either have the hazard entirely present or entirely absent.
                    for i in 0..4 {
                        let quadrant = quadrants[i];
                        if constricted_hazards[i].is_none() {
                            //Presence of Entire and None type are always separated by a node with Partial presence.
                            //If a neighbor is already resolved to Entire or None, this quadrant will have the same presence.
                            let [neighbor_0, neighbor_1] = Rect::QUADRANT_NEIGHBOR_LAYOUT[i];
                            let presence_n0 = constricted_hazards[neighbor_0]
                                .as_ref()
                                .map(|h| &h.presence);
                            let presence_n1 = constricted_hazards[neighbor_1]
                                .as_ref()
                                .map(|h| &h.presence);

                            let presence = match (presence_n0, &presence_n1) {
                                (Some(QTHazPresence::None), Some(QTHazPresence::Entire))
                                | (Some(QTHazPresence::Entire), Some(QTHazPresence::None)) => {
                                    unreachable!(
                                        "one of the neighbors is Entire, the other is None, this quadrant should be Partial"
                                    )
                                }
                                (Some(QTHazPresence::Entire), _) => QTHazPresence::Entire,
                                (_, Some(QTHazPresence::Entire)) => QTHazPresence::Entire,
                                (Some(QTHazPresence::None), _) => QTHazPresence::None,
                                (_, Some(QTHazPresence::None)) => QTHazPresence::None,
                                _ => {
                                    //Neither of its neighbors is resolved, check its position.
                                    let haz_pos = self.entity.position();
                                    //Since partial presence is not possible, checking whether the center of the quadrant collides or not suffices
                                    let colliding = shape.collides_with(&quadrant.centroid());
                                    match (haz_pos, colliding) {
                                        (GeoPosition::Interior, true) => QTHazPresence::Entire,
                                        (GeoPosition::Exterior, false) => QTHazPresence::Entire,
                                        _ => QTHazPresence::None,
                                    }
                                }
                            };

                            constricted_hazards[i] = Some(QTHazard {
                                qt_bbox: quadrant,
                                entity: self.entity,
                                presence,
                                active: self.active,
                            });
                        }
                    }

                    let constricted_hazards = constricted_hazards
                        .map(|h| h.expect("all constricted hazards should be resolved"));

                    Some(constricted_hazards)
                }
            }
        }
    }
}

fn compute_edge_collisions_in_quadrant(
    quadrant: Rect,
    relevant_edges: &RelevantEdges,
    shape: &Arc<SPolygon>,
) -> Option<QTHazPartial> {
    let mut p_haz = None;

    let mut check_edge = |idx| {
        let edge = shape.edge(idx);
        if quadrant.collides_with(&edge) {
            p_haz
                .get_or_insert(QTHazPartial {
                    shape: Arc::downgrade(shape),
                    edges: RelevantEdges::Some(vec![]),
                })
                .register_edge(idx);
        }
    };

    match relevant_edges {
        RelevantEdges::All => {
            for i in 0..shape.n_vertices() {
                check_edge(i);
            }
        }
        RelevantEdges::Some(indices) => {
            for &i in indices {
                check_edge(i);
            }
        }
    }

    p_haz
}
