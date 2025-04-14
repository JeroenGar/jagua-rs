use crate::collision_detection::hazards::HazardEntity;
use crate::collision_detection::hazards::filter::HazardFilter;
use crate::entities::general::PItemKey;
use slotmap::SecondaryMap;

/// Trait for structs that can track and store detected [`HazardEntity`]s.
/// Used in 'collision collection' queries to avoid having to repeatedly check hazards induced by an already detected entity.
pub trait HazardDetector: HazardFilter {
    fn contains(&self, haz: &HazardEntity) -> bool;

    fn push(&mut self, haz: HazardEntity);

    fn remove(&mut self, haz: &HazardEntity);

    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn len(&self) -> usize;

    fn iter(&self) -> impl Iterator<Item = &HazardEntity>;
}

/// Implements [`HazardFilter`] for any type that implements [`HazardDetector`].
/// Any [`HazardEntity`]s that are already in the detector are considered irrelevant.
impl<T> HazardFilter for T
where
    T: HazardDetector,
{
    fn is_irrelevant(&self, haz: &HazardEntity) -> bool {
        self.contains(haz)
    }
}

/// Basic implementation of a [`HazardDetector`].
/// Hazards from [`HazardEntity::PlacedItem`] have instant lookups, the rest are stored in a Vec.
#[derive(Debug, Clone)]
pub struct BasicHazardDetector {
    pi_hazards: SecondaryMap<PItemKey, HazardEntity>,
    other: Vec<HazardEntity>,
}

impl BasicHazardDetector {
    pub fn new() -> Self {
        BasicHazardDetector {
            pi_hazards: SecondaryMap::new(),
            other: Vec::new(),
        }
    }

    pub fn clear(&mut self) {
        self.pi_hazards.clear();
        self.other.clear();
    }
}

impl HazardDetector for BasicHazardDetector {
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
