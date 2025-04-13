use crate::collision_detection::hazards::{Hazard, HazardEntity};
use itertools::Itertools;

/// Trait for filters which ignore all [`Hazard`]s induced by specific [`HazardEntity`]s during querying.
pub trait HazardFilter {
    fn is_irrelevant(&self, entity: &HazardEntity) -> bool;
}

/// Returns the entities that are deemed irrelevant by the specified `HazardFilter`.
pub fn generate_irrelevant_hazards<'a>(
    filter: &impl HazardFilter,
    hazards: impl Iterator<Item = &'a Hazard>,
) -> Vec<HazardEntity> {
    hazards
        .filter_map(|h| match filter.is_irrelevant(&h.entity) {
            true => Some(h.entity),
            false => None,
        })
        .collect_vec()
}

/// Deems all hazards induced by the `Bin` as irrelevant.
#[derive(Clone)]
pub struct BinHazardFilter;

/// Deems hazards induced by `QualityZone`s above a cutoff quality as irrelevant.
#[derive(Clone, Debug)]
pub struct QZHazardFilter(pub usize);

/// Deems hazards induced by specific entities as irrelevant.
pub struct EntityHazardFilter(pub Vec<HazardEntity>);

/// Combines multiple `HazardFilter`s into a single filter.
pub struct CombinedHazardFilter<'a> {
    pub filters: Vec<Box<&'a dyn HazardFilter>>,
}

impl HazardFilter for BinHazardFilter {
    fn is_irrelevant(&self, entity: &HazardEntity) -> bool {
        match entity {
            HazardEntity::PlacedItem { .. } => false,
            HazardEntity::BinExterior => true,
            HazardEntity::BinHole { .. } => true,
            HazardEntity::InferiorQualityZone { .. } => true,
        }
    }
}

impl<'a> HazardFilter for CombinedHazardFilter<'a> {
    fn is_irrelevant(&self, entity: &HazardEntity) -> bool {
        self.filters.iter().any(|f| f.is_irrelevant(entity))
    }
}

impl HazardFilter for EntityHazardFilter {
    fn is_irrelevant(&self, entity: &HazardEntity) -> bool {
        self.0.contains(entity)
    }
}

impl HazardFilter for QZHazardFilter {
    fn is_irrelevant(&self, entity: &HazardEntity) -> bool {
        match entity {
            HazardEntity::InferiorQualityZone { quality, .. } => *quality >= self.0,
            _ => false,
        }
    }
}

impl HazardFilter for &[HazardEntity] {
    fn is_irrelevant(&self, haz: &HazardEntity) -> bool {
        self.contains(&haz)
    }
}
