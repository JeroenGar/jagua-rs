use crate::collision_detection::hazard::Hazard;
use crate::collision_detection::hazard::HazardEntity;
use crate::collision_detection::hazard_helpers::{DetectionMap, HazardDetector};
use crate::collision_detection::hpg::grid::Grid;
use crate::collision_detection::hpg::hazard_proximity_grid::{DirtyState, HazardProximityGrid};
use crate::collision_detection::hpg::hpg_cell::HPGCell;
use crate::collision_detection::quadtree::qt_node::QTNode;
use crate::collision_detection::quadtree::qt_traits::QTQueryable;
use crate::fsize;
use crate::geometry::fail_fast::sp_surrogate::SPSurrogate;
use crate::geometry::geo_enums::{GeoPosition, GeoRelation};
use crate::geometry::geo_traits::{CollidesWith, Shape, Transformable, TransformableFrom};
use crate::geometry::primitives::aa_rectangle::AARectangle;
use crate::geometry::primitives::circle::Circle;
use crate::geometry::primitives::edge::Edge;
use crate::geometry::primitives::point::Point;
use crate::geometry::primitives::simple_polygon::SimplePolygon;
use crate::geometry::transformation::Transformation;
use crate::util::assertions;
use crate::util::config::CDEConfig;
use indexmap::IndexSet;
use tribool::Tribool;

/// The Collision Detection Engine (CDE).
/// The CDE can resolve a range of collision queries
/// and update its state by registering and deregistering hazards.
#[derive(Clone, Debug)]
pub struct CDEngine {
    quadtree: QTNode,
    static_hazards: Vec<Hazard>,
    dynamic_hazards: Vec<Hazard>,
    haz_prox_grid: Option<HazardProximityGrid>,
    config: CDEConfig,
    bbox: AARectangle,
    uncommitted_deregisters: Vec<Hazard>,
}

/// Snapshot of the state of [CDEngine] at a given time.
/// The [CDEngine] can take snapshots of itself at any time, and use them to restore to that state later.
#[derive(Clone, Debug)]
pub struct CDESnapshot {
    dynamic_hazards: Vec<Hazard>,
    grid: Option<Grid<HPGCell>>,
}

impl CDEngine {
    pub fn new(bbox: AARectangle, static_hazards: Vec<Hazard>, config: CDEConfig) -> CDEngine {
        let haz_prox_grid = match config.hpg_n_cells {
            0 => None,
            hpg_n_cells => Some(HazardProximityGrid::new(
                bbox.clone(),
                &static_hazards,
                hpg_n_cells,
            )),
        };

        let mut qt_root = QTNode::new(config.quadtree_depth, bbox.clone());

        for haz in static_hazards.iter() {
            qt_root.register_hazard(haz.into());
        }

        CDEngine {
            quadtree: qt_root,
            static_hazards,
            dynamic_hazards: vec![],
            haz_prox_grid,
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
                self.quadtree.register_hazard((&hazard).into());
                hazard
            }
        };
        if let Some(hpg) = self.haz_prox_grid.as_mut() {
            hpg.register_hazard(&hazard)
        }
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
    /// <br>
    /// Call [`Self::commit_deregisters`] to commit all uncommitted deregisters in both quadtree & hazard proximity grid
    /// or [`Self::flush_haz_prox_grid`] to just clear the hazard proximity grid.
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
        if let Some(hpg) = self.haz_prox_grid.as_mut() {
            hpg.deregister_hazard(hazard_entity, self.dynamic_hazards.iter(), commit_instant)
        }
        debug_assert!(assertions::qt_contains_no_dangling_hazards(self));
    }

    pub fn create_snapshot(&mut self) -> CDESnapshot {
        self.commit_deregisters();
        assert!(
            self.haz_prox_grid
                .as_ref()
                .map_or(true, |hpg| !hpg.is_dirty())
        );
        CDESnapshot {
            dynamic_hazards: self.dynamic_hazards.clone(),
            grid: self.haz_prox_grid.as_ref().map(|hpg| hpg.grid.clone()),
        }
    }

    /// Restores the CDE to a previous state, as described by the snapshot.
    pub fn restore(&mut self, snapshot: &CDESnapshot) {
        //Quadtree
        let mut hazards_to_remove = self
            .dynamic_hazards
            .iter()
            .map(|h| h.entity)
            .collect::<IndexSet<HazardEntity>>();
        debug_assert!(hazards_to_remove.len() == self.dynamic_hazards.len());
        let mut hazards_to_add = vec![];

        for hazard in snapshot.dynamic_hazards.iter() {
            let hazard_already_present = hazards_to_remove.swap_remove(&hazard.entity);
            if !hazard_already_present {
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
            self.quadtree.register_hazard((&hazard).into());
            self.dynamic_hazards.push(hazard);
        }

        //Hazard proximity grid
        if let Some(hpg) = self.haz_prox_grid.as_mut() {
            hpg.restore(snapshot.grid.clone().expect("no hpg in snapshot"));
        }

        debug_assert!(self.dynamic_hazards.len() == snapshot.dynamic_hazards.len());
    }

    /// Commits all pending deregisters by actually removing them from the quadtree
    /// and flushing the hazard proximity grid.
    pub fn commit_deregisters(&mut self) {
        for uc_haz in self.uncommitted_deregisters.drain(..) {
            self.quadtree.deregister_hazard(uc_haz.entity);
        }
        if let Some(hpg) = self.haz_prox_grid.as_mut() {
            hpg.flush_deregisters(self.dynamic_hazards.iter())
        }
    }

    pub fn quadtree(&self) -> &QTNode {
        &self.quadtree
    }

    pub fn number_of_nodes(&self) -> usize {
        1 + self.quadtree.get_number_of_children()
    }

    pub fn bbox(&self) -> &AARectangle {
        &self.bbox
    }

    pub fn smallest_qt_node_dimension(&self) -> fsize {
        let bbox = &self.quadtree.bbox;
        let level = self.quadtree.level;
        //every level, the dimension is halved
        bbox.width() / (2.0 as fsize).powi(level as i32)
    }

    pub fn config(&self) -> CDEConfig {
        self.config
    }

    /// If the grid has uncommitted deregisters, it is considered dirty and cannot be accessed.
    /// To flush all the changes, call [`Self::flush_haz_prox_grid`].
    pub fn haz_prox_grid(&self) -> Result<&HazardProximityGrid, DirtyState> {
        let grid = self.haz_prox_grid.as_ref().expect("no hpg present");
        match grid.is_dirty() {
            true => Err(DirtyState),
            false => Ok(grid),
        }
    }

    /// Flushes all uncommitted deregisters in the [`HazardProximityGrid`].
    pub fn flush_haz_prox_grid(&mut self) {
        if let Some(hpg) = self.haz_prox_grid.as_mut() {
            hpg.flush_deregisters(self.dynamic_hazards.iter())
        }
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

    ///Checks whether a reference simple polygon, with a transformation applies, collides with any of the hazards.
    ///The check is first done on the surrogate, then with the actual shape.
    ///A buffer shape is used as a temporary storage for the transformed shape.
    /// # Arguments
    /// * `reference_shape` - The shape to be checked for collisions
    /// * `transform` - The transformation to be applied to the reference shape
    /// * `buffer_shape` - A temporary storage for the transformed shape
    /// * `irrelevant_hazards` - entities to be ignored during the check
    pub fn surrogate_or_poly_collides(
        &self,
        reference_shape: &SimplePolygon,
        transform: &Transformation,
        buffer_shape: &mut SimplePolygon,
        irrelevant_hazards: &[HazardEntity],
    ) -> bool {
        //Begin with checking the surrogate for collisions
        match self.surrogate_collides(reference_shape.surrogate(), transform, irrelevant_hazards) {
            true => true,
            false => {
                //Transform the reference_shape and store the result in the buffer_shape
                buffer_shape.transform_from(reference_shape, transform);
                self.poly_collides(buffer_shape, irrelevant_hazards)
            }
        }
    }

    ///Checks whether a simple polygon collides with any of the (relevant) hazards
    /// # Arguments
    /// * `shape` - The shape (already transformed) to be checked for collisions
    /// * `irrelevant_hazards` - entities to be ignored during the check
    pub fn poly_collides(
        &self,
        shape: &SimplePolygon,
        irrelevant_hazards: &[HazardEntity],
    ) -> bool {
        match self.bbox.relation_to(&shape.bbox()) {
            //Not fully inside bbox => definite collision
            GeoRelation::Disjoint | GeoRelation::Enclosed | GeoRelation::Intersecting => true,
            GeoRelation::Surrounding => {
                self.poly_collides_by_edge_intersection(shape, irrelevant_hazards)
                    || self.poly_collides_by_containment(shape, irrelevant_hazards)
            }
        }
    }

    /// Checks whether a surrogate collides with any of the (relevant) hazards.
    /// # Arguments
    /// * `base_surrogate` - The (untransformed) surrogate to be checked for collisions
    /// * `transform` - The transformation to be applied to the surrogate
    /// * `irrelevant_hazards` - entities to be ignored during the check
    pub fn surrogate_collides(
        &self,
        base_surrogate: &SPSurrogate,
        transform: &Transformation,
        irrelevant_hazards: &[HazardEntity],
    ) -> bool {
        for pole in base_surrogate.ff_poles() {
            let t_pole = pole.transform_clone(transform);
            if self
                .quadtree
                .collides(&t_pole, irrelevant_hazards)
                .is_some()
            {
                return true;
            }
        }
        for pier in base_surrogate.ff_piers() {
            let t_pier = pier.transform_clone(transform);
            if self
                .quadtree
                .collides(&t_pier, irrelevant_hazards)
                .is_some()
            {
                return true;
            }
        }
        false
    }

    /// Checks whether a point definitely collides with any of the (relevant) hazards.
    /// Only fully hazardous nodes in the quadtree are considered.
    pub fn point_definitely_collides_with(&self, point: &Point, entity: HazardEntity) -> Tribool {
        match self.bbox.collides_with(point) {
            false => Tribool::Indeterminate, //point is outside the quadtree, so no information available
            true => self.quadtree.definitely_collides_with(point, entity),
        }
    }

    /// Checks whether an edge definitely collides with any of the (relevant) hazards.
    /// Only fully hazardous nodes in the quadtree are considered.
    pub fn edge_definitely_collides(
        &self,
        edge: &Edge,
        irrelevant_hazards: &[HazardEntity],
    ) -> Tribool {
        match !self.bbox.collides_with(&edge.start) || !self.bbox.collides_with(&edge.end) {
            true => Tribool::True, //if either the start or end of the edge is outside the quadtree, it definitely collides
            false => self.quadtree.definitely_collides(edge, irrelevant_hazards),
        }
    }

    /// Checks whether a circle definitely collides with any of the (relevant) hazards.
    /// Only fully hazardous nodes in the quadtree are considered.
    pub fn circle_definitely_collides(
        &self,
        circle: &Circle,
        irrelevant_hazards: &[HazardEntity],
    ) -> Tribool {
        match self.bbox.collides_with(&circle.center) {
            false => Tribool::True, //outside the quadtree, so definitely collides
            true => self
                .quadtree
                .definitely_collides(circle, irrelevant_hazards),
        }
    }

    fn poly_collides_by_edge_intersection(
        &self,
        shape: &SimplePolygon,
        irrelevant_hazards: &[HazardEntity],
    ) -> bool {
        shape
            .edge_iter()
            .any(|e| self.quadtree.collides(&e, irrelevant_hazards).is_some())
    }

    fn poly_collides_by_containment(
        &self,
        shape: &SimplePolygon,
        irrelevant_hazards: &[HazardEntity],
    ) -> bool {
        //collect all active and non-ignored hazards
        self.all_hazards()
            .filter(|h| h.active && !irrelevant_hazards.contains(&h.entity))
            .any(|haz| self.poly_or_hazard_are_contained(shape, haz))
    }

    fn poly_or_hazard_are_contained(&self, shape: &SimplePolygon, haz: &Hazard) -> bool {
        //Due to possible fp issues, we check if the bboxes are "almost" related
        //"almost" meaning that, when edges are very close together, they are considered equal.
        //Some relations which would normally be seen as Intersecting are now being considered Enclosed/Surrounding
        let haz_shape = haz.shape.as_ref();
        let bbox_relation = haz_shape.bbox().almost_relation_to(&shape.bbox());

        let (s_mu, s_omega) = match bbox_relation {
            GeoRelation::Surrounding => (shape, haz_shape), //inclusion possible
            GeoRelation::Enclosed => (haz_shape, shape),    //inclusion possible
            GeoRelation::Disjoint | GeoRelation::Intersecting => {
                //no inclusion is possible
                return match haz.entity.position() {
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
                .definitely_collides_with(&s_mu.poi.center, haz.entity)
                .try_into()
            {
                return collides;
            }
        }
        let inclusion = s_omega.collides_with(&s_mu.poi.center);

        match haz.entity.position() {
            GeoPosition::Interior => inclusion,
            GeoPosition::Exterior => !inclusion,
        }
    }

    /// Stores all the (relevant) hazards present inside any [QTQueryable] entity in the detector
    pub fn hazards_within<T>(
        &self,
        entity: &T,
        irrelevant_hazards: &[HazardEntity],
        detected: &mut impl HazardDetector,
    ) where
        T: QTQueryable,
    {
        irrelevant_hazards
            .iter()
            .for_each(|i_haz| detected.push(i_haz.clone()));
        self.quadtree.collect_collisions(entity, detected);
        irrelevant_hazards
            .iter()
            .for_each(|i_haz| detected.remove(i_haz));

        //Check if the shape is outside the quadtree
        let centroid_in_qt = self.bbox.collides_with(&entity.centroid());
        if !centroid_in_qt && detected.len() == 0 {
            // The shape centroid is outside the quadtree
            if !irrelevant_hazards.contains(&HazardEntity::BinExterior) {
                //Add the bin as a hazard, unless it is ignored
                detected.push(HazardEntity::BinExterior);
            }
        }
    }

    /// Collects all hazards with which the polygon collides.
    /// Any hazards in `irrelevant_hazards` are ignored.
    pub fn collect_poly_collisions(
        &self,
        shape: &SimplePolygon,
        irrelevant_hazards: &[HazardEntity],
    ) -> DetectionMap {
        let mut detection_map = DetectionMap::new();
        self.collect_poly_collisions_in_detector(shape, irrelevant_hazards, &mut detection_map);
        detection_map
    }

    /// Collects all hazards with which the polygon collides and stores them in the detector.
    /// Any hazards in `irrelevant_hazards` are ignored, as well as hazards present in the detector before the call.
    pub fn collect_poly_collisions_in_detector(
        &self,
        shape: &SimplePolygon,
        irrelevant_hazards: &[HazardEntity],
        detector: &mut impl HazardDetector,
    ) {
        //temporarily add the irrelevant hazards to the detector
        irrelevant_hazards
            .iter()
            .for_each(|i_haz| detector.push(i_haz.clone()));

        //collect all colliding entities due to edge intersection
        shape
            .edge_iter()
            .for_each(|e| self.quadtree.collect_collisions(&e, detector));

        //collect all colliding entities due to containment
        //TODO: check if gathering the hazards inside the bbox using the quadtree is faster
        self.all_hazards().filter(|h| h.active).for_each(|h| {
            if !detector.contains(&h.entity) && self.poly_or_hazard_are_contained(shape, h) {
                detector.push(h.entity);
            }
        });

        //drain the irrelevant hazards, leaving only the colliding entities
        irrelevant_hazards
            .iter()
            .for_each(|i_haz| detector.remove(i_haz));
    }

    /// Collects all hazards with which the surrogate collides.
    /// Any hazards in `irrelevant_hazards` are ignored.
    pub fn collect_surrogate_collisions(
        &self,
        base_surrogate: &SPSurrogate,
        transform: &Transformation,
        irrelevant_hazards: &[HazardEntity],
    ) -> DetectionMap {
        let mut detection_map = DetectionMap::new();
        self.collect_surrogate_collisions_in_detector(
            base_surrogate,
            transform,
            irrelevant_hazards,
            &mut detection_map,
        );
        detection_map
    }

    /// Collects all hazards with which the surrogate collides and stores them in the detector.
    /// Any hazards in `irrelevant_hazards` are ignored, as well as hazards present in the detector before the call.
    pub fn collect_surrogate_collisions_in_detector(
        &self,
        base_surrogate: &SPSurrogate,
        transform: &Transformation,
        irrelevant_hazards: &[HazardEntity],
        detector: &mut impl HazardDetector,
    ) {
        //temporarily add the irrelevant hazards to the buffer
        irrelevant_hazards
            .iter()
            .for_each(|i_haz| detector.push(i_haz.clone()));

        for pole in base_surrogate.ff_poles() {
            let t_pole = pole.transform_clone(transform);
            self.quadtree.collect_collisions(&t_pole, detector)
        }
        for pier in base_surrogate.ff_piers() {
            let t_pier = pier.transform_clone(transform);
            self.quadtree.collect_collisions(&t_pier, detector);
        }

        //drain the irrelevant hazards, leaving only the colliding entities
        irrelevant_hazards
            .iter()
            .for_each(|i_haz| detector.remove(i_haz));
    }
}
