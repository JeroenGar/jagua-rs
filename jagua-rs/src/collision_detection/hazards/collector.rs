use crate::collision_detection::hazards::filter::HazardFilter;
use crate::collision_detection::hazards::{HazKey, HazardEntity};
use slotmap::SecondaryMap;

/// Trait for structs that can track and store detected [`Hazard`](crate::collision_detection::hazards::Hazard)s.
/// Used in 'collision collection' queries to avoid having to repeatedly check hazards induced by one that has already been detected.
pub trait HazardCollector: HazardFilter {
    fn contains(&self, hkey: HazKey) -> bool;

    fn insert(&mut self, hkey: HazKey, entity: HazardEntity);

    fn remove_by_key(&mut self, hkey: HazKey);

    fn remove_by_entity(&mut self, entity: &HazardEntity);

    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn len(&self) -> usize;

    fn iter(&self) -> impl Iterator<Item = (HazKey, &HazardEntity)>;

    fn keys(&self) -> impl Iterator<Item = HazKey> {
        self.iter().map(|(k, _)| k)
    }
}

/// Basic implementation of a [`HazardCollector`] using a `SecondaryMap` to store hazards by their `HazKey`.
impl HazardCollector for SecondaryMap<HazKey, HazardEntity> {
    fn contains(&self, hkey: HazKey) -> bool {
        self.contains_key(hkey)
    }

    fn insert(&mut self, hkey: HazKey, entity: HazardEntity) {
        self.insert(hkey, entity);
    }

    fn remove_by_key(&mut self, hkey: HazKey) {
        self.remove(hkey);
    }

    fn remove_by_entity(&mut self, entity: &HazardEntity) {
        if let Some((hkey, _)) = self.iter().find(|(_, v)| *v == entity) {
            self.remove(hkey);
        }
    }

    fn len(&self) -> usize {
        self.len()
    }

    fn iter(&self) -> impl Iterator<Item = (HazKey, &HazardEntity)> {
        self.iter()
    }
}
