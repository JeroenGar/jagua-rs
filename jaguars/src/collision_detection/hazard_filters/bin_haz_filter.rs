use crate::collision_detection::hazard_filters::hazard_filter::HazardFilter;
use crate::collision_detection::hazard::HazardEntity;

/// Filter that deems all hazards induced by the Bin as irrelevant
#[derive(Clone)]
pub struct BinHazardFilter;

impl HazardFilter for BinHazardFilter {
    fn is_irrelevant(&self, entity: &HazardEntity) -> bool {
        match entity {
            HazardEntity::PlacedItem(_) => false,
            HazardEntity::BinExterior => true,
            HazardEntity::BinHole { .. } => true,
            HazardEntity::QualityZoneInferior { .. } => true,
        }
    }
}
