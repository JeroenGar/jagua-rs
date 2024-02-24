use std::borrow::Borrow;

use crate::collision_detection::hazard::Hazard;
use crate::collision_detection::hazard::HazardEntity;
use crate::collision_detection::quadtree::qt_partial_hazard::{EdgeIndices, QTPartialHazard};
use crate::geometry::geo_enums::{GeoPosition, GeoRelation};
use crate::geometry::geo_traits::{CollidesWith, Shape};
use crate::geometry::primitives::aa_rectangle::AARectangle;

/// Represents the manifestation of a `Hazard` in a `QTNode`
#[derive(Clone, Debug)]
pub struct QTHazard {
    pub entity: HazardEntity,
    pub presence: QTHazPresence,
    pub active: bool,
}

#[derive(Clone, Debug)]
pub enum QTHazPresence {
    None,
    Partial(QTPartialHazard),
    Entire,
}

impl<T> From<T> for QTHazard
where
    T: Borrow<Hazard>,
{
    fn from(hazard: T) -> Self {
        Self {
            entity: hazard.borrow().entity.clone(),
            presence: QTHazPresence::Partial(hazard.borrow().into()),
            active: true,
        }
    }
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

    pub fn constrict(&self, quadrants: [&AARectangle; 4]) -> [Option<Self>; 4] {
        match &self.presence {
            QTHazPresence::None => [None, None, None, None],
            QTHazPresence::Entire => [
                Some(self.clone()),
                Some(self.clone()),
                Some(self.clone()),
                Some(self.clone()),
            ],
            QTHazPresence::Partial(partial_haz) => {
                //check the bbox of the hazard with the bboxes of the quadrants
                let haz_bbox = partial_haz.shape_arc().bbox();
                let haz_quad_relations = [
                    haz_bbox.relation_to(&quadrants[0]),
                    haz_bbox.relation_to(&quadrants[1]),
                    haz_bbox.relation_to(&quadrants[2]),
                    haz_bbox.relation_to(&quadrants[3]),
                ];
                let mut constricted_presence = [None, None, None, None];

                if let Some(quad_index) = haz_quad_relations
                    .iter()
                    .position(|r| r == &GeoRelation::Enclosed)
                {
                    //if the hazard is entirely inside one of the quadrants, clone the current hazard for that quadrant and set all the rest to none
                    //ensure all the other quadrants are disjoint
                    debug_assert!(haz_quad_relations
                        .iter()
                        .enumerate()
                        .filter(|(i, _)| *i != quad_index)
                        .all(|(_, r)| r == &GeoRelation::Disjoint));
                    constricted_presence[quad_index] = Some(self.presence.clone());
                } else {
                    //the hazard is partially active in multiple quadrants, find them
                    let shape = partial_haz.shape_arc();
                    let mut check_collisions_with_quadrants = |edge_index: usize| {
                        let edge = shape.get_edge(edge_index);
                        for quad_index in 0..4 {
                            let quadrant = quadrants[quad_index];
                            if quadrant.collides_with(&edge) {
                                let constricted_haz_presence = constricted_presence[quad_index]
                                    .get_or_insert(QTHazPresence::Partial(QTPartialHazard::new(
                                        partial_haz.shape_arc(),
                                        EdgeIndices::Some(vec![]),
                                    )));
                                match constricted_haz_presence {
                                    QTHazPresence::Partial(constricted_haz) => {
                                        constricted_haz.add_edge_index(edge_index);
                                    }
                                    _ => panic!("constricted hazard is not partial"),
                                }
                            }
                        }
                    };

                    match &partial_haz.edge_indices {
                        EdgeIndices::All => {
                            for edge_index in 0..shape.number_of_points() {
                                check_collisions_with_quadrants(edge_index);
                            }
                        }
                        EdgeIndices::Some(indices) => {
                            for edge_index in indices {
                                check_collisions_with_quadrants(*edge_index);
                            }
                        }
                    };

                    //for the quadrants that do not have any intersecting edges, determine if they are entirely inside or outside the hazard
                    for i in 0..4 {
                        if constricted_presence[i].is_none() {
                            //check if a neighbor is already resolved
                            // Because nodes with Entire and None are never neighboring (they are always separated by a node with Partial),
                            // if a neighbor is either Entire or None, this quadrant is also Entire or None
                            let [n_0, n_1] = AARectangle::QUADRANT_NEIGHBOR_LAYOUT[i];
                            constricted_presence[i] =
                                match (&constricted_presence[n_0], &constricted_presence[n_1]) {
                                    (Some(QTHazPresence::Entire), _)
                                    | (_, Some(QTHazPresence::Entire)) => {
                                        Some(QTHazPresence::Entire)
                                    }
                                    (Some(QTHazPresence::None), _)
                                    | (_, Some(QTHazPresence::None)) => Some(QTHazPresence::None),
                                    _ => {
                                        let point_to_test = quadrants[i].centroid();
                                        match (
                                            self.entity.position(),
                                            shape.collides_with(&point_to_test),
                                        ) {
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

                //convert them to QTHazards
                let constricted_hazards: [Option<Self>; 4] =
                    constricted_presence.map(|hp| match hp {
                        Some(hp) => Self::new(self.entity.clone(), hp, self.active),
                        None => None,
                    });
                constricted_hazards
            }
        }
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
