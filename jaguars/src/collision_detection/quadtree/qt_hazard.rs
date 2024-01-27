use crate::collision_detection::hazards::hazard::Hazard;
use crate::collision_detection::hazards::hazard_entity::HazardEntity;
use crate::collision_detection::quadtree::qt_hazard_type::QTHazPresence;
use crate::collision_detection::quadtree::qt_partial_hazard::{EdgeIndices, QTPartialHazard};
use crate::geometry::geo_enums::{GeoPosition, GeoRelation};
use crate::geometry::geo_traits::{CollidesWith, Shape};
use crate::geometry::primitives::aa_rectangle::AARectangle;

// QTNode children array layout:
// 0 -- 1
// |    |
// 2 -- 3
const CHILD_NEIGHBORS: [[usize; 2]; 4] = [[1, 2], [0, 3], [0, 3], [1, 2]];

//Hazards in a QTNode
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct QTHazard {
    entity: HazardEntity,
    presence: QTHazPresence,
    active: bool,
}

impl From<&Hazard> for QTHazard {
    fn from(hazard: &Hazard) -> Self {
        Self {
            entity: hazard.entity().clone(),
            presence: QTHazPresence::Partial(hazard.into()),
            active: true,
        }
    }
}

impl QTHazard {
    fn new(entity: HazardEntity, presence: QTHazPresence, active: bool) -> Option<Self> {
        match presence {
            QTHazPresence::None => None,
            _ => Some(Self { entity, presence, active })
        }
    }

    /// This function returns a constricted version of QTHazard for a smaller rectangle.
    /// If the hazard is not present in the rectangle, None is returned.
    pub fn constrict(&self, quadrants: [&AARectangle; 4]) -> [Option<Self>; 4] {
        match &self.presence {
            QTHazPresence::None => [None, None, None, None],
            QTHazPresence::Entire => [Some(self.clone()), Some(self.clone()), Some(self.clone()), Some(self.clone())],
            QTHazPresence::Partial(partial_haz) => {
                //check the bbox of the hazard with the bboxes of the quadrants
                let haz_bbox = partial_haz.shape().bbox();
                let haz_quad_relations = [
                    haz_bbox.relation_to(quadrants[0]),
                    haz_bbox.relation_to(quadrants[1]),
                    haz_bbox.relation_to(quadrants[2]),
                    haz_bbox.relation_to(quadrants[3]),
                ];
                let mut constricted_presence = [None, None, None, None];

                if let Some(quad_index) = haz_quad_relations.iter().position(|r| r == &GeoRelation::Enclosed) {
                    //if the hazard is entirely inside one of the quadrants, clone the current hazard for that quadrant and set all the rest to none
                    //ensure all the other quadrants are disjoint
                    debug_assert!(haz_quad_relations.iter().enumerate()
                        .filter(|(i, _)| *i != quad_index)
                        .all(|(_, r)| r == &GeoRelation::Disjoint));
                    constricted_presence[quad_index] = Some(self.presence.clone());
                } else {
                    //the hazard is partially active in multiple quadrants, find them
                    let shape = partial_haz.shape();
                    let mut check_collisions_with_quadrants = |edge_index: usize| {
                        let edge = shape.get_edge(edge_index);
                        for quad_index in 0..4 {
                            let quadrant = quadrants[quad_index];
                            if quadrant.collides_with(&edge) {
                                let constricted_haz_presence = constricted_presence[quad_index].get_or_insert(
                                    QTHazPresence::Partial(
                                        QTPartialHazard::new(
                                            partial_haz.shape(),
                                            partial_haz.presence(),
                                            EdgeIndices::Some(vec![]),
                                        )
                                    )
                                );
                                match constricted_haz_presence {
                                    QTHazPresence::Partial(constricted_haz) => {
                                        constricted_haz.add_edge_index(edge_index);
                                    }
                                    _ => panic!("constricted hazard is not partial"),
                                }
                            }
                        }
                    };

                    match partial_haz.edge_indices() {
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
                        match constricted_presence[i] {
                            Some(_) => (), //already handled
                            None => {
                                //check if a neighbor is already resolved
                                let [n_0, n_1] = CHILD_NEIGHBORS[i];
                                constricted_presence[i] = match (&constricted_presence[n_0], &constricted_presence[n_1]) {
                                    (Some(QTHazPresence::Entire), _) | (_, Some(QTHazPresence::Entire)) => {
                                        Some(QTHazPresence::Entire)
                                    }
                                    (Some(QTHazPresence::None), _) | (_, Some(QTHazPresence::None)) => {
                                        Some(QTHazPresence::None)
                                    }
                                    _ => {
                                        let point_to_test = quadrants[i].centroid();
                                        match (partial_haz.presence(), shape.collides_with(&point_to_test)) {
                                            (GeoPosition::Interior, true) => Some(QTHazPresence::Entire),
                                            (GeoPosition::Exterior, false) => Some(QTHazPresence::Entire),
                                            _ => Some(QTHazPresence::None),
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                //convert them to QTHazards
                let constricted_hazards: [Option<Self>; 4] = constricted_presence
                    .map(|hp|
                        match hp {
                            Some(hp) => Self::new(self.entity.clone(), hp, self.active),
                            None => None
                        }
                    );
                constricted_hazards
            }
        }
    }

    pub fn entity(&self) -> &HazardEntity {
        &self.entity
    }

    pub fn haz_presence(&self) -> &QTHazPresence {
        &self.presence
    }

    pub fn activate(&mut self) {
        self.active = true;
    }

    pub fn deactivate(&mut self) {
        self.active = false;
    }

    pub fn is_active(&self) -> bool {
        self.active
    }
}