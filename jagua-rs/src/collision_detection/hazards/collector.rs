use crate::collision_detection::hazards::{BasicHazardEntity, HazKey, HazardEntity};
use slotmap::{Key, SecondaryMap};

/// Trait for structs that can track and store detected [`Hazard`](crate::collision_detection::hazards::Hazard)s.
/// Used in 'collision collection' queries to avoid having to repeatedly check hazards induced by one that has already been detected.
pub trait HazardCollector {
    /// The entities the collected hazards are induced by.
    type Entity: HazardEntity;

    fn contains_key(&self, hkey: HazKey) -> bool;

    fn contains_entity(&self, entity: &Self::Entity) -> bool {
        self.iter().any(|(_, e)| e == entity)
    }

    fn insert(&mut self, hkey: HazKey, entity: Self::Entity);

    fn remove_by_key(&mut self, hkey: HazKey);

    fn remove_by_entity(&mut self, entity: &Self::Entity) {
        let hkey = self
            .iter()
            .find(|(_, v)| *v == entity)
            .map(|(hkey, _)| hkey)
            .expect("entity not found in collector");
        self.remove_by_key(hkey);
    }

    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn len(&self) -> usize;

    fn iter(&self) -> impl Iterator<Item = (HazKey, &Self::Entity)>;

    fn keys(&self) -> impl Iterator<Item = HazKey> {
        self.iter().map(|(k, _)| k)
    }

    fn entities(&self) -> impl Iterator<Item = &Self::Entity> {
        self.iter().map(|(_, e)| e)
    }
}

/// A basic [`HazardCollector`] storing hazards by their [`HazKey`].
#[derive(Clone, Debug)]
pub struct BasicHazardCollector<E = BasicHazardEntity> {
    detected: SecondaryMap<HazKey, E>,
    /// Lossy negative filter for `detected`. The low six bits of each key select one of these 64
    /// bits. An unset bit proves that the key is absent; a set bit is only a possible match and is
    /// verified in `detected`. Bits stay set after removal because several keys may share a bit.
    /// Stale bits only cause an extra map lookup.
    detected_key_bits: u64,
}

impl<E: HazardEntity> BasicHazardCollector<E> {
    #[must_use]
    pub fn new() -> Self {
        Self::with_capacity(0)
    }

    #[must_use]
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            detected: SecondaryMap::with_capacity(capacity),
            detected_key_bits: 0,
        }
    }

    pub fn clear(&mut self) {
        self.detected.clear();
        self.detected_key_bits = 0;
    }

    #[must_use]
    pub fn contains_key(&self, hkey: HazKey) -> bool {
        self.detected_key_bits & Self::key_bit(hkey) != 0 && self.detected.contains_key(hkey)
    }

    pub fn insert(&mut self, hkey: HazKey, entity: E) -> Option<E> {
        self.detected_key_bits |= Self::key_bit(hkey);
        self.detected.insert(hkey, entity)
    }

    pub fn remove(&mut self, hkey: HazKey) -> Option<E> {
        self.detected.remove(hkey)
    }

    pub fn retain(&mut self, mut predicate: impl FnMut(HazKey, &mut E) -> bool) {
        self.detected.retain(|key, entity| predicate(key, entity));
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.detected.is_empty()
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.detected.len()
    }

    pub fn iter(&self) -> impl Iterator<Item = (HazKey, &E)> {
        self.detected.iter()
    }

    fn key_bit(hkey: HazKey) -> u64 {
        1 << (hkey.data().as_ffi() & 63)
    }
}

impl<E: HazardEntity> Default for BasicHazardCollector<E> {
    fn default() -> Self {
        Self::new()
    }
}

impl<E: HazardEntity> HazardCollector for BasicHazardCollector<E> {
    type Entity = E;

    fn contains_key(&self, hkey: HazKey) -> bool {
        self.contains_key(hkey)
    }

    fn insert(&mut self, hkey: HazKey, entity: E) {
        self.insert(hkey, entity);
    }

    fn remove_by_key(&mut self, hkey: HazKey) {
        self.remove(hkey);
    }

    fn len(&self) -> usize {
        self.len()
    }

    fn iter(&self) -> impl Iterator<Item = (HazKey, &E)> {
        self.iter()
    }
}

impl<'a, E> IntoIterator for &'a BasicHazardCollector<E> {
    type Item = (HazKey, &'a E);
    type IntoIter = slotmap::secondary::Iter<'a, HazKey, E>;

    fn into_iter(self) -> Self::IntoIter {
        self.detected.iter()
    }
}
