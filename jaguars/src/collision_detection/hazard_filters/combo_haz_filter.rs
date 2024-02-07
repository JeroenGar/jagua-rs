use crate::collision_detection::hazard_filters::hazard_filter::HazardFilter;
use crate::collision_detection::hazard::HazardEntity;

/// A filter that combines multiple hazard filters into one
pub struct CombinedHazardFilter<'a> {
    pub filters: Vec<Box<&'a dyn HazardFilter>>,
}

impl<'a> HazardFilter for CombinedHazardFilter<'a> {
    fn is_irrelevant(&self, entity: &HazardEntity) -> bool {
        self.filters.iter()
            .any(|f| f.is_irrelevant(entity))
    }
}

impl<'a> CombinedHazardFilter<'a> {
    pub fn new() -> Self {
        Self {
            filters: Vec::new(),
        }
    }

    pub fn add(mut self, filter: &'a dyn HazardFilter) -> Self {
        self.filters.push(Box::new(filter));
        self
    }
}
