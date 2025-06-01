use crate::collision_detection::hazards::HazardEntity;
use crate::collision_detection::hazards::filter::HazardFilter;
use crate::collision_detection::quadtree::QTHazPresence;
use crate::collision_detection::quadtree::QTHazard;
use std::cmp::Ordering;

/// Vector of `QTHazard`s, which always remains sorted by activeness then presence.
/// <br>
/// This is a performance optimization to be able to quickly return the "strongest" hazard
/// Strongest meaning the first active hazard with the highest [`QTHazPresence`] (`Entire` > `Partial` > `None`)
#[derive(Clone, Debug, Default)]
pub struct QTHazardVec {
    hazards: Vec<QTHazard>,
    /// Number of active hazards in the vector
    n_active_hazards: usize,
    /// Number of edges from active hazards in the vector
    n_active_edges: usize,
}

impl QTHazardVec {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, haz: QTHazard) {
        debug_assert!(
            self.hazards
                .iter()
                .filter(|other| other.entity == haz.entity
                    && matches!(haz.entity, HazardEntity::PlacedItem { .. }))
                .count()
                == 0,
            "More than one hazard from same item entity in the vector! (This should never happen!)"
        );
        match self
            .hazards
            .binary_search_by(|probe| order_by_descending_strength(probe, &haz))
        {
            Ok(pos) | Err(pos) => {
                if haz.active {
                    self.n_active_hazards += 1;
                    self.n_active_edges += haz.n_edges();
                }
                self.hazards.insert(pos, haz);
            }
        }
    }

    pub fn remove(&mut self, haz_entity: HazardEntity) -> Option<QTHazard> {
        debug_assert!(
            self.hazards
                .iter()
                .filter(|ch| ch.entity == haz_entity
                    && matches!(ch.entity, HazardEntity::PlacedItem { .. }))
                .count()
                <= 1,
            "More than one hazard from same item entity in the vector! (This should never happen!)"
        );
        let pos = self.hazards.iter().position(|ch| ch.entity == haz_entity);
        match pos {
            Some(pos) => {
                let haz = self.hazards.remove(pos);
                if haz.active {
                    self.n_active_hazards -= 1;
                    self.n_active_edges -= haz.n_edges();
                }
                Some(haz)
            }
            None => None,
        }
    }

    #[inline(always)]
    /// Returns the strongest hazard (if any), meaning the first active hazard with the highest [QTHazPresence] (`Entire` > `Partial` > `None`)
    /// Ignores any hazards that are deemed irrelevant by the filter.
    pub fn strongest(&self, filter: &impl HazardFilter) -> Option<&QTHazard> {
        debug_assert!(assert_caches_correct(self));
        self.active_hazards()
            .iter()
            .find(|hz| !filter.is_irrelevant(&hz.entity))
    }

    pub fn get(&self, entity: HazardEntity) -> Option<&QTHazard> {
        self.hazards
            .iter()
            .filter(|hz| hz.active)
            .find(|hz| hz.entity == entity)
    }

    pub fn activate_hazard(&mut self, entity: HazardEntity) -> bool {
        match self.remove(entity) {
            Some(mut hazard) => {
                debug_assert!(!hazard.active);
                hazard.active = true;
                self.add(hazard);
                true
            }
            None => false,
        }
    }

    pub fn deactivate_hazard(&mut self, entity: HazardEntity) -> bool {
        match self.remove(entity) {
            Some(mut hazard) => {
                debug_assert!(hazard.active);
                hazard.active = false;
                self.add(hazard);
                true
            }
            None => false,
        }
    }

    pub fn active_hazards(&self) -> &[QTHazard] {
        debug_assert!(assert_caches_correct(self));
        &self.hazards[0..self.n_active_hazards]
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
        self.hazards
            .iter()
            .all(|hz| matches!(hz.presence, QTHazPresence::Entire))
    }

    pub fn n_active_hazards(&self) -> usize {
        debug_assert!(assert_caches_correct(self));
        self.n_active_hazards
    }

    pub fn n_active_edges(&self) -> usize {
        debug_assert!(assert_caches_correct(self));
        self.n_active_edges
    }
}

fn order_by_descending_strength(qth1: &QTHazard, qth2: &QTHazard) -> Ordering {
    let qth_presence_sortkey = |qth: &QTHazard| match qth.presence {
        QTHazPresence::None => 0,
        QTHazPresence::Partial(_) => 1,
        QTHazPresence::Entire => 2,
    };

    //sort in descending order of active (true > false) then by presence (Entire > Partial > None)
    qth1.active
        .cmp(&qth2.active)
        .then({
            let sk1 = qth_presence_sortkey(qth1);
            let sk2 = qth_presence_sortkey(qth2);
            sk1.cmp(&sk2)
        })
        .reverse()
}

fn assert_caches_correct(qthazard_vec: &QTHazardVec) -> bool {
    assert_eq!(
        qthazard_vec.hazards.iter().filter(|hz| hz.active).count(),
        qthazard_vec.n_active_hazards,
        "Active hazards count is not correct!"
    );
    assert!(
        qthazard_vec
            .hazards
            .windows(2)
            .all(|w| order_by_descending_strength(&w[0], &w[1]) != Ordering::Greater),
        "Hazards are not sorted correctly!"
    );
    assert_eq!(
        qthazard_vec
            .hazards
            .iter()
            .filter(|hz| hz.active)
            .map(|hz| hz.n_edges())
            .sum::<usize>(),
        qthazard_vec.n_active_edges,
        "Active edges count is not correct!"
    );
    true
}
