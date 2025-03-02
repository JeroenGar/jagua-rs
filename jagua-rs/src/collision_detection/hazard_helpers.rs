use crate::collision_detection::hazard::HazardEntity;
use crate::entities::placed_item::PItemKey;
use slotmap::SecondaryMap;

/// Trait for structs that can be used to filter out irrelevant hazards.
/// Basically only used in [`QTHazardVec::strongest()`](crate::collision_detection::quadtree::qt_hazard_vec::QTHazardVec::strongest).
pub trait HazardIgnorer {
    fn is_irrelevant(&self, haz: &HazardEntity) -> bool;
}

/// Trait for structs that can track and store already detected hazards.
/// Interface made to mimic a Vec of `HazardEntity`s.
pub trait HazardDetector: HazardIgnorer {
    fn contains(&self, haz: &HazardEntity) -> bool;

    fn push(&mut self, haz: HazardEntity);

    fn remove(&mut self, haz: &HazardEntity);

    fn len(&self) -> usize;

    fn iter(&self) -> impl Iterator<Item = &HazardEntity>;
}

/// Datastructure to register which Hazards are detected during collision collection.
/// Hazards caused by placed items have instant lookups, the others are stored in a Vec.
/// It also stores an index for each hazard, which can be used to determine the order in which they were detected.
#[derive(Debug)]
pub struct DetectionMap {
    pi_hazards: SecondaryMap<PItemKey, (HazardEntity, usize)>,
    other: Vec<(HazardEntity, usize)>,
    idx_counter: usize,
}

impl DetectionMap {
    pub fn new() -> Self {
        DetectionMap {
            idx_counter: 0,
            pi_hazards: SecondaryMap::new(),
            other: Vec::new(),
        }
    }

    pub fn clear(&mut self) {
        self.idx_counter = 0;
        self.pi_hazards.clear();
        self.other.clear();
    }
}

impl HazardDetector for DetectionMap {
    fn contains(&self, haz: &HazardEntity) -> bool {
        match haz {
            HazardEntity::PlacedItem { pk, .. } => self.pi_hazards.contains_key(*pk),
            _ => self.other.iter().find(|(h, _)| h == haz).is_some(),
        }
    }

    fn push(&mut self, haz: HazardEntity) {
        match haz {
            HazardEntity::PlacedItem { pk, .. } => {
                self.pi_hazards.insert(pk, (haz, self.idx_counter));
            }
            _ => self.other.push((haz, self.idx_counter)),
        }
        self.idx_counter += 1;
    }

    fn remove(&mut self, haz: &HazardEntity) {
        match haz {
            HazardEntity::PlacedItem { pk, .. } => {
                self.pi_hazards.remove(*pk);
            }
            _ => self.other.retain(|(h, _)| h != haz),
        }
    }

    fn len(&self) -> usize {
        self.pi_hazards.len() + self.other.len()
    }

    fn iter(&self) -> impl Iterator<Item = &HazardEntity> {
        self.pi_hazards
            .iter()
            .map(|(_, (h, _))| h)
            .chain(self.other.iter().map(|(h, _)| h))
    }
}

impl DetectionMap {
    pub fn iter_with_index(&self) -> impl Iterator<Item = &(HazardEntity, usize)> {
        self.pi_hazards.values().chain(self.other.iter())
    }

    pub fn index_counter(&self) -> usize {
        self.idx_counter
    }
}

impl HazardIgnorer for DetectionMap {
    fn is_irrelevant(&self, haz: &HazardEntity) -> bool {
        self.contains(haz)
    }
}

impl HazardIgnorer for &[HazardEntity] {
    fn is_irrelevant(&self, haz: &HazardEntity) -> bool {
        self.contains(&haz)
    }
}
