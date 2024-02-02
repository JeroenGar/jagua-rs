use itertools::Itertools;

use crate::collision_detection::hazard::Hazard;
use crate::collision_detection::hazard::HazardEntity;

pub trait HazardFilter {
    fn is_relevant(&self, entity: &HazardEntity) -> bool;
}

pub fn ignored_entities<'a>(filter: &impl HazardFilter, hazards: impl Iterator<Item=&'a Hazard>) -> Vec<HazardEntity> {
    hazards
        .filter(|h| !filter.is_relevant(&h.entity))
        .map(|h| h.entity.clone())
        .collect_vec()
}