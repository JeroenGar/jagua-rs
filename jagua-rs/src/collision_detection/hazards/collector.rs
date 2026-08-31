use crate::collision_detection::hazards::filter::HazardFilter;
use crate::collision_detection::hazards::{HazKey, HazardEntity};
use slotmap::{Key, SecondaryMap};

/// Trait for structs that can track and store detected [`Hazard`](crate::collision_detection::hazards::Hazard)s.
/// Used in 'collision collection' queries to avoid having to repeatedly check hazards induced by one that has already been detected.
pub trait HazardCollector: HazardFilter {
    fn contains_key(&self, hkey: HazKey) -> bool;

    fn contains_entity(&self, entity: &HazardEntity) -> bool {
        self.iter().any(|(_, e)| e == entity)
    }

    fn insert(&mut self, hkey: HazKey, entity: HazardEntity);

    fn remove_by_key(&mut self, hkey: HazKey);

    fn remove_by_entity(&mut self, entity: &HazardEntity) {
        let hkey = self
            .iter()
            .find(|(_, v)| *v == entity)
            .map(|(hkey, _)| hkey)
            .expect("HazardEntity not found in collector");
        self.remove_by_key(hkey);
    }

    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn len(&self) -> usize;

    fn iter(&self) -> impl Iterator<Item = (HazKey, &HazardEntity)>;

    fn keys(&self) -> impl Iterator<Item = HazKey> {
        self.iter().map(|(k, _)| k)
    }

    fn entities(&self) -> impl Iterator<Item = &HazardEntity> {
        self.iter().map(|(_, e)| e)
    }
}

/// A basic [`HazardCollector`] storing hazards by their [`HazKey`].
#[derive(Clone, Debug)]
pub struct BasicHazardCollector {
    detected: SecondaryMap<HazKey, HazardEntity>,
    /// Lossy negative filter for `detected`. The low six bits of each key select one of these 64
    /// bits. An unset bit proves that the key is absent; a set bit is only a possible match and is
    /// verified in `detected`. Bits stay set after removal because several keys may share a bit.
    /// Stale bits only cause an extra map lookup.
    detected_key_bits: u64,
}

impl BasicHazardCollector {
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

    pub fn insert(&mut self, hkey: HazKey, entity: HazardEntity) -> Option<HazardEntity> {
        self.detected_key_bits |= Self::key_bit(hkey);
        self.detected.insert(hkey, entity)
    }

    pub fn remove(&mut self, hkey: HazKey) -> Option<HazardEntity> {
        self.detected.remove(hkey)
    }

    pub fn retain(&mut self, mut predicate: impl FnMut(HazKey, &mut HazardEntity) -> bool) {
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

    pub fn iter(&self) -> impl Iterator<Item = (HazKey, &HazardEntity)> {
        self.detected.iter()
    }

    fn key_bit(hkey: HazKey) -> u64 {
        1 << (hkey.data().as_ffi() & 63)
    }
}

impl Default for BasicHazardCollector {
    fn default() -> Self {
        Self::new()
    }
}

impl HazardCollector for BasicHazardCollector {
    fn contains_key(&self, hkey: HazKey) -> bool {
        self.contains_key(hkey)
    }

    fn insert(&mut self, hkey: HazKey, entity: HazardEntity) {
        self.insert(hkey, entity);
    }

    fn remove_by_key(&mut self, hkey: HazKey) {
        self.remove(hkey);
    }

    fn len(&self) -> usize {
        self.len()
    }

    fn iter(&self) -> impl Iterator<Item = (HazKey, &HazardEntity)> {
        self.iter()
    }
}

impl<'a> IntoIterator for &'a BasicHazardCollector {
    type Item = (HazKey, &'a HazardEntity);
    type IntoIter = slotmap::secondary::Iter<'a, HazKey, HazardEntity>;

    fn into_iter(self) -> Self::IntoIter {
        self.detected.iter()
    }
}
