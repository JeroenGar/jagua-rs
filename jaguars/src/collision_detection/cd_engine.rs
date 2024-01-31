use indexmap::IndexSet;
use itertools::Itertools;
use tribool::Tribool;

use crate::collision_detection::cde_snapshot::CDESnapshot;
use crate::collision_detection::haz_prox_grid::hazard_proximity_grid::{HazardProximityGrid, PendingChangesErr};
use crate::collision_detection::hazards::hazard::Hazard;
use crate::collision_detection::hazards::hazard_entity::HazardEntity;
use crate::collision_detection::quadtree::qt_node::QTNode;
use crate::geometry::fail_fast::sp_surrogate::SPSurrogate;
use crate::geometry::geo_enums::{GeoPosition, GeoRelation};
use crate::geometry::geo_traits::{CollidesWith, Shape, Transformable};
use crate::geometry::primitives::aa_rectangle::AARectangle;
use crate::geometry::primitives::circle::Circle;
use crate::geometry::primitives::edge::Edge;
use crate::geometry::primitives::point::Point;
use crate::geometry::primitives::simple_polygon::SimplePolygon;
use crate::geometry::transformation::Transformation;
use crate::util::assertions;
use crate::util::config::{CDEConfig, HazProxConfig, QuadTreeConfig};

#[derive(Clone, Debug)]
pub struct CDEngine {
    quadtree: QTNode,
    static_hazards: Vec<Hazard>,
    dynamic_hazards: Vec<Hazard>,
    haz_prox_grid: Option<HazardProximityGrid>,
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

        let haz_prox_grid = match config.haz_prox {
            HazProxConfig::Disabled => None,
            HazProxConfig::Enabled { n_cells } => {
                Some(HazardProximityGrid::new(bbox.clone(), &static_hazards, n_cells))
            }
        };

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
        self.haz_prox_grid.as_mut().map(|hpg| hpg.register_hazard(&hazard));
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
        self.haz_prox_grid.as_mut().map(|hpg| hpg.deregister_hazard(hazard_entity, self.dynamic_hazards.iter(), commit_instantly));

        debug_assert!(assertions::qt_contains_no_dangling_hazards(&self));
    }

    pub fn create_snapshot(&mut self) -> CDESnapshot {
        self.commit_deregisters();
        assert!(!self.haz_prox_grid.as_ref().map_or(false, |hpg| hpg.has_pending_deregisters()));
        CDESnapshot::new(
            self.dynamic_hazards.clone(),
            self.haz_prox_grid.as_ref().map(|hpg| hpg.grid().clone()),
        )
    }

    pub fn restore(&mut self, snapshot: &CDESnapshot) {
        //Quadtree
        let mut hazards_to_remove = self.dynamic_hazards.iter().map(|h| h.entity().clone()).collect::<IndexSet<HazardEntity>>();
        debug_assert!(hazards_to_remove.len() == self.dynamic_hazards.len());
        let mut hazards_to_add = vec![];

        for hazard in snapshot.dynamic_hazards().iter() {
            let hazard_already_present = hazards_to_remove.swap_remove(hazard.entity());
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

        //Hazard proximity grid
        self.haz_prox_grid.as_mut().map(|hpg| {
            hpg.restore(snapshot.grid().clone().expect("no hpg in snapshot"));
        });

        debug_assert!(self.dynamic_hazards.len() == snapshot.dynamic_hazards().len());
    }

    fn commit_deregisters(&mut self) {
        for uc_haz in self.uncommited_deregisters.drain(..) {
            self.quadtree.deregister_hazard(uc_haz.entity());
        }
        self.haz_prox_grid.as_mut().map(|hpg| hpg.flush_deregisters(self.dynamic_hazards.iter()));
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
        let grid = self.haz_prox_grid.as_ref().expect("no hpg present");
        match grid.has_pending_deregisters() {
            true => Err(PendingChangesErr),
            false => Ok(grid)
        }
    }

    pub fn flush_changes(&mut self) {
        self.haz_prox_grid.as_mut().map(|hpg| hpg.flush_deregisters(self.dynamic_hazards.iter()));
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
    pub fn poly_collides(&self, shape: &SimplePolygon, ignored_entities: &[HazardEntity]) -> bool {
        match self.bbox.relation_to(&shape.bbox()) {
            GeoRelation::Disjoint | GeoRelation::Enclosed | GeoRelation::Intersecting => {
                return true;
            } //Not fully inside quadtree -> definite collision
            GeoRelation::Surrounding => {
                //Intersection test
                if shape.edge_iter().any(|e| self.quadtree.collides(&e, ignored_entities).is_some()) {
                    return true;
                }

                //Containment test
                if self.collision_by_containment(shape, ignored_entities) {
                    return true;
                }
                return false;
            }
        }
    }

    pub fn surrogate_collides(&self, base_surrogate: &SPSurrogate, transform: &Transformation, ignored_entities: &[HazardEntity]) -> bool {
        for pole in base_surrogate.ff_poles() {
            let t_pole = pole.transform_clone(transform);
            if self.quadtree.collides(&t_pole, ignored_entities).is_some() {
                return true;
            }
        }
        for pier in base_surrogate.piers() {
            let t_pier = pier.transform_clone(transform);
            if self.quadtree.collides(&t_pier, ignored_entities).is_some() {
                return true;
            }
        }
        false
    }

    pub fn point_definitely_collides_with(&self, point: &Point, entity: &HazardEntity) -> Tribool {
        match self.bbox.collides_with(point) {
            false => Tribool::Indeterminate, //point is outside the quadtree, so no information available
            true => self.quadtree.point_definitely_collides_with(point, entity)
        }
    }

    pub fn edge_definitely_collides(&self, edge: &Edge, ignored_entities: &[HazardEntity]) -> Tribool {
        match !self.bbox.collides_with(&edge.start()) || !self.bbox.collides_with(&edge.end()) {
            true => Tribool::True, //if either the start or end of the edge is outside the quadtree, it definitely collides
            false => self.quadtree.definitely_collides(edge, ignored_entities)
        }
    }

    pub fn circle_definitely_collides(&self, circle: &Circle, ignored_entities: &[HazardEntity]) -> Tribool {
        match self.bbox.collides_with(&circle.center()) {
            false => Tribool::True, //outside the quadtree, so definitely collides
            true => self.quadtree.definitely_collides(circle, ignored_entities)
        }
    }

    pub fn entities_in_circle(&self, circle: &Circle, ignored_entities: &[HazardEntity]) -> Vec<HazardEntity> {
        let mut colliding_entities = vec![];
        let mut ignored_entities = ignored_entities.iter().cloned().collect_vec();

        //Keep testing the quadtree for intersections until no (non-ignored) entities collide
        while let Some(haz_entity) = self.quadtree.collides(circle, &ignored_entities) {
            colliding_entities.push(haz_entity.clone());
            ignored_entities.push(haz_entity.clone());
        }

        let circle_center_in_qt = self.bbox.collides_with(&circle.center());

        if !circle_center_in_qt && colliding_entities.is_empty() {
            // The circle center is outside the quadtree
            if !ignored_entities.contains(&&HazardEntity::BinOuter) {
                //Add the bin as a hazard, unless it is ignored
                colliding_entities.push(HazardEntity::BinOuter);
            }
        }

        colliding_entities
    }

    fn collision_by_containment(&self, shape: &SimplePolygon, ignored_entities: &[HazardEntity]) -> bool
    {
        //collect all active and non-ignored hazards
        let mut relevant_hazards = self.all_hazards()
            .filter(|h| h.is_active())
            .filter(|h| !ignored_entities.contains(h.entity()));

        relevant_hazards.any(|haz| {
            //due to possible floating point precision issues, we need to check if the bboxes are "almost" related
            //"almost" meaning that, when edges are close together, they are considered equal.
            //Which results in Intersecting relations being considered Enclosed or Surrounding
            let haz_shape = haz.shape().as_ref();
            let bbox_relation = haz_shape.bbox().almost_relation_to(&shape.bbox());

            let (s_mu, s_omega) = match bbox_relation {
                GeoRelation::Surrounding => (shape, haz_shape), //inclusion possible
                GeoRelation::Enclosed => (haz_shape, shape), //inclusion possible
                GeoRelation::Disjoint | GeoRelation::Intersecting => {
                    //no inclusion is possible
                    match haz.entity().presence() {
                        GeoPosition::Interior => return false,
                        GeoPosition::Exterior => return true,
                    }
                },
            };

            if std::ptr::eq(haz_shape, s_omega) {
                //s_omega is registered in the quadtree.
                //maybe the quadtree can help us.
                match self.quadtree.point_definitely_collides_with(&s_mu.poi().center(), haz.entity()).try_into() {
                    Ok(collides) => return collides,
                    Err(_) => (), //no definitive answer
                }
            }
            let inclusion = s_omega.collides_with(&s_mu.poi().center());

            match haz.entity().presence() {
                GeoPosition::Interior => inclusion,
                GeoPosition::Exterior => !inclusion,
            }
        })
    }
}