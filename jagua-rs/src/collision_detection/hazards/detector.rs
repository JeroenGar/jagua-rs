use crate::collision_detection::hazards::HazardEntity;
use crate::collision_detection::hazards::filter::HazardFilter;
use crate::entities::general::PItemKey;
use slotmap::SecondaryMap;

#[cfg(doc)]
use crate::collision_detection::hazards::Hazard;

/// Trait for structs that can track and store already detected [`Hazard`]s during querying.
/// Used during 'collision collection' to avoid repeatedly checking the same hazards.
/// Interface designed to mimic a Vec of [`HazardEntity`]s.
pub trait HazardDetector: HazardFilter {
    fn new() -> Self;
    fn clear(&mut self);
    fn contains(&self, haz: &HazardEntity) -> bool;

    fn push(&mut self, haz: HazardEntity);

    fn remove(&mut self, haz: &HazardEntity);

    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn len(&self) -> usize;

    fn iter(&self) -> impl Iterator<Item = &HazardEntity>;
}

/// Basic implementation of a [`HazardDetector`].
/// Hazards from [`HazardEntity::PlacedItem`] have instant lookups, the rest are stored in a Vec.
#[derive(Debug)]
pub struct BasicHazardDetector {
    pi_hazards: SecondaryMap<PItemKey, HazardEntity>,
    other: Vec<HazardEntity>,
}

impl HazardFilter for BasicHazardDetector {
    fn is_irrelevant(&self, haz: &HazardEntity) -> bool {
        self.contains(haz)
    }
}

impl HazardDetector for BasicHazardDetector {
    fn new() -> Self {
        BasicHazardDetector {
            pi_hazards: SecondaryMap::new(),
            other: Vec::new(),
        }
    }

    fn clear(&mut self) {
        self.pi_hazards.clear();
        self.other.clear();
    }

    fn contains(&self, haz: &HazardEntity) -> bool {
        match haz {
            HazardEntity::PlacedItem { pk, .. } => self.pi_hazards.contains_key(*pk),
            _ => self.other.iter().find(|&h| h == haz).is_some(),
        }
    }

    fn push(&mut self, haz: HazardEntity) {
        debug_assert!(!self.contains(&haz));
        match haz {
            HazardEntity::PlacedItem { pk, .. } => {
                self.pi_hazards.insert(pk, haz);
            }
            _ => self.other.push(haz),
        }
    }

    fn remove(&mut self, haz: &HazardEntity) {
        match haz {
            HazardEntity::PlacedItem { pk, .. } => {
                self.pi_hazards.remove(*pk);
            }
            _ => self.other.retain(|h| h != haz),
        }
    }

    fn len(&self) -> usize {
        self.pi_hazards.len() + self.other.len()
    }

    fn iter(&self) -> impl Iterator<Item = &HazardEntity> {
        self.pi_hazards
            .iter()
            .map(|(_, h)| h)
            .chain(self.other.iter().map(|h| h))
    }
}
