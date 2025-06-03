use crate::collision_detection::hazards::HazardEntity;
use crate::collision_detection::hazards::detector::HazardDetector;
use crate::collision_detection::hazards::filter::HazardFilter;
use crate::collision_detection::quadtree::QTHazPresence;
use crate::collision_detection::quadtree::QTHazard;
use crate::collision_detection::quadtree::qt_hazard_vec::QTHazardVec;
use crate::collision_detection::quadtree::qt_traits::QTQueryable;
use crate::geometry::geo_traits::CollidesWith;
use crate::geometry::primitives::Rect;

/// Quadtree node
#[derive(Clone, Debug)]
pub struct QTNode {
    /// The level of the node in the tree, 0 being the bottom-most level
    pub level: u8,
    /// The bounding box of the node
    pub bbox: Rect,
    /// The children of the node, if any
    pub children: Option<Box<[QTNode; 4]>>,
    /// The hazards present in the node
    pub hazards: QTHazardVec,
    /// Stop traversing the quadtree and perform collision detection immediately when the total number of edges in a node falls below this number
    pub cd_threshold: u8,
}

impl QTNode {
    pub fn new(level: u8, bbox: Rect, cd_threshold: u8) -> Self {
        QTNode {
            level,
            bbox,
            children: None,
            hazards: QTHazardVec::new(),
            cd_threshold,
        }
    }

    pub fn register_hazard(&mut self, hazard: QTHazard) {
        fn constrict_and_register_to_children(
            children: &mut Option<Box<[QTNode; 4]>>,
            hazard: &QTHazard,
        ) {
            if let Some(children) = children.as_mut() {
                // Constrict the hazard to the bounding boxes of the children
                let child_bboxes = children.each_ref().map(|c| c.bbox);
                let child_hazards = hazard.constrict(child_bboxes);

                // Register the hazards to the children if present
                child_hazards
                    .into_iter()
                    .enumerate()
                    .for_each(|(i, child_haz)| {
                        match child_haz.presence {
                            QTHazPresence::None => (), // No need to register if the hazard is not present
                            QTHazPresence::Partial(_) | QTHazPresence::Entire => {
                                children[i].register_hazard(child_haz);
                            }
                        }
                    });
            }
        }

        //Check if we have to expand the node (generate children)
        if let None = self.children
            && self.level > 0
            && let QTHazPresence::Partial(_) = hazard.presence
        {
            // Generate a child for every quadrant
            let children = self
                .bbox
                .quadrants()
                .map(|quad| QTNode::new(self.level - 1, quad, self.cd_threshold));
            self.children = Some(Box::new(children));

            // Register all previous hazards to them
            for hazard in self.hazards.all_hazards() {
                constrict_and_register_to_children(&mut self.children, hazard);
            }
        }

        constrict_and_register_to_children(&mut self.children, &hazard);
        self.hazards.add(hazard);
    }

    pub fn deregister_hazard(&mut self, hazard_entity: HazardEntity) {
        let modified = self.hazards.remove(hazard_entity).is_some();

        if modified && self.hazards.no_partial_hazards() {
            // Drop the children if there are no partially present hazards left
            self.children = None;
        }

        if modified && let Some(children) = &mut self.children {
            // Deregister the hazard from all children
            children
                .iter_mut()
                .for_each(|child| child.deregister_hazard(hazard_entity));
        }
    }

    pub fn activate_hazard(&mut self, entity: HazardEntity) {
        let modified = self.hazards.activate_hazard(entity);
        if modified {
            if let Some(children) = &mut self.children {
                children.iter_mut().for_each(|c| c.activate_hazard(entity))
            }
        }
    }

    pub fn deactivate_hazard(&mut self, entity: HazardEntity) {
        let modified = self.hazards.deactivate_hazard(entity);
        if modified {
            if let Some(children) = &mut self.children {
                children
                    .iter_mut()
                    .for_each(|c| c.deactivate_hazard(entity))
            }
        }
    }

    /// Used to detect collisions in a binary fashion: either there is a collision or there isn't.
    /// Returns `None` if no collision between the entity and any hazard is detected,
    /// otherwise the first encountered hazard that collides with the entity is returned.
    pub fn collides<T: QTQueryable>(
        &self,
        entity: &T,
        filter: &impl HazardFilter,
    ) -> Option<&HazardEntity> {
        match self.hazards.strongest(filter) {
            None => None,
            Some(strongest_hazard) => match entity.collides_with(&self.bbox) {
                false => None,
                true => match strongest_hazard.presence {
                    QTHazPresence::None => None,
                    QTHazPresence::Entire => Some(&strongest_hazard.entity),
                    QTHazPresence::Partial(_) => {
                        // Condition to perform collision detection now or pass it to children:
                        match &self.children {
                            Some(children) => {
                                //Check if any of the children collide with the entity
                                children
                                    .iter()
                                    .map(|child| child.collides(entity, filter))
                                    .find(|x| x.is_some())
                                    .flatten()
                            }
                            None => {
                                //Check if any of the partially present (and active) hazards collide with the entity
                                let mut relevant_hazards = self
                                    .hazards
                                    .active_hazards()
                                    .iter()
                                    .filter(|hz| !filter.is_irrelevant(&hz.entity));

                                relevant_hazards
                                    .find(|hz| match &hz.presence {
                                        QTHazPresence::None => false,
                                        QTHazPresence::Entire => {
                                            unreachable!("should have been handled above")
                                        }
                                        QTHazPresence::Partial(p_haz) => {
                                            p_haz.collides_with(entity)
                                        }
                                    })
                                    .map(|hz| &hz.entity)
                            }
                        }
                    }
                },
            },
        }
    }

    /// Gathers all hazards that collide with the entity and reports them to the `detector`.
    /// All hazards already present in the `detector` are ignored.
    pub fn collect_collisions<T: QTQueryable>(
        &self,
        entity: &T,
        detector: &mut impl HazardDetector,
    ) {
        if !entity.collides_with(&self.bbox) {
            // Entity does not collide with the node
            return;
        }

        // Condition to perform collision detection now or pass it to children:
        let perform_cd_now = self.hazards.n_active_edges() <= self.cd_threshold as usize;

        match (self.children.as_ref(), perform_cd_now) {
            (Some(children), false) => {
                //Do not perform any CD on this level, check the children
                children.iter().for_each(|child| {
                    child.collect_collisions(entity, detector);
                })
            }
            _ => {
                //Check the hazards now
                for hz in self.hazards.active_hazards().iter() {
                    match &hz.presence {
                        QTHazPresence::None => (),
                        QTHazPresence::Entire => {
                            if !detector.contains(&hz.entity) {
                                detector.push(hz.entity)
                            }
                        }
                        QTHazPresence::Partial(p_haz) => {
                            if !detector.contains(&hz.entity) && p_haz.collides_with(entity) {
                                detector.push(hz.entity);
                            }
                        }
                    }
                }
            }
        }
    }
}
