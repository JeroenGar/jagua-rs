use std::slice::Iter;

use crate::collision_detection::hazards::hazard_entity::HazardEntity;
use crate::collision_detection::quadtree::qt_hazard::QTHazard;
use crate::collision_detection::quadtree::qt_hazard_type::QTHazType;

//Vector of QTHazards, which always keep the Entire types at the front of the vector
#[derive(Clone, Debug)]
pub struct QTHazardVec {
    hazards: Vec<QTHazard>,
    strongest: Option<usize>,
}

impl QTHazardVec {
    pub fn new() -> Self {
        QTHazardVec {
            hazards: Vec::new(),
            strongest: None,
        }
    }

    pub fn add(&mut self, ch: QTHazard) {
        debug_assert!(self.hazards.iter().filter(|other| other.entity() == ch.entity() && matches!(ch.entity(), HazardEntity::Item(_))).count() == 0, "More than one hazard from same item entity in the vector! (This should never happen!)");
        match ch.haz_type() {
            QTHazType::Entire => {
                self.hazards.insert(0, ch);
            }
            QTHazType::Partial(_) => {
                self.hazards.push(ch);
            }
        }
        self.strongest = self.hazards.iter().position(|ch| ch.is_active());
    }

    pub fn remove(&mut self, hz: &HazardEntity) -> Option<QTHazard> {
        debug_assert!(self.hazards.iter().filter(|ch| ch.entity() == hz && matches!(ch.entity(), HazardEntity::Item(_))).count() <= 1, "More than one hazard from same item entity in the vector! (This should never happen!)");
        let pos = self.hazards.iter().position(|ch| ch.entity() == hz);
        let removed_hazard = match pos {
            Some(pos) => Some(self.hazards.remove(pos)),
            None => None,
        };
        self.strongest = self.hazards.iter().position(|ch| ch.is_active());
        removed_hazard
    }

    #[inline(always)]
    pub fn strongest(&self, ignored_entities: Option<&Vec<&HazardEntity>>) -> Option<&QTHazard> {
        match ignored_entities {
            None => {
                self.strongest.map(|pos| &self.hazards[pos])
            }
            Some(ignored_entities) => {
                self.strongest.map(|pos| {
                    let mut pos = pos;
                    while pos < self.hazards.len() && ignored_entities.contains(&self.hazards[pos].entity()) {
                        pos += 1;
                    }
                    if pos < self.hazards.len() {
                        Some(&self.hazards[pos])
                    } else {
                        None
                    }
                }).flatten()
            }
        }
    }

    pub fn get(&self, entity: &HazardEntity) -> Option<&QTHazard> {
        self.hazards.iter()
            .filter(|hz| hz.is_active())
            .find(|hz| hz.entity() == entity)
    }

    pub fn activate_hazard(&mut self, entity: &HazardEntity) -> bool {
        let hazard = self.hazards.iter_mut().find(|hz| hz.entity() == entity);
        match hazard {
            Some(hazard) => {
                debug_assert!(!hazard.is_active());
                hazard.activate();
                self.strongest = self.hazards.iter().position(|ch| ch.is_active());
                true
            }
            None => false
        }
    }

    pub fn deactivate_hazard(&mut self, entity: &HazardEntity) -> bool {
        let hazard = self.hazards.iter_mut().find(|hz| hz.entity() == entity);
        match hazard {
            Some(hazard) => {
                debug_assert!(hazard.is_active());
                hazard.deactivate();
                self.strongest = self.hazards.iter().position(|ch| ch.is_active());
                true
            }
            None => false
        }
    }

    pub fn active_iter(&self) -> impl Iterator<Item=&QTHazard> {
        self.hazards.iter().filter(|hz| hz.is_active())
    }

    pub fn all_iter(&self) -> Iter<QTHazard> {
        self.hazards.iter()
    }

    pub fn is_empty(&self) -> bool {
        self.hazards.is_empty()
    }

    pub fn len(&self) -> usize {
        self.hazards.len()
    }
}