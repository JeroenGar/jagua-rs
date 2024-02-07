use itertools::Itertools;

use crate::collision_detection::hazard::Hazard;
use crate::collision_detection::hazard::HazardEntity;


/// A filter that determines which hazards are relevant for a specific purpose
pub trait HazardFilter {
    fn is_irrelevant(&self, entity: &HazardEntity) -> bool;
}


/// Returns the HazardEntities that are deemed irrelevant the given filter
pub fn get_irrelevant_hazard_entities<'a>(filter: &impl HazardFilter, hazards: impl Iterator<Item=&'a Hazard>) -> Vec<HazardEntity> {
    hazards.filter_map(|h| {
        match filter.is_irrelevant(&h.entity){
            true => Some(h.entity.clone()),
            false => None
        }
    }).collect_vec()
}