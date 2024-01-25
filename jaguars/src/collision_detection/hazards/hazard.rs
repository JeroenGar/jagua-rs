use std::sync::Arc;

use crate::collision_detection::hazards::hazard_entity::HazardEntity;
use crate::geometry::primitives::simple_polygon::SimplePolygon;

#[derive(Clone, Debug)]
pub struct Hazard {
    entity: HazardEntity,
    shape: Arc<SimplePolygon>,
    active: bool,
}

impl Hazard {
    pub fn new(entity: HazardEntity, shape: Arc<SimplePolygon>) -> Self {
        Self {
            entity,
            shape,
            active: true,
        }
    }
    pub fn activate(&mut self) {
        self.active = true;
    }

    pub fn deactivate(&mut self) {
        self.active = false;
    }

    pub fn is_active(&self) -> bool {
        self.active
    }

    pub fn entity(&self) -> &HazardEntity {
        &self.entity
    }

    pub fn shape(&self) -> &Arc<SimplePolygon> {
        &self.shape
    }
}