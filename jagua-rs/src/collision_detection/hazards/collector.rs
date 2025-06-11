use crate::collision_detection::hazards::filter::HazardFilter;
use crate::collision_detection::hazards::{HazKey, HazardEntity};
use slotmap::SecondaryMap;

/// Trait for structs that can track and store detected [`HazardEntity`]s.
/// Used in 'collision collection' queries to avoid having to repeatedly check hazards induced by an already detected entity.
pub trait HazardCollector: HazardFilter {
    fn contains(&self, hkey: HazKey) -> bool;

    fn insert(&mut self, hkey: HazKey, entity: HazardEntity);

    fn remove(&mut self, hkey: HazKey);

    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn len(&self) -> usize;

    fn iter(&self) -> impl Iterator<Item = HazKey>;
}

/// Basic implementation of a [`HazardCollector`] using a `SecondaryMap` to store hazards by their `HazKey`.
impl HazardCollector for SecondaryMap<HazKey, HazardEntity> {
    fn contains(&self, hkey: HazKey) -> bool {
        self.contains_key(hkey)
    }

    fn insert(&mut self, hkey: HazKey, entity: HazardEntity) {
        self.insert(hkey, entity);
    }

    fn remove(&mut self, hkey: HazKey) {
        self.remove(hkey);
    }

    fn len(&self) -> usize {
        self.len()
    }

    fn iter(&self) -> impl Iterator<Item = HazKey> {
        self.keys()
    }
}
