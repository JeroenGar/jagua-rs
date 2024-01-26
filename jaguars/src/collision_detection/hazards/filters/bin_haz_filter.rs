use crate::collision_detection::hazards::filters::hazard_filter::HazardFilter;
use crate::collision_detection::hazards::hazard_entity::HazardEntity;

#[derive(Clone)]
pub struct BinHazardFilter;

impl HazardFilter for BinHazardFilter {
    fn is_relevant(&self, entity: &HazardEntity) -> bool {
        match entity {
            HazardEntity::Item(_) => true,
            HazardEntity::BinOuter => false,
            HazardEntity::BinHole { .. } => false,
            HazardEntity::QualityZoneInferior { .. } => false,
        }
    }
}
