//Constricting CollisionHazards can be a costly operation.
//Especially the cases where constricting a partial hazard results in 0 edge intersections.
//In this situation, the hazard either becomes an "entire" hazard or stops existing (absent).
//Determining if the rectangle is inside or outside requires point-in-polygon tests (expensive).
//However, child nodes where the hazard is "entire" and nodes where the hazard is absent can never be neighbors.
//There always must be a "partial" hazard (aka edges) separating them.
//Therefore, if a constriction leads to 0 intersections and a neighbor is "entire" hazard,
//the resulting hazard must also be "entire". Same story with absent hazard.

//This cache saves previously calculated Absent and Entire inclusion for QTNode siblings,
//allowing is us to save on point-in-polygon tests where possible.


use crate::collision_detection::quadtree::qt_hazard::QTHazard;
use crate::collision_detection::quadtree::qt_hazard_type::QTHazPresence;

// QTNode children array layout:
// 0 -- 1
// |    |
// 2 -- 3
const CHILD_NEIGHBORS: [[usize; 2]; 4] = [[1, 2], [0, 3], [0, 3], [1, 2]];

pub struct ConstrictCache([Option<CCEntry>; 4]);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum CCEntry {
    AbsentHazard,
    EntireHazard,
}

impl ConstrictCache {
    pub fn new() -> Self {
        Self([None; 4])
    }

    pub fn fetch(&self, child_index: usize) -> Option<CCEntry> {
        let n0 = CHILD_NEIGHBORS[child_index][0];
        let n1 = CHILD_NEIGHBORS[child_index][1];

        if let Some(entry) = self.0[n0] {
            return Some(entry);
        }
        if let Some(entry) = self.0[n1] {
            return Some(entry);
        }
        None
    }

    pub fn store(&mut self, child_index: usize, hazard: &Option<QTHazard>) {
        match hazard {
            Some(hazard) => {
                match hazard.haz_type() {
                    QTHazPresence::Partial(_) => (),
                    QTHazPresence::Entire => self.0[child_index] = Some(CCEntry::EntireHazard),
                }
            }
            None => self.0[child_index] = Some(CCEntry::AbsentHazard),
        };
    }
}