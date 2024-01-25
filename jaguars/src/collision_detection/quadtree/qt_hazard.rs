use std::borrow::Borrow;

use crate::collision_detection::hazards::hazard::Hazard;
use crate::collision_detection::hazards::hazard_entity::HazardEntity;
use crate::collision_detection::quadtree::constrict_cache::{CCEntry, ConstrictCache};
use crate::collision_detection::quadtree::edge_interval_iter::EdgeIntervalIterator;
use crate::collision_detection::quadtree::qt_hazard_type::QTHazType;
use crate::collision_detection::quadtree::qt_partial_hazard::QTPartialHazard;
use crate::geometry::primitives::aa_rectangle::{AARectangle};
use crate::geometry::geo_traits::{CollidesWith, Shape};
use crate::geometry::primitives::point::Point;
use crate::geometry::geo_enums::{GeoPosition, GeoRelation};

//CollisionHazards define possible causes for a collision
//Associated with a QTNode
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct QTHazard {
    entity: HazardEntity,
    //Cause of the hazard
    haz_type: QTHazType,
    //Encompasses either the entire QTNode or only partially
    active: bool,
}

impl From<&Hazard> for QTHazard {
    fn from(hazard: &Hazard) -> Self {
        Self {
            entity: hazard.entity().clone(),
            haz_type: QTHazType::Partial(hazard.into()),
            active: true,
        }
    }
}

impl QTHazard {
    fn new(entity: HazardEntity, haz_type: QTHazType, active: bool) -> Self {
        Self { entity, haz_type, active }
    }

    /// This function returns a constricted version of hazard for a smaller rectangle.
    /// If the hazard is not present in the rectangle, None is returned.
    pub fn constrict(&self, rect: &AARectangle, qt_node_index: usize, cache: &ConstrictCache) -> Option<Self> {
        //Couple of cases that are easy to resolve
        if let QTHazType::Partial(partial_hazard) = &self.haz_type {
            if partial_hazard.intervals().len() == 1 && partial_hazard.intervals()[0] == (0, 0) &&
                partial_hazard.position() == GeoPosition::Interior {
                //If the hazard is an interior hazard and encompasses the entire shape
                let bbox = partial_hazard.shape().upgrade().expect("polygon reference is not alive").bbox();

                //If its bounding box is either completely inside the rectangle or completely unrelated we can return early
                let relation = rect.relation_to(&bbox);
                match relation {
                    GeoRelation::Disjoint => return None,
                    GeoRelation::Surrounding => return Some(self.clone()),
                    _ => {}
                };
            }
        }

        match &self.haz_type {
            QTHazType::Entire => Some(self.clone()), //Entire hazards always remain entire inclusion when constricted
            QTHazType::Partial(partial_hazard) => {
                //Partial hazards can become either no hazard, entire or partial inclusion when constricted
                let mut child_intervals = Vec::with_capacity(partial_hazard.intervals().len());
                let shape = partial_hazard.shape().upgrade().expect("polygon reference is not alive");
                let n_points = shape.number_of_points();

                //Test all the intervals of edges active in the original hazard
                for interval in partial_hazard.intervals() {
                    //every existing interval could potentially split in multiple intervals due to the contraction of the bbox
                    let mut child_interval_start: Option<usize> = None;
                    for (i, j) in EdgeIntervalIterator::new(*interval, n_points) {
                        let edge = shape.get_edge(i, j);
                        match (child_interval_start, rect.collides_with(&edge)) {
                            (None, true) => {
                                //inactive -> active
                                child_interval_start = Some(i);
                            }
                            (Some(start), false) => {
                                //active -> inactive
                                child_intervals.push((start, i));
                                child_interval_start = None;
                            }
                            (Some(_), true) | (None, false) => {
                                //active -> active or inactive -> inactive
                            }
                        }
                    }
                    if let Some(start) = child_interval_start {
                        //if the child interval was not closed before the end of the interval
                        match child_intervals.get_mut(0) {
                            None => {
                                //no previous child intervals have been detected, so end the current one at the end of the interval
                                child_intervals.push((start, interval.1));
                            }
                            Some(first_child_interval) => {
                                if first_child_interval.0 == interval.0 {
                                    //first child interval start at the front of the interval, merge first and last children
                                    first_child_interval.0 = start;
                                } else {
                                    child_intervals.push((start, interval.1));
                                }
                            }
                        }
                    }
                }

                match child_intervals.is_empty() {
                    true => {
                        //rectangle does not intersect with any of the edges
                        //meaning is either entirely inside or outside the shape
                        let entire_or_absent_hazard = match cache.fetch(qt_node_index) {
                            Some(cache_entry) => cache_entry,
                            None => match (partial_hazard.position(), shape.collides_with(&rect.centroid())) {
                                (GeoPosition::Interior, true) => {
                                    debug_assert!(rect.corners().iter().all(|c| {
                                        let middle = Point((rect.centroid().0 + c.0) / 2.0, (rect.centroid().1 + c.1) / 2.0);
                                        shape.collides_with(&middle)
                                    }), "inconsistent pip test, shape: {:?},\n point: {:?},\n corners {:?}", shape, rect.centroid(), rect.corners());
                                    CCEntry::EntireHazard
                                }
                                (GeoPosition::Exterior, false) => {
                                    debug_assert!(rect.corners().iter().all(|c| {
                                        let middle = Point((rect.centroid().0 + c.0) / 2.0, (rect.centroid().1 + c.1) / 2.0);
                                        !shape.collides_with(&middle)
                                    }), "inconsistent pip test, shape: {:?},\n point: {:?},\n corners {:?}", shape, rect.centroid(), rect.corners());
                                    CCEntry::EntireHazard
                                }
                                (_, _) => CCEntry::AbsentHazard
                            }
                        };

                        match entire_or_absent_hazard {
                            CCEntry::EntireHazard => {
                                Some(QTHazard::new(
                                    self.entity.clone(),
                                    QTHazType::Entire,
                                    self.active,
                                ))
                            }
                            CCEntry::AbsentHazard => None,
                        }
                    }
                    false => {
                        //create a new collision hazard with the child intervals
                        Some(QTHazard::new(
                            self.entity.clone(),
                            QTHazType::Partial(
                                QTPartialHazard::new(
                                    partial_hazard.shape().clone(),
                                    partial_hazard.position(),
                                    child_intervals,
                                )),
                            self.active,
                        ))
                    }
                }
            }
        }
    }

    pub fn entity(&self) -> &HazardEntity {
        &self.entity
    }

    pub fn haz_type(&self) -> &QTHazType {
        &self.haz_type
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