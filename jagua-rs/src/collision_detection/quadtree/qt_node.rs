use tribool::Tribool;

use crate::collision_detection::hazard::HazardEntity;
use crate::collision_detection::quadtree::qt_hazard::QTHazPresence;
use crate::collision_detection::quadtree::qt_hazard::QTHazard;
use crate::collision_detection::quadtree::qt_hazard_vec::QTHazardVec;
use crate::collision_detection::quadtree::qt_partial_hazard::QTPartialHazard;
use crate::geometry::geo_traits::CollidesWith;
use crate::geometry::primitives::aa_rectangle::AARectangle;
use crate::geometry::primitives::point::Point;

/// A node in the quadtree
#[derive(Clone, Debug)]
pub struct QTNode {
    /// The level of the node in the tree, 0 being the bottom-most level
    pub level: u8,
    /// The bounding box of the node
    pub bbox: AARectangle,
    /// The children of the node, if any
    pub children: Option<Box<[QTNode; 4]>>,
    /// The hazards present in the node
    pub hazards: QTHazardVec,
}

impl QTNode {
    pub fn new(level: u8, bbox: AARectangle) -> Self {
        QTNode {
            level,
            bbox,
            children: None,
            hazards: QTHazardVec::new(),
        }
    }

    pub fn register_hazard(&mut self, hazard: QTHazard) {
        fn register_to_children(children: &mut Option<Box<[QTNode; 4]>>, hazard: &QTHazard) {
            if let Some(children) = children.as_mut() {
                let child_bboxes = [0, 1, 2, 3].map(|i| &children[i].bbox);
                let c_hazards = hazard.constrict(child_bboxes);

                for (i, c_hazard) in c_hazards.into_iter().enumerate() {
                    if let Some(c_hazard) = c_hazard {
                        children[i].register_hazard(c_hazard);
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

    pub fn deregister_hazard(&mut self, hazard_entity: &HazardEntity) {
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

    pub fn activate_hazard(&mut self, entity: &HazardEntity) {
        let modified = self.hazards.activate_hazard(entity);
        if modified {
            match &mut self.children {
                Some(children) => children.iter_mut().for_each(|c| c.activate_hazard(entity)),
                None => (),
            }
        }
    }

    pub fn deactivate_hazard(&mut self, entity: &HazardEntity) {
        let modified = self.hazards.deactivate_hazard(entity);
        if modified {
            match &mut self.children {
                Some(children) => children
                    .iter_mut()
                    .for_each(|c| c.deactivate_hazard(entity)),
                None => (),
            }
        }
    }

    fn generate_children(&mut self) {
        if self.level > 0 {
            let quadrants = self.bbox.quadrants();
            let children = quadrants.map(|q| QTNode::new(self.level - 1, q));
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

    /// Returns None if no collision between the entity and any hazard is detected,
    /// otherwise returns the first encountered hazard that collides with the entity
    /// In practice T is usually an `Edge` or `Circle`
    pub fn collides<T>(
        &self,
        entity: &T,
        irrelevant_hazards: &[HazardEntity],
    ) -> Option<&HazardEntity>
    where
        T: CollidesWith<AARectangle>,
        QTPartialHazard: CollidesWith<T>,
    {
        match self.hazards.strongest(irrelevant_hazards) {
            None => None,
            Some(strongest_hazard) => match entity.collides_with(&self.bbox) {
                false => None,
                true => match strongest_hazard.presence {
                    QTHazPresence::None => None,
                    QTHazPresence::Entire => Some(&strongest_hazard.entity),
                    QTHazPresence::Partial(_) => match &self.children {
                        Some(children) => {
                            //Check if any of the children intersect with the entity
                            children
                                .iter()
                                .map(|child| child.collides(entity, irrelevant_hazards))
                                .find(|x| x.is_some())
                                .flatten()
                        }
                        None => {
                            //Check if any of the partially present (and active) hazards collide with the entity
                            let mut relevant_hazards = self
                                .hazards
                                .active_hazards()
                                .iter()
                                .filter(|hz| !irrelevant_hazards.contains(&hz.entity));

                            relevant_hazards
                                .find(|hz| match &hz.presence {
                                    QTHazPresence::None => false,
                                    QTHazPresence::Entire => {
                                        unreachable!("should have been handled above")
                                    }
                                    QTHazPresence::Partial(p_haz) => {
                                        !irrelevant_hazards.contains(&hz.entity)
                                            && p_haz.collides_with(entity)
                                    }
                                })
                                .map(|hz| &hz.entity)
                        }
                    },
                },
            },
        }
    }

    pub fn definitely_collides<T>(&self, entity: &T, irrelevant_hazards: &[HazardEntity]) -> Tribool
    where
        T: CollidesWith<AARectangle>,
    {
        match self.hazards.strongest(irrelevant_hazards) {
            None => Tribool::False,
            Some(hazard) => match (entity.collides_with(&self.bbox), &hazard.presence) {
                (false, _) | (_, QTHazPresence::None) => Tribool::False,
                (true, QTHazPresence::Entire) => Tribool::True,
                (true, QTHazPresence::Partial(_)) => match &self.children {
                    Some(children) => {
                        //There is a partial hazard and the node has children, check all children
                        let mut result = Tribool::False; //Assume no collision
                        for i in 0..4 {
                            let child = &children[i];
                            match child.definitely_collides(entity, irrelevant_hazards) {
                                Tribool::True => return Tribool::True,
                                Tribool::Indeterminate => result = Tribool::Indeterminate,
                                Tribool::False => {}
                            }
                        }
                        result
                    }
                    None => Tribool::Indeterminate,
                },
            },
        }
    }

    pub fn point_definitely_collides_with(&self, point: &Point, entity: &HazardEntity) -> Tribool {
        match self.hazards.get(entity) {
            None => Tribool::False, //Node does not contain entity
            Some(hazard) => match self.bbox.collides_with(point) {
                false => Tribool::False, //Hazard present, but the point is fully outside the node
                true => match hazard.presence {
                    QTHazPresence::None => Tribool::False, //The hazard is of type None, a collision is impossible
                    QTHazPresence::Entire => Tribool::True, //The hazard is of type Entire, a collision is guaranteed
                    QTHazPresence::Partial(_) => match &self.children {
                        Some(children) => {
                            //There is a partial hazard and the node has children, check all children
                            let mut result = Tribool::False; //Assume no collision
                            for i in 0..4 {
                                let child = &children[i];
                                match child.point_definitely_collides_with(point, entity) {
                                    Tribool::True => return Tribool::True, //If a child for sure collides, we can immediately return Yes
                                    Tribool::Indeterminate => result = Tribool::Indeterminate, //If a child might collide, switch from to Maybe
                                    Tribool::False => {} //If child does not collide, do nothing
                                }
                            }
                            result
                        }
                        None => Tribool::Indeterminate, //There are no children, so we can't be sure
                    },
                },
            },
        }
    }
}
