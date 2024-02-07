use crate::collision_detection::hazard_filters::hazard_filter::HazardFilter;
use crate::collision_detection::hazard::HazardEntity;


/// A filter that deems hazards induced by quality zones above a certain quality as irrelevant
#[derive(Clone, Debug)]
pub struct QZHazardFilter {
    pub cutoff_quality: usize,
}

impl HazardFilter for QZHazardFilter {
    fn is_irrelevant(&self, entity: &HazardEntity) -> bool {
        match entity {
            HazardEntity::QualityZoneInferior { quality, .. } => *quality >= self.cutoff_quality,
            _ => false,
        }
    }
}