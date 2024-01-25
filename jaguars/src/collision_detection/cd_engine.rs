use indexmap::IndexSet;
use itertools::Itertools;

use crate::collision_detection::cde_snapshot::CDESnapshot;
use crate::collision_detection::collision::Collides;
use crate::collision_detection::haz_prox_grid::hazard_proximity_grid::{HazardProximityGrid, PendingChangesErr};
use crate::collision_detection::hazards::hazard::Hazard;
use crate::collision_detection::hazards::hazard_entity::HazardEntity;
use crate::collision_detection::quadtree::qt_node::QTNode;
use crate::geometry::primitives::aa_rectangle::{AARectangle};
use crate::geometry::primitives::circle::Circle;
use crate::geometry::primitives::edge::Edge;
use crate::geometry::geo_traits::{CollidesWith, Shape, Transformable};
use crate::geometry::primitives::point::Point;
use crate::geometry::geo_enums::{GeoPosition, GeoRelation};
use crate::geometry::primitives::simple_polygon::SimplePolygon;
use crate::geometry::primitives::sp_surrogate::SPSurrogate;
use crate::geometry::transformation::Transformation;
use crate::util::assertions;
use crate::util::config::{CDEConfig, QuadTreeConfig};

#[derive(Clone, Debug)]
pub struct CDEngine {
    quadtree: QTNode,
    static_hazards: Vec<Hazard>,
    dynamic_hazards: Vec<Hazard>,
    haz_prox_grid: HazardProximityGrid,
    config: CDEConfig,
    bbox: AARectangle,
    uncommited_deregisters: Vec<Hazard>,
}

impl CDEngine {
    pub fn new(bbox: AARectangle, static_hazards: Vec<Hazard>, config: CDEConfig) -> CDEngine {
        let qt_depth = match config.quadtree {
            QuadTreeConfig::FixedDepth(depth) => depth,
            QuadTreeConfig::Auto => panic!("not implemented, quadtree depth must be specified"),
        };

        let haz_prox_grid = HazardProximityGrid::new(bbox.clone(), &static_hazards, config.haz_prox);
        let mut qt_root = QTNode::new(qt_depth, bbox.clone(), None);

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
            uncommited_deregisters: vec![],
        }
    }

    //UPDATE ---------------------------------------------------------------------------------------

    pub fn register_hazard(&mut self, hazard: Hazard) {
        debug_assert!(self.dynamic_hazards.iter().find(|h| h.entity() == hazard.entity()).is_none(), "Hazard already registered");
        let hazard_in_uncommitted_deregs = self.uncommited_deregisters.iter().position(|h| h.entity() == hazard.entity());

        let hazard = match hazard_in_uncommitted_deregs {
            Some(index) => {
                let unc_hazard = self.uncommited_deregisters.swap_remove(index);
                self.quadtree.activate_hazard(unc_hazard.entity());
                unc_hazard
            }
            None => {
                self.quadtree.register_hazard((&hazard).into());
                hazard
            }
        };
        self.haz_prox_grid.register_hazard(&hazard);
        self.dynamic_hazards.push(hazard);

        debug_assert!(assertions::qt_contains_no_dangling_hazards(&self));
    }

    pub fn deregister_hazard(&mut self, hazard_entity: &HazardEntity, commit_instantly: bool) {
        let haz_index = self.dynamic_hazards.iter().position(|h| h.entity() == hazard_entity).expect("Hazard not found");

        let hazard = self.dynamic_hazards.swap_remove(haz_index);

        match commit_instantly {
            true => self.quadtree.deregister_hazard(hazard_entity),
            false => {
                self.quadtree.deactivate_hazard(hazard_entity);
                self.uncommited_deregisters.push(hazard);
            }
        }
        self.haz_prox_grid.deregister_hazard(hazard_entity, self.dynamic_hazards.iter(), commit_instantly);

        debug_assert!(assertions::qt_contains_no_dangling_hazards(&self));
    }

    pub fn create_snapshot(&mut self) -> CDESnapshot {
        self.commit_deregisters();
        assert!(!self.haz_prox_grid.has_pending_deregisters());
        CDESnapshot::new(
            self.dynamic_hazards.clone(),
            self.haz_prox_grid.grid().clone(),
            self.haz_prox_grid.value().unwrap(),
        )
    }

    pub fn restore(&mut self, snapshot: &CDESnapshot) {
        //QUADTREE
        let mut hazards_to_remove = self.dynamic_hazards.iter().map(|h| h.entity().clone()).collect::<IndexSet<HazardEntity>>();
        debug_assert!(hazards_to_remove.len() == self.dynamic_hazards.len());
        let mut hazards_to_add = vec![];

        for hazard in snapshot.dynamic_hazards().iter() {
            let hazard_already_present = hazards_to_remove.remove(hazard.entity());
            if !hazard_already_present {
                hazards_to_add.push(hazard.clone());
            }
        }

        //Hazards currently registered in the CDE, but not in the snapshot
        for hazard in hazards_to_remove.iter() {
            let haz_index = self.dynamic_hazards.iter().position(|h| h.entity() == hazard).expect("Hazard not found");
            self.dynamic_hazards.swap_remove(haz_index);
            self.quadtree.deregister_hazard(&hazard);
        }

        //Some of the uncommitted deregisters might be in present in snapshot, if so we can just reactivate them
        for unc_haz in self.uncommited_deregisters.drain(..) {
            if let Some(pos) = hazards_to_add.iter().position(|h| h.entity() == unc_haz.entity()) {
                //the uncommitted removed hazard needs to be activated again
                self.quadtree.activate_hazard(unc_haz.entity());
                self.dynamic_hazards.push(unc_haz);
                hazards_to_add.swap_remove(pos);
            } else {
                //uncommitted deregister is not preset in the snapshot, delete it from the quadtree
                self.quadtree.deregister_hazard(unc_haz.entity());
            }
        }

        for hazard in hazards_to_add {
            self.quadtree.register_hazard((&hazard).into());
            self.dynamic_hazards.push(hazard);
        }

        //HAZPROXGRID
        self.haz_prox_grid.restore(snapshot.grid().clone(), snapshot.grid_value());

        debug_assert!(self.dynamic_hazards.len() == snapshot.dynamic_hazards().len());
    }

    fn commit_deregisters(&mut self) {
        for uc_haz in self.uncommited_deregisters.drain(..) {
            self.quadtree.deregister_hazard(uc_haz.entity());
        }
        self.haz_prox_grid.flush_deregisters(self.dynamic_hazards.iter());
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

    pub fn smallest_qt_node_dimension(&self) -> f64 {
        let bbox = self.quadtree.bbox();
        let level = self.quadtree.level();
        //every level, the dimension is halved
        bbox.width() / 2.0_f64.powi(level as i32)
    }

    pub fn config(&self) -> CDEConfig {
        self.config
    }

    pub fn haz_prox_grid(&self) -> Result<&HazardProximityGrid, PendingChangesErr> {
        match self.haz_prox_grid.has_pending_deregisters() {
            true => Err(PendingChangesErr),
            false => Ok(&self.haz_prox_grid)
        }
    }

    pub fn flush_changes(&mut self) {
        self.haz_prox_grid.flush_deregisters(self.dynamic_hazards.iter());
    }

    pub fn has_uncommitted_deregisters(&self) -> bool {
        self.uncommited_deregisters.len() > 0
    }

    pub fn dynamic_hazards(&self) -> &Vec<Hazard> {
        &self.dynamic_hazards
    }

    pub fn static_hazards(&self) -> &Vec<Hazard> {
        &self.static_hazards
    }

    pub fn all_hazards(&self) -> impl Iterator<Item=&Hazard> {
        self.static_hazards.iter().chain(self.dynamic_hazards.iter())
    }

    //QUERY ----------------------------------------------------------------------------------------
    pub fn poly_collides(&self, shape: &SimplePolygon, ignored_entities: Option<&Vec<&HazardEntity>>) -> bool {
        match self.bbox.relation_to(&shape.bbox()) {
            GeoRelation::Disjoint | GeoRelation::Enclosed | GeoRelation::Intersecting => {
                return true;
            } //Not fully inside quadtree -> definite collision
            GeoRelation::Surrounding => {
                //Intersection test
                if shape.edge_iter().any(|e| self.quadtree.edge_collides(&e, ignored_entities).is_some()) {
                    return true;
                }

                //Inclusion test
                if self.collision_by_inclusion(shape, ignored_entities) {
                    return true;
                }
                return false;
            }
        }
    }

    pub fn surrogate_collides(&self, base_surrogate: &SPSurrogate, transform: &Transformation, ignored_entities: Option<&Vec<&HazardEntity>>) -> bool {
        let ff_range_poles = base_surrogate.config().ff_range_poles();
        let ff_range_clips = base_surrogate.config().ff_range_clips();

        for pole in &base_surrogate.poles()[ff_range_poles] {
            let t_pole = pole.transform_clone(transform);
            if self.quadtree.circle_collides(&t_pole, ignored_entities).is_some() {
                return true;
            }
        }
        for clip in &base_surrogate.clips()[ff_range_clips] {
            let t_clip = clip.transform_clone(transform);
            if self.quadtree.edge_collides(&t_clip, ignored_entities).is_some() {
                return true;
            }
        }
        false
    }

    pub fn point_definitely_collides_with(&self, point: &Point, entity: &HazardEntity) -> Collides {
        if !self.bbox.collides_with(point) {
            //point is outside of the quadtree, so no information available
            Collides::Unsure
        } else {
            self.quadtree.point_definitely_collides_with(point, entity)
        }
    }

    pub fn edge_definitely_collides(&self, edge: &Edge, ignored_entities: Option<&Vec<&HazardEntity>>) -> bool {
        match !self.bbox.collides_with(&edge.start()) || !self.bbox.collides_with(&edge.end()) {
            true => {
                //if either the start or end of the edge is outside the quadtree, it definitely collides
                true
            }
            false => {
                self.quadtree.edge_definitely_collides(edge, ignored_entities) == Collides::Yes
            }
        }
    }

    pub fn circle_definitely_collides(&self, circle: &Circle, ignored_entities: Option<&Vec<&HazardEntity>>) -> bool {
        match !self.bbox.collides_with(&circle.center()) {
            true => true,
            false => {
                //let haz_entities_to_ignore = hazard_filter::ignored_entities(hazard_filter, &mut self.all_hazards());
                self.quadtree.circle_definitely_collides(circle, ignored_entities) == Collides::Yes
            }
        }
    }

    pub fn entities_in_circle(&self, circle: &Circle, ignored_entities: Option<&Vec<&HazardEntity>>) -> Vec<HazardEntity> {
        let mut colliding_entities = vec![];
        let mut ignored_entities = ignored_entities.map_or(vec![], |v| v.clone());

        //Keep testing the quadtree for intersections until no (non-ignored) entities collide
        loop {
            match self.quadtree.circle_collides(circle, Some(&ignored_entities)) {
                Some(e) => {
                    colliding_entities.push(e.clone());
                    ignored_entities.push(e);
                }
                None => break
            }
        }

        let circle_center_in_bbox = self.bbox.collides_with(&circle.center());

        if !circle_center_in_bbox && colliding_entities.is_empty() {
            if !ignored_entities.contains(&&HazardEntity::BinOuter) {
                //If the center of the circle falls outside the scope of the quadtree, add the bin as a hazard, unless it is ignored
                colliding_entities.push(HazardEntity::BinOuter);
            }
        }

        colliding_entities
    }

    fn collision_by_inclusion(&self, shape: &SimplePolygon, ignored_entities: Option<&Vec<&HazardEntity>>) -> bool
    {
        //TODO: restructure to improve readability

        let mut relevant_hazards = self.all_hazards()
            .filter(|h| h.is_active())
            .filter(|h| ignored_entities.map_or(true, |e| !e.contains(&h.entity())));

        let shape_point = shape.surrogate().pole_of_inaccessibility().center();

        relevant_hazards.any(|haz| {
            let haz_point = haz.shape().surrogate().pole_of_inaccessibility().center();

            let bbox_relation = haz.shape().bbox().relation_to(&shape.bbox());

            let (point, poly) = match (bbox_relation, haz.entity().presence()) {
                (GeoRelation::Disjoint, GeoPosition::Exterior) => return true, //exclusion collision
                (GeoRelation::Disjoint, GeoPosition::Interior) => return false, //no inclusion collision possible with unrelated bboxes
                (GeoRelation::Intersecting, GeoPosition::Exterior) => (shape_point, haz.shape().as_ref()), //exclusion collision possible
                (GeoRelation::Surrounding, _) => (shape_point, haz.shape().as_ref()), //collision possible
                (GeoRelation::Enclosed, _) => (haz_point, shape), //collision possible
                (GeoRelation::Intersecting, GeoPosition::Interior) => {
                    //Due to limited fp precision, we need to check if the bboxes are almost related
                    match haz.shape().bbox().almost_relation_to(&shape.bbox()) {
                        GeoRelation::Enclosed => (haz_point, shape), //collision possible
                        GeoRelation::Surrounding => (shape_point, haz.shape().as_ref()), //collision possible
                        _ => return false, //no inclusion possible with intersecting bboxes
                    }
                }
            };

            if std::ptr::eq(poly, haz.shape().as_ref()) {
                //The poly to test against is the hazard, which is registered in the quadtree.
                //We can use the quadtree for possibly a faster resolve time
                match self.quadtree.point_definitely_collides_with(&point, haz.entity()) {
                    Collides::Yes => return true, //inclusion or exclusion collision
                    Collides::No => return false,
                    Collides::Unsure => (),
                }
            }

            match (haz.entity().presence(), poly.collides_with(&point)) {
                (GeoPosition::Interior, true) => true, //inclusion collision
                (GeoPosition::Exterior, false) => true, //exclusion collision
                (GeoPosition::Exterior, true) => false,
                (GeoPosition::Interior, false) => false,
            }
        })
    }
}