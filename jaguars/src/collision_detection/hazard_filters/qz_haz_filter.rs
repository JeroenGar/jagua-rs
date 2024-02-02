use crate::collision_detection::hazard_filters::hazard_filter::HazardFilter;
use crate::collision_detection::hazard::HazardEntity;

#[derive(Clone, Debug)]
pub struct QZHazardFilter {
    pub base_quality: usize,
}

impl HazardFilter for QZHazardFilter {
    fn is_relevant(&self, entity: &HazardEntity) -> bool {
        match entity {
            HazardEntity::QualityZoneInferior { quality, .. } => *quality < self.base_quality,
            _ => true,
        }
    }
}