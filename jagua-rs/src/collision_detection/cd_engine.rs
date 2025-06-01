use crate::collision_detection::hazards::Hazard;
use crate::collision_detection::hazards::HazardEntity;
use crate::collision_detection::hazards::detector::HazardDetector;
use crate::collision_detection::hazards::filter::HazardFilter;
use crate::collision_detection::quadtree::{QTHazPresence, QTHazard, QTNode};
use crate::geometry::Transformation;
use crate::geometry::fail_fast::{SPSurrogate, SPSurrogateConfig};
use crate::geometry::geo_enums::{GeoPosition, GeoRelation};
use crate::geometry::geo_traits::{CollidesWith, Transformable};
use crate::geometry::primitives::Rect;
use crate::geometry::primitives::SPolygon;
use crate::util::assertions;
use itertools::Itertools;
use serde::{Deserialize, Serialize};

/// The Collision Detection Engine (CDE).
/// [`Hazard`]s can be (de)registered and collision queries can be performed.
#[derive(Clone, Debug)]
pub struct CDEngine {
    pub quadtree: QTNode,
    pub static_hazards: Vec<Hazard>,
    pub dynamic_hazards: Vec<Hazard>,
    pub config: CDEConfig,
    pub bbox: Rect,
    pub uncommitted_deregisters: Vec<Hazard>,
}

impl CDEngine {
    pub fn new(bbox: Rect, static_hazards: Vec<Hazard>, config: CDEConfig) -> CDEngine {
        let mut qt_root = QTNode::new(config.quadtree_depth, bbox);

        for haz in static_hazards.iter() {
            let qt_haz = QTHazard::from_qt_root(qt_root.bbox, haz);
            qt_root.register_hazard(qt_haz);
        }

        CDEngine {
            quadtree: qt_root,
            static_hazards,
            dynamic_hazards: vec![],
            config,
            bbox,
            uncommitted_deregisters: vec![],
        }
    }

    /// Registers a new hazard in the CDE.
    pub fn register_hazard(&mut self, hazard: Hazard) {
        debug_assert!(
            !self
                .dynamic_hazards
                .iter()
                .any(|h| h.entity == hazard.entity),
            "Hazard already registered"
        );
        let hazard_in_uncommitted_deregs = self
            .uncommitted_deregisters
            .iter()
            .position(|h| h.entity == hazard.entity);

        let hazard = match hazard_in_uncommitted_deregs {
            Some(index) => {
                let unc_hazard = self.uncommitted_deregisters.swap_remove(index);
                self.quadtree.activate_hazard(unc_hazard.entity);
                unc_hazard
            }
            None => {
                let qt_haz = QTHazard::from_qt_root(self.bbox, &hazard);
                self.quadtree.register_hazard(qt_haz);
                hazard
            }
        };
        self.dynamic_hazards.push(hazard);

        debug_assert!(assertions::qt_contains_no_dangling_hazards(self));
    }

    /// Removes a hazard from the CDE.
    /// If `commit_instant` the deregistration is fully executed immediately.
    /// If not, the deregistration causes the hazard to be deactivated in the quadtree and
    /// the hazard_proximity_grid to become dirty (and therefore inaccessible).
    /// <br>
    /// Can be beneficial not to `commit_instant` if multiple hazards are to be deregistered, or if the chance of
    /// restoring from a snapshot with the hazard present is high.
    pub fn deregister_hazard(&mut self, hazard_entity: HazardEntity, commit_instant: bool) {
        let haz_index = self
            .dynamic_hazards
            .iter()
            .position(|h| h.entity == hazard_entity)
            .expect("Hazard not found");

        let hazard = self.dynamic_hazards.swap_remove(haz_index);

        match commit_instant {
            true => self.quadtree.deregister_hazard(hazard_entity),
            false => {
                self.quadtree.deactivate_hazard(hazard_entity);
                self.uncommitted_deregisters.push(hazard);
            }
        }
        debug_assert!(assertions::qt_contains_no_dangling_hazards(self));
    }

    pub fn create_snapshot(&mut self) -> CDESnapshot {
        self.commit_deregisters();
        CDESnapshot {
            dynamic_hazards: self.dynamic_hazards.clone(),
        }
    }

    /// Restores the CDE to a previous state, as described by the snapshot.
    pub fn restore(&mut self, snapshot: &CDESnapshot) {
        //Quadtree
        let mut hazards_to_remove = self.dynamic_hazards.iter().map(|h| h.entity).collect_vec();
        debug_assert!(hazards_to_remove.len() == self.dynamic_hazards.len());
        let mut hazards_to_add = vec![];

        for hazard in snapshot.dynamic_hazards.iter() {
            let hazard_already_present = hazards_to_remove.iter().position(|h| h == &hazard.entity);
            if let Some(idx) = hazard_already_present {
                //the hazard is already present in the CDE, remove it from the hazards to remove
                hazards_to_remove.swap_remove(idx);
            } else {
                //the hazard is not present in the CDE, add it to the list of hazards to add
                hazards_to_add.push(hazard.clone());
            }
        }

        //Hazards currently registered in the CDE, but not in the snapshot
        for haz_entity in hazards_to_remove.iter() {
            let haz_index = self
                .dynamic_hazards
                .iter()
                .position(|h| &h.entity == haz_entity)
                .expect("Hazard not found");
            self.dynamic_hazards.swap_remove(haz_index);
            self.quadtree.deregister_hazard(*haz_entity);
        }

        //Some of the uncommitted deregisters might be in present in snapshot, if so we can just reactivate them
        for unc_haz in self.uncommitted_deregisters.drain(..) {
            if let Some(pos) = hazards_to_add
                .iter()
                .position(|h| h.entity == unc_haz.entity)
            {
                //the uncommitted removed hazard needs to be activated again
                self.quadtree.activate_hazard(unc_haz.entity);
                self.dynamic_hazards.push(unc_haz);
                hazards_to_add.swap_remove(pos);
            } else {
                //uncommitted deregister is not preset in the snapshot, delete it from the quadtree
                self.quadtree.deregister_hazard(unc_haz.entity);
            }
        }

        for hazard in hazards_to_add {
            let qt_haz = QTHazard::from_qt_root(self.bbox, &hazard);
            self.quadtree.register_hazard(qt_haz);
            self.dynamic_hazards.push(hazard);
        }

        debug_assert!(self.dynamic_hazards.len() == snapshot.dynamic_hazards.len());
    }

    /// Commits all pending deregisters by actually removing them from the quadtree
    pub fn commit_deregisters(&mut self) {
        for uncommitted_hazard in self.uncommitted_deregisters.drain(..) {
            self.quadtree.deregister_hazard(uncommitted_hazard.entity);
        }
    }

    pub fn quadtree(&self) -> &QTNode {
        &self.quadtree
    }

    pub fn number_of_nodes(&self) -> usize {
        1 + self.quadtree.get_number_of_children()
    }

    pub fn bbox(&self) -> Rect {
        self.bbox
    }

    pub fn config(&self) -> CDEConfig {
        self.config
    }

    pub fn has_uncommitted_deregisters(&self) -> bool {
        !self.uncommitted_deregisters.is_empty()
    }

    /// Returns all hazards in the CDE, which can change during the lifetime of the CDE.
    pub fn dynamic_hazards(&self) -> &Vec<Hazard> {
        &self.dynamic_hazards
    }

    /// Returns all hazards in the CDE, which cannot change during the lifetime of the CDE.
    pub fn static_hazards(&self) -> &Vec<Hazard> {
        &self.static_hazards
    }

    /// Returns all hazards in the CDE, both static and dynamic.
    pub fn all_hazards(&self) -> impl Iterator<Item = &Hazard> {
        self.static_hazards
            .iter()
            .chain(self.dynamic_hazards.iter())
    }

    /// Checks whether a simple polygon collides with any of the (relevant) hazards
    /// # Arguments
    /// * `shape` - The shape (already transformed) to be checked for collisions
    /// * `filter` - Hazard filter to be applied
    pub fn detect_poly_collision(&self, shape: &SPolygon, filter: &impl HazardFilter) -> bool {
        if self.bbox.relation_to(shape.bbox) != GeoRelation::Surrounding {
            //The CDE does not capture the entire shape, so we can immediately return true
            true
        } else {
            //Instead of each time starting from the quadtree root, we can use the virtual root (lowest level node which fully surrounds the shape)
            let v_qt_root = self.get_virtual_root(shape);

            // Check for edge intersections with the shape
            for edge in shape.edge_iter() {
                if v_qt_root.collides(&edge, filter).is_some() {
                    return true;
                }
            }

            // Check for containment of the shape in any of the hazards
            for qt_hazard in v_qt_root.hazards.active_hazards() {
                match &qt_hazard.presence {
                    QTHazPresence::None => {}
                    QTHazPresence::Entire => unreachable!(
                        "Entire hazards in the virtual root should have been caught by the edge intersection tests"
                    ),
                    QTHazPresence::Partial(qthaz_partial) => {
                        if !filter.is_irrelevant(&qt_hazard.entity)
                            && self.poly_or_hazard_are_contained(
                                shape,
                                &qthaz_partial.shape,
                                qt_hazard.entity,
                            )
                        {
                            // The hazard is contained in the shape (or vice versa)
                            return true;
                        }
                    }
                }
            }

            false
        }
    }

    /// Checks whether a surrogate collides with any of the (relevant) hazards.
    /// # Arguments
    /// * `base_surrogate` - The (untransformed) surrogate to be checked for collisions
    /// * `transform` - The transformation to be applied to the surrogate
    /// * `filter` - Hazard filter to be applied
    pub fn detect_surr_collision(
        &self,
        base_surrogate: &SPSurrogate,
        transform: &Transformation,
        filter: &impl HazardFilter,
    ) -> bool {
        for pole in base_surrogate.ff_poles() {
            let t_pole = pole.transform_clone(transform);
            if self.quadtree.collides(&t_pole, filter).is_some() {
                return true;
            }
        }
        for pier in base_surrogate.ff_piers() {
            let t_pier = pier.transform_clone(transform);
            if self.quadtree.collides(&t_pier, filter).is_some() {
                return true;
            }
        }
        false
    }

    /// Check for collision by containment between a polygon and a hazard.
    pub fn poly_or_hazard_are_contained(
        &self,
        shape: &SPolygon,
        haz_shape: &SPolygon,
        haz_entity: HazardEntity,
    ) -> bool {
        //Due to possible fp issues, we check if the bboxes are "almost" related
        //"almost" meaning that, when edges are very close together, they are considered equal.
        //Some relations which would normally be seen as Intersecting are now being considered Enclosed/Surrounding
        let bbox_relation = haz_shape.bbox.almost_relation_to(shape.bbox);

        let (s_mu, s_omega) = match bbox_relation {
            GeoRelation::Surrounding => (shape, haz_shape), //inclusion possible
            GeoRelation::Enclosed => (haz_shape, shape),    //inclusion possible
            GeoRelation::Disjoint | GeoRelation::Intersecting => {
                //no inclusion is possible
                return match haz_entity.position() {
                    GeoPosition::Interior => false,
                    GeoPosition::Exterior => true,
                };
            }
        };

        if std::ptr::eq(haz_shape, s_omega) {
            //s_omega is registered in the quadtree.
            //maybe the quadtree can help us.
            if let Ok(collides) = self
                .quadtree
                .definitely_collides_with(&s_mu.poi.center, haz_entity)
                .try_into()
            {
                return collides;
            }
        }
        let inclusion = s_omega.collides_with(&s_mu.poi.center);

        match haz_entity.position() {
            GeoPosition::Interior => inclusion,
            GeoPosition::Exterior => !inclusion,
        }
    }

    /// Collects all hazards with which the polygon collides and reports them to the detector.
    pub fn collect_poly_collisions(&self, shape: &SPolygon, detector: &mut impl HazardDetector) {
        if self.bbox.relation_to(shape.bbox) != GeoRelation::Surrounding {
            detector.push(HazardEntity::Exterior)
        }

        //Instead of each time starting from the quadtree root, we can use the virtual root (lowest level node which fully surrounds the shape)
        let v_quadtree = self.get_virtual_root(shape);

        //collect all colliding entities due to edge intersection
        shape
            .edge_iter()
            .for_each(|e| v_quadtree.collect_collisions(&e, detector));

        v_quadtree
            .hazards
            .active_hazards()
            .iter()
            .for_each(|qt_haz| match &qt_haz.presence {
                QTHazPresence::Entire | QTHazPresence::None => {}
                QTHazPresence::Partial(qt_par_haz) => {
                    if !detector.contains(&qt_haz.entity)
                        && self.poly_or_hazard_are_contained(
                            shape,
                            &qt_par_haz.shape,
                            qt_haz.entity,
                        )
                    {
                        detector.push(qt_haz.entity);
                    }
                }
            })
    }

    /// Collects all hazards with which the surrogate collides and reports them to the detector.
    pub fn collect_surr_collisions(
        &self,
        base_surrogate: &SPSurrogate,
        transform: &Transformation,
        detector: &mut impl HazardDetector,
    ) {
        for pole in base_surrogate.ff_poles() {
            let t_pole = pole.transform_clone(transform);
            self.quadtree.collect_collisions(&t_pole, detector)
        }
        for pier in base_surrogate.ff_piers() {
            let t_pier = pier.transform_clone(transform);
            self.quadtree.collect_collisions(&t_pier, detector);
        }
    }

    /// Returns the lowest `QTNode` that completely surrounds the bounding box of the shape.
    /// Can be used to start collision detection from a lower level in the quadtree.
    pub fn get_virtual_root(&self, shape: &SPolygon) -> &QTNode {
        let mut v_root = &self.quadtree;
        while let Some(children) = v_root.children.as_ref() {
            // Keep going down the tree until we cannot find a child that fully surrounds the shape
            let surrounding_child = children
                .iter()
                .find(|child| child.bbox.relation_to(shape.bbox) == GeoRelation::Surrounding);
            match surrounding_child {
                Some(child) => v_root = child,
                None => break,
            }
        }
        v_root
    }
}

///Configuration of the [`CDEngine`]
#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub struct CDEConfig {
    ///Maximum depth of the quadtree
    pub quadtree_depth: u8,
    ///Configuration of the surrogate generation for items
    pub item_surrogate_config: SPSurrogateConfig,
}

/// Snapshot of the state of [`CDEngine`]. Can be used to restore to a previous state.
#[derive(Clone, Debug)]
pub struct CDESnapshot {
    dynamic_hazards: Vec<Hazard>,
}
