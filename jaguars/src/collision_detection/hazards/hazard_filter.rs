use itertools::Itertools;

use crate::collision_detection::hazards::hazard::Hazard;
use crate::collision_detection::hazards::hazard_entity::HazardEntity;

pub trait HazardFilter {
    fn is_relevant(&self, entity: &HazardEntity) -> bool;
}

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

pub fn ignored_entities<'a>(filter: &impl HazardFilter, hazards: impl Iterator<Item=&'a Hazard>) -> Option<Vec<&'a HazardEntity>> {
    let ignored_entities = hazards
        .filter(|h| !filter.is_relevant(h.entity()))
        .map(|h| h.entity()).collect_vec();

    match ignored_entities.is_empty() {
        true => None,
        false => Some(ignored_entities)
    }
}