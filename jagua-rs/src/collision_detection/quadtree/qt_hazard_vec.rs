use std::cmp::Ordering;

use crate::collision_detection::hazard::HazardEntity;
use crate::collision_detection::quadtree::qt_hazard::QTHazard;
use crate::collision_detection::quadtree::qt_hazard::QTHazPresence;

/// Vector of `QTHazard`s, which always remains sorted by activeness then presence.
/// <br>
/// This is a performance optimization to be able to quickly return the "strongest" hazard
/// Strongest meaning the first active hazard with the highest presence (`Entire` > `Partial` > `None`)
#[derive(Clone, Debug)]
pub struct QTHazardVec {
    /// Sorted in descending order by strongest
    hazards: Vec<QTHazard>,
    n_active: usize,
}

impl QTHazardVec {
    pub fn new() -> Self {
        QTHazardVec {
            hazards: Vec::new(),
            n_active: 0,
        }
    }

    pub fn add(&mut self, haz: QTHazard) {
        debug_assert!(self.hazards.iter().filter(|other| other.entity == haz.entity && matches!(haz.entity, HazardEntity::PlacedItem(_))).count() == 0, "More than one hazard from same item entity in the vector! (This should never happen!)");
        match self.hazards.binary_search_by(|probe| order_by_descending_strength(probe, &haz)) {
            Ok(pos) | Err(pos) => {
                self.n_active += haz.active as usize;
                self.hazards.insert(pos, haz);
            }
        }
    }

    pub fn remove(&mut self, haz_entity: &HazardEntity) -> Option<QTHazard> {
        debug_assert!(self.hazards.iter().filter(|ch| &ch.entity == haz_entity && matches!(ch.entity, HazardEntity::PlacedItem(_))).count() <= 1, "More than one hazard from same item entity in the vector! (This should never happen!)");
        let pos = self.hazards.iter().position(|ch| &ch.entity == haz_entity);
        let removed_hazard = match pos {
            Some(pos) => {
                let haz = self.hazards.remove(pos);
                self.n_active -= haz.active as usize;
                Some(haz)
            }
            None => None,
        };
        removed_hazard
    }

    #[inline(always)]
    pub fn strongest(&self, irrelevant_hazards: &[HazardEntity]) -> Option<&QTHazard> {
        debug_assert!(self.hazards.iter().filter(|hz| hz.active).count() == self.n_active, "Active hazards count is not correct!");
        debug_assert!(self.hazards.windows(2).all(|w| order_by_descending_strength(&w[0], &w[1]) != Ordering::Greater), "Hazards are not sorted correctly!");
        match (self.n_active, irrelevant_hazards) {
            //no active hazards
            (0, _) => None,
            //no ignored entities
            (_, []) => Some(&self.hazards[0]),
            //Some ignored entities
            (_, _) => self.hazards[0..self.n_active].iter().find(|hz| !irrelevant_hazards.contains(&hz.entity)),
        }
    }

    pub fn get(&self, entity: &HazardEntity) -> Option<&QTHazard> {
        self.hazards.iter()
            .filter(|hz| hz.active)
            .find(|hz| &hz.entity == entity)
    }

    pub fn activate_hazard(&mut self, entity: &HazardEntity) -> bool {
        match self.hazards.iter_mut().position(|hz| &hz.entity == entity) {
            Some(index) => {
                let mut hazard = self.hazards.remove(index);
                debug_assert!(!hazard.active);
                hazard.active = true;
                self.add(hazard);
                true
            }
            None => false
        }
    }

    pub fn deactivate_hazard(&mut self, entity: &HazardEntity) -> bool {
        match self.hazards.iter_mut().position(|hz| &hz.entity == entity) {
            Some(index) => {
                let mut hazard = self.hazards.remove(index);
                debug_assert!(hazard.active);
                hazard.active = false;
                self.n_active -= 1;
                self.add(hazard);
                true
            }
            None => false
        }
    }

    pub fn active_hazards(&self) -> &[QTHazard] {
        &self.hazards[0..self.n_active]
    }

    pub fn all_hazards(&self) -> &[QTHazard] {
        &self.hazards
    }

    pub fn is_empty(&self) -> bool {
        self.hazards.is_empty()
    }

    pub fn len(&self) -> usize {
        self.hazards.len()
    }

    pub fn has_only_entire_hazards(&self) -> bool {
        self.hazards.iter().all(|hz| matches!(hz.presence,QTHazPresence::Entire))
    }
}

fn order_by_descending_strength(qth1: &QTHazard, qth2: &QTHazard) -> Ordering {
//sort in descending order of active (true > false) then by presence (Entire > Partial > None)
    match qth1.active.cmp(&qth2.active).reverse() {
        Ordering::Equal => {
            let (p1, p2): (u8, u8) = ((&qth1.presence).into(), (&qth2.presence).into());
            p1.cmp(&p2).reverse()
        }
        other => other,
    }
}