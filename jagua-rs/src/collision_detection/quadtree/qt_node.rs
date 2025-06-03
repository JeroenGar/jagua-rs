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
        fn register_to_children(children: &mut Option<Box<[QTNode; 4]>>, hazard: &QTHazard) {
            if let Some(children) = children.as_mut() {
                let child_bboxes = [0, 1, 2, 3].map(|i| children[i].bbox);
                let c_hazards = hazard.constrict(child_bboxes);

                if let Some(c_hazards) = c_hazards {
                    for (child, c_hazard) in children.iter_mut().zip(c_hazards) {
                        if !matches!(c_hazard.presence, QTHazPresence::None) {
                            child.register_hazard(c_hazard);
                        }
                    }
                }
            }
        }

        //If the hazard is of the partial type, and we are not at the max tree depth: generate children
        if !self.has_children()
            && self.level > 0
            && matches!(hazard.presence, QTHazPresence::Partial(_))
        {
            self.generate_children();
            //register all existing hazards to the newly created children
            for hazard in self.hazards.all_hazards() {
                register_to_children(&mut self.children, hazard);
            }
        }

        register_to_children(&mut self.children, &hazard);
        self.hazards.add(hazard);
    }

    pub fn deregister_hazard(&mut self, hazard_entity: HazardEntity) {
        let removed_ch = self.hazards.remove(hazard_entity);

        if removed_ch.is_some() && self.has_children() {
            if self.hazards.is_empty() || self.hazards.has_only_entire_hazards() {
                //If there are no hazards, or only entire hazards, drop the children
                self.children = None;
            } else {
                //Otherwise, recursively deregister the entity from the children
                self.children
                    .as_mut()
                    .unwrap()
                    .iter_mut()
                    .for_each(|child| child.deregister_hazard(hazard_entity));
            }
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

    fn generate_children(&mut self) {
        if self.level > 0 {
            let quadrants = self.bbox.quadrants();
            let children = quadrants.map(|q| QTNode::new(self.level - 1, q, self.cd_threshold));
            self.children = Some(Box::new(children));
        }
    }

    pub fn get_number_of_children(&self) -> usize {
        match &self.children {
            Some(children) => {
                4 + children
                    .iter()
                    .map(|x| x.get_number_of_children())
                    .sum::<usize>()
            }
            None => 0,
        }
    }

    pub fn has_children(&self) -> bool {
        self.children.is_some()
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
