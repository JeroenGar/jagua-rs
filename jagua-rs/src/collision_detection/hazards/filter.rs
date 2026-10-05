use crate::collision_detection::hazards::collector::HazardCollector;
use crate::collision_detection::hazards::{BasicHazardEntity, HazKey, Hazard};
use slotmap::{SecondaryMap, SlotMap};

/// Decides which [`Hazard`]s a collision query ignores, by key or by the entity inducing them.
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

    /// Creates a filter that deems all inferior quality zones above or at a certain quality as irrelevant.
    #[must_use]
    pub fn from_irrelevant_qzones(
        required_quality: usize,
        haz_map: &SlotMap<HazKey, Hazard>,
    ) -> Self {
        HazKeyFilter(
            haz_map
                .iter()
                .filter_map(|(hkey, h)| {
                    match h.entity {
                        BasicHazardEntity::InferiorQualityZone { quality, .. }
                            if quality >= required_quality =>
                        {
                            // Zones meeting the item's minimum quality do not block it.
                            Some((hkey, ()))
                        }
                        _ => None,
                    }
                })
                .collect(),
        )
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

/// Deems no hazards as irrelevant.
#[derive(Clone, Debug)]
pub struct NoFilter;

impl<E> HazardFilter<E> for NoFilter {
    fn is_irrelevant(&self, _: HazKey, _: &E) -> bool {
        false
    }
}

/// Implements [`HazardFilter`] for any type that implements [`HazardCollector`].
/// Any hazards that are already in the collector are considered irrelevant.
impl<T: HazardCollector> HazardFilter<T::Entity> for T {
    fn is_irrelevant(&self, key: HazKey, _: &T::Entity) -> bool {
        self.contains_key(key)
    }
}
