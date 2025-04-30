use crate::collision_detection::hazards::HazardEntity;

/// Trait for filters to ignore all [`Hazard`](crate::collision_detection::hazards::Hazard)s induced by specific [`HazardEntity`]s.
/// Enables collision queries to ignore specific hazards during the check.
pub trait HazardFilter {
    fn is_irrelevant(&self, entity: &HazardEntity) -> bool;
}

/// Deems no hazards as irrelevant.
#[derive(Clone, Debug)]
pub struct NoHazardFilter;

/// Deems all hazards induced by the [`Bin`](crate::entities::general::Bin) as irrelevant.
#[derive(Clone, Debug)]
pub struct BinHazardFilter;

/// Deems hazards induced by [`InferiorQualityZone`](crate::entities::general::InferiorQualityZone)s above a cutoff quality as irrelevant.
#[derive(Clone, Debug)]
pub struct QZHazardFilter(pub usize);

/// Deems hazards induced by specific [`HazardEntity`]s as irrelevant.
#[derive(Clone, Debug)]
pub struct EntityHazardFilter(pub Vec<HazardEntity>);

impl HazardFilter for NoHazardFilter {
    fn is_irrelevant(&self, _entity: &HazardEntity) -> bool {
        false
    }
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

/// Deems hazards induced by `self` as irrelevant.
impl HazardFilter for HazardEntity {
    fn is_irrelevant(&self, haz: &HazardEntity) -> bool {
        self == haz
    }
}
