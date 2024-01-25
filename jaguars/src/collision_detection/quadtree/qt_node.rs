use itertools::Itertools;

use crate::collision_detection::collision::Collides;
use crate::collision_detection::hazards::hazard_entity::HazardEntity;
use crate::collision_detection::quadtree::constrict_cache::ConstrictCache;
use crate::collision_detection::quadtree::qt_hazard::QTHazard;
use crate::collision_detection::quadtree::qt_hazard_type::QTHazType;
use crate::collision_detection::quadtree::qt_hazard_vec::QTHazardVec;
use crate::geometry::primitives::aa_rectangle::AARectangle;
use crate::geometry::primitives::circle::Circle;
use crate::geometry::primitives::edge::Edge;
use crate::geometry::geo_traits::CollidesWith;
use crate::geometry::primitives::point::Point;

#[derive(Clone, Debug)]
pub struct QTNode {
    level: u8,
    bbox: AARectangle,
    children: Option<Box<[QTNode; 4]>>,
    hazards: QTHazardVec,
}

impl QTNode {
    pub fn new(level: u8, bbox: AARectangle, children: Option<Box<[QTNode; 4]>>) -> Self {
        let hazards = QTHazardVec::new();
        QTNode {
            level,
            bbox,
            children,
            hazards,
        }
    }

    pub fn register_hazard(&mut self, hazard: QTHazard) {
        fn register_to_children(children: &mut Option<Box<[QTNode; 4]>>, hazard: &QTHazard) {
            if let Some(children) = children.as_mut() {
                let mut c_cache = ConstrictCache::new();
                for i in 0..children.len() {
                    let child = &mut children[i];
                    let c_hazard = hazard.constrict(&child.bbox, i, &c_cache);
                    c_cache.store(i, &c_hazard);
                    if let Some(c_hazard) = c_hazard {
                        child.register_hazard(c_hazard);
                    }
                }
            }
        }

        self.invalidate_cache();

        //If the hazard is of the partial type, and we are not at the max tree depth: generate children
        if !self.has_children() && self.level > 0 && matches!(hazard.haz_type(), QTHazType::Partial(_)) {
            self.generate_children();
            //register all existing hazards to the newly created children
            for hazard in self.hazards.all_iter() {
                register_to_children(&mut self.children, hazard);
            }
        }

        register_to_children(&mut self.children, &hazard);
        self.hazards.add(hazard);
    }

    pub fn deregister_hazard(&mut self, hazard_entity: &HazardEntity) {
        self.invalidate_cache();

        let removed_ch = self.hazards.remove(hazard_entity);

        if removed_ch.is_some() && self.has_children() {
            if self.hazards.is_empty() || self.hazards.all_iter().all(|h| matches!(h.haz_type(), QTHazType::Entire)) {
                //If there are no more inclusion, or only inclusion of type Entire, drop the children
                self.drop_children();
            } else {
                //Otherwise, recursively deregister the entity from the children
                self.children.as_mut().unwrap().iter_mut()
                    .for_each(|child| child.deregister_hazard(hazard_entity));
            }
        }
    }

    pub fn activate_hazard(&mut self, entity: &HazardEntity) {
        let modified = self.hazards.activate_hazard(entity);
        if modified {
            self.invalidate_cache();
            match &mut self.children {
                Some(children) => children.iter_mut()
                    .for_each(|c| c.activate_hazard(entity)),
                None => ()
            }
        }
    }

    pub fn deactivate_hazard(&mut self, entity: &HazardEntity) {
        let modified = self.hazards.deactivate_hazard(entity);
        if modified {
            self.invalidate_cache();
            match &mut self.children {
                Some(children) => children.iter_mut()
                    .for_each(|c| c.deactivate_hazard(entity)),
                None => ()
            }
        }
    }

    fn generate_children(&mut self) {
        if self.level > 0 {
            self.invalidate_cache();

            self.children = Some(
                Box::new(
                    self.bbox.quadrants()
                        .map(|split_bbox|
                            QTNode::new(self.level - 1, split_bbox, None)
                        )
                )
            );
        }
    }

    pub fn get_number_of_children(&self) -> usize {
        match &self.children {
            Some(children) => 4 + children.iter().map(|x| x.get_number_of_children()).sum::<usize>(),
            None => 0
        }
    }

    pub fn drop_children(&mut self) {
        self.invalidate_cache();
        self.children = None
    }

    pub fn has_children(&self) -> bool {
        self.children.is_some()
    }

    pub fn level(&self) -> u8 {
        self.level
    }

    pub fn bbox(&self) -> &AARectangle {
        &self.bbox
    }

    pub fn children(&self) -> &Option<Box<[QTNode; 4]>> {
        &self.children
    }

    pub fn hazards(&self) -> &QTHazardVec {
        &self.hazards
    }

    fn invalidate_cache(&mut self) {
        //leave this here in case it is useful later
    }
    pub fn edge_collides(&self, edge: &Edge, ignored_entities: Option<&Vec<&HazardEntity>>) -> Option<&HazardEntity> {
        match self.hazards.strongest(ignored_entities) {
            None => None,
            Some(strongest_hazard) => match self.bbox.collides_with(edge) {
                false => None,
                true => match strongest_hazard.haz_type() {
                    QTHazType::Entire => {
                        Some(strongest_hazard.entity())
                    }
                    QTHazType::Partial(_) => match &self.children {
                        Some(children) => {
                            //Search if any of the children intersect with the edge
                            children.iter()
                                .map(|child| child.edge_collides(edge, ignored_entities))
                                .find(|x| x.is_some())
                                .flatten()
                        }
                        None => {
                            //If there are no children, we will need to resort to line intersection tests
                            for hz in self.hazards.active_iter() {
                                match hz.haz_type() {
                                    QTHazType::Entire => {} //non-ignored Entire inclusion are caught by the previous match
                                    QTHazType::Partial(partial_hazard) => {
                                        if ignored_entities.map_or(true, |ie| !ie.contains(&hz.entity())) {
                                            //do intersection test if this entity is not ignored
                                            if partial_hazard.collides_with(edge) {
                                                return Some(hz.entity());
                                            }
                                        }
                                    }
                                }
                            }
                            None
                        }
                    }
                }
            }
        }
    }

    pub fn circle_collides(&self, circle: &Circle, ignored_entities: Option<&Vec<&HazardEntity>>) -> Option<&HazardEntity> {
        match self.hazards.strongest(ignored_entities) {
            None => None,
            Some(strongest_hazard) => match circle.collides_with(self.bbox()) {
                false => None,
                true => match strongest_hazard.haz_type() {
                    QTHazType::Entire => Some(strongest_hazard.entity()),
                    QTHazType::Partial(_) => match self.children() {
                        Some(children) => {
                            //Search if any of the children intersect with the circle
                            children.iter()
                                .map(|child| child.circle_collides(circle, ignored_entities))
                                .find(|x| x.is_some())
                                .flatten()
                        }
                        None => {
                            for hz in self.hazards.active_iter() {
                                match hz.haz_type() {
                                    QTHazType::Entire => {} //non-ignored Entire inclusion are caught by the previous match
                                    QTHazType::Partial(partial_hazard) => {
                                        if ignored_entities.map_or(true, |ie| !ie.contains(&hz.entity())) {
                                            //do intersection test if this shape is not ignored
                                            if partial_hazard.collides_with(circle) {
                                                return Some(hz.entity());
                                            }
                                        }
                                    }
                                }
                            }
                            None
                        }
                    }
                }
            }
        }
    }

    pub fn circle_definitely_collides(&self, circle: &Circle, ignored_entities: Option<&Vec<&HazardEntity>>) -> Collides {
        match self.hazards.strongest(ignored_entities) {
            None => Collides::No,
            Some(hazard) => match (circle.collides_with(self.bbox()), hazard.haz_type()) {
                (false, _) => Collides::No,
                (true, QTHazType::Entire) => Collides::Yes,
                (true, QTHazType::Partial(_)) => match self.children() {
                    Some(children) => {
                        //There is a partial hazard and the node has children, check all children
                        let mut collides = Collides::No; //Assume no collision
                        for i in 0..4 {
                            let child = &children[i];
                            match child.circle_definitely_collides(circle, ignored_entities) {
                                Collides::Yes => return Collides::Yes, //If a child for sure collides, we can immediately return Yes
                                Collides::Unsure => collides = Collides::Unsure, //If a child might collide, switch from to Maybe
                                Collides::No => {} //If child does not collide, do nothing
                            }
                        }
                        collides
                    }
                    None => Collides::Unsure,
                },
            },
        }
    }

    pub fn edge_definitely_collides(&self, edge: &Edge, ignored_entities: Option<&Vec<&HazardEntity>>) -> Collides {
        match self.hazards.strongest(ignored_entities) {
            None => Collides::No, //Node does not contain inclusion
            Some(strongest_hazard) => match self.bbox.collides_with(edge) {
                false => Collides::No, //Hazard present, but the edge is fully outside of the node
                true => match strongest_hazard.haz_type() {
                    QTHazType::Entire => Collides::Yes, //The hazard is of type Entire, a collision is guaranteed
                    QTHazType::Partial(_) => match &self.children {
                        Some(children) => {
                            //There is a partial hazard and the node has children, check all children
                            let mut collides = Collides::No; //Assume no collision
                            for i in 0..4 {
                                let child = &children[i];
                                match child.edge_definitely_collides(edge, ignored_entities) {
                                    Collides::Yes => return Collides::Yes, //If a child for sure collides, we can immediately return Yes
                                    Collides::Unsure => collides = Collides::Unsure, //If a child might collide, switch to Maybe
                                    Collides::No => {} //If child does not collide, do nothing
                                }
                            }
                            collides
                        }
                        None => Collides::Unsure, //There are no children, so we can't be sure
                    }
                }
            }
        }
    }

    pub fn point_definitely_collides_with(&self, point: &Point, entity: &HazardEntity) -> Collides {
        match self.hazards.get(entity) {
            None => Collides::No, //Node does not contain inclusion
            Some(hazard) => match self.bbox.collides_with(point) {
                false => Collides::No, //Hazard present, but the point is fully outside of the node
                true => match hazard.haz_type() {
                    QTHazType::Entire => Collides::Yes, //The hazard is of type Entire, a collision is guaranteed
                    QTHazType::Partial(_) => match &self.children {
                        Some(children) => {
                            //There is a partial hazard and the node has children, check all children
                            let mut collides = Collides::No; //Assume no collision
                            for i in 0..4 {
                                let child = &children[i];
                                match child.point_definitely_collides_with(point, entity) {
                                    Collides::Yes => return Collides::Yes, //If a child for sure collides, we can immediately return Yes
                                    Collides::Unsure => collides = Collides::Unsure, //If a child might collide, switch from No to Maybe
                                    Collides::No => () //If child does not collide, do nothing
                                }
                            }
                            collides
                        }
                        None => Collides::Unsure, //There are no children, so we can't be sure
                    }
                }
            }
        }
    }
}
