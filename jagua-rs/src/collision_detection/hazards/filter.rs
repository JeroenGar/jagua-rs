use crate::collision_detection::hazards::{BasicHazardEntity, HazKey};
use slotmap::SecondaryMap;

/// Decides which [`Hazard`](super::Hazard)s a collision query ignores, by key or by the entity inducing them.
pub trait HazardFilter<E> {
    /// Whether the hazard registered under `key` and induced by `entity` is ignored.
    fn is_irrelevant(&self, key: HazKey, entity: &E) -> bool;
}

/// Deems hazards with specific [`HazKey`]'s as irrelevant.
#[derive(Clone, Debug)]
pub struct HazKeyFilter(pub SecondaryMap<HazKey, ()>);

impl HazKeyFilter {
    pub fn from_keys(keys: impl IntoIterator<Item = HazKey>) -> Self {
        HazKeyFilter(keys.into_iter().map(|k| (k, ())).collect())
    }
}

impl<E> HazardFilter<E> for HazKeyFilter {
    fn is_irrelevant(&self, key: HazKey, _: &E) -> bool {
        self.0.contains_key(key)
    }
}

/// Deems hazards induced by itself as irrelevant.
impl<E> HazardFilter<E> for HazKey {
    fn is_irrelevant(&self, key: HazKey, _: &E) -> bool {
        *self == key
    }
}

/// Ignores the hazards either filter ignores.
impl<E, A: HazardFilter<E>, B: HazardFilter<E>> HazardFilter<E> for (A, B) {
    fn is_irrelevant(&self, key: HazKey, entity: &E) -> bool {
        self.0.is_irrelevant(key, entity) || self.1.is_irrelevant(key, entity)
    }
}

/// Deems no hazards as irrelevant.
#[derive(Clone, Debug)]
pub struct NoFilter;

impl<E> HazardFilter<E> for NoFilter {
    fn is_irrelevant(&self, _: HazKey, _: &E) -> bool {
        false
    }
}

/// Ignores inferior quality zones that meet an item's minimum quality.
/// Holes and zones below the minimum keep blocking it.
#[derive(Clone, Copy, Debug)]
pub struct QualityZoneFilter {
    pub min_quality: usize,
}

impl HazardFilter<BasicHazardEntity> for QualityZoneFilter {
    fn is_irrelevant(&self, _: HazKey, entity: &BasicHazardEntity) -> bool {
        matches!(
            entity,
            BasicHazardEntity::InferiorQualityZone { quality, .. } if *quality >= self.min_quality
        )
    }
}
