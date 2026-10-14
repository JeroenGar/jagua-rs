use crate::collision_detection::hazard_index::HazardIndex;
use crate::collision_detection::hazards::HazKey;
use crate::collision_detection::hazards::Hazard;
use crate::collision_detection::hazards::HazardGeometry;
use crate::collision_detection::hazards::collector::HazardCollector;
use crate::collision_detection::hazards::filter::HazardFilter;
use crate::collision_detection::hazards::{BasicHazardEntity, HazardEntity};
use crate::collision_detection::narrow_phase::{self, EdgeChains};
use crate::geometry::Transformation;
use crate::geometry::fail_fast::{SPSurrogate, SPSurrogateConfig};
use crate::geometry::geo_enums::{GeoPosition, GeoRelation};
use crate::geometry::geo_traits::{CollidesWith, Transformable};
use crate::geometry::primitives::{Circle, Rect, SPolygon};
use crate::util::assertions;
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use slotmap::{SecondaryMap, SlotMap};

/// The Collision Detection Engine (CDE).
/// [`Hazard`]s can be (de)registered and collision queries can be performed.
/// The entity type `E` defines what can induce a hazard, see [`HazardEntity`].
#[derive(Clone, Debug)]
pub struct CDEngine<E = BasicHazardEntity> {
    /// Shapes reaching outside this rectangle collide with the exterior
    bbox: Rect,
    /// All hazards registered in the CDE (active and inactive)
    pub(crate) hazards_map: SlotMap<HazKey, Hazard<E>>,
    /// Configuration of the CDE
    pub(crate) config: CDEConfig,
    /// The key of the hazard that represents the exterior of the container.
    hkey_exterior: HazKey,
    /// Bounding boxes of all hazards except the exterior
    pub(crate) index: HazardIndex,
    /// Keys of the hazards the CDE was created with, which snapshots leave out
    base_keys: SecondaryMap<HazKey, ()>,
}

impl<E: HazardEntity> CDEngine<E> {
    /// Creates an engine with the static hazards, exactly one of which covers the exterior.
    /// Snapshots hold only the hazards registered later.
    #[must_use]
    pub fn new(bbox: Rect, static_hazards: Vec<Hazard<E>>, config: CDEConfig) -> Self {
        let mut hazards_map = SlotMap::with_key();
        for haz in static_hazards {
            hazards_map.insert(haz);
        }
        let base_keys = hazards_map.keys().map(|k| (k, ())).collect();

        let mut exteriors = hazards_map
            .iter()
            .filter(|(_, h)| h.entity().scope() == GeoPosition::Exterior)
            .map(|(hkey, _)| hkey);
        let hkey_exterior = exteriors
            .next()
            .expect("No exterior hazard registered in the CDE");
        assert!(
            exteriors.next().is_none(),
            "More than one exterior hazard registered in the CDE"
        );

        let mut index = HazardIndex::default();
        for (hkey, haz) in &hazards_map {
            if hkey != hkey_exterior {
                index.insert(hkey, haz.shape().bbox);
            }
        }

        let cde = CDEngine {
            bbox,
            hazards_map,
            config,
            hkey_exterior,
            index,
            base_keys,
        };
        debug_assert!(assertions::index_matches_hazards(&cde));
        cde
    }

    /// Registers a new hazard in the CDE.
    pub fn register_hazard(&mut self, hazard: Hazard<E>) {
        debug_assert!(
            !self
                .hazards_map
                .values()
                .any(|h| h.entity() == hazard.entity()),
            "Hazard with an identical entity already registered"
        );
        debug_assert!(
            hazard.entity().scope() == GeoPosition::Interior,
            "The exterior hazard can only be registered at construction"
        );
        let bbox = hazard.shape().bbox;
        let hkey = self.hazards_map.insert(hazard);
        self.index.insert(hkey, bbox);
        debug_assert!(assertions::index_matches_hazards(self));
    }

    /// Removes a hazard from the CDE.
    pub fn deregister_hazard_by_entity(&mut self, hazard_entity: E) -> Hazard<E> {
        let hkey = self
            .hazards_map
            .iter()
            .find(|(_, h)| h.entity() == hazard_entity)
            .map(|(hkey, _)| hkey)
            .expect("Cannot deregister hazard that is not registered");
        self.deregister_hazard_by_key(hkey)
    }

    /// Removes a hazard registered after construction from the CDE.
    pub fn deregister_hazard_by_key(&mut self, hkey: HazKey) -> Hazard<E> {
        debug_assert!(
            !self.base_keys.contains_key(hkey),
            "Hazards the CDE was created with cannot be deregistered"
        );
        let hazard = self
            .hazards_map
            .remove(hkey)
            .expect("Cannot deregister hazard that is not registered");
        self.index.remove(hkey, hazard.shape().bbox);
        debug_assert!(assertions::index_matches_hazards(self));

        hazard
    }

    /// Saves the hazards registered after construction, to restore them later with
    /// [`CDEngine::restore`].
    #[must_use]
    pub fn save(&self) -> CDESnapshot<E> {
        let dynamic_hazards = self
            .hazards_map
            .iter()
            .filter(|(hkey, _)| !self.base_keys.contains_key(*hkey))
            .map(|(_, h)| h.clone())
            .collect_vec();
        CDESnapshot { dynamic_hazards }
    }

    /// Restores the hazards registered after construction to those of a snapshot, keeping
    /// those present in both.
    pub fn restore(&mut self, snapshot: &CDESnapshot<E>) {
        //Restore the hazards, by doing a 'diff' between the current state and the snapshot

        //Determine which hazards need to be removed and which need to be added
        let mut hazards_to_remove = self
            .hazards_map
            .iter()
            .filter(|(hkey, _)| !self.base_keys.contains_key(*hkey))
            .map(|(hkey, h)| (hkey, h.entity()))
            .collect_vec();
        let mut hazards_to_add = vec![];

        for hazard in &snapshot.dynamic_hazards {
            let present = hazards_to_remove
                .iter()
                .position(|(_, h)| *h == hazard.entity());
            if let Some(idx) = present {
                //the hazard is already present in the CDE, remove it from the hazards to remove
                hazards_to_remove.swap_remove(idx);
            } else {
                //the hazard is not present in the CDE, add it to the list of hazards to add
                hazards_to_add.push(hazard.clone());
            }
        }

        //Remove all hazards currently in the CDE but not in the snapshot
        for (hkey, _) in hazards_to_remove {
            self.deregister_hazard_by_key(hkey);
        }

        //Add all hazards in the snapshot but not currently in the CDE
        for hazard in hazards_to_add {
            self.register_hazard(hazard);
        }

        debug_assert_eq!(
            self.hazards_map.len() - self.base_keys.len(),
            snapshot.dynamic_hazards.len()
        );
    }

    /// Returns all hazards in the CDE
    pub fn hazards(&self) -> impl Iterator<Item = &Hazard<E>> {
        self.hazards_map.values()
    }

    /// Checks whether a simple polygon collides with any of the (relevant) hazards
    /// # Arguments
    /// * `shape` - The shape (already transformed) to be checked for collisions
    /// * `filter` - Hazard filter to be applied
    pub fn detect_poly_collision(&self, shape: &SPolygon, filter: &impl HazardFilter<E>) -> bool {
        if self.bbox.relation_to(shape.bbox) != GeoRelation::Surrounding {
            //The CDE does not capture the entire shape, so we can immediately return true
            return true;
        }
        let exterior = self.hazards_map[self.hkey_exterior].entity();
        if !filter.is_irrelevant(self.hkey_exterior, &exterior)
            && self.collides_with_exterior(shape)
        {
            return true;
        }
        self.index.any_overlapping(shape.bbox, |hkey| {
            let haz = &self.hazards_map[hkey];
            !filter.is_irrelevant(hkey, &haz.entity()) && Self::poly_collides_with(shape, haz)
        })
    }

    /// Checks whether a surrogate collides with any of the (relevant) hazards.
    /// A `true` result proves a collision for the represented polygon.
    /// A `false` result is inconclusive; follow with [`Self::detect_poly_collision`].
    /// # Arguments
    /// * `base_surrogate` - The (untransformed) surrogate to be checked for collisions
    /// * `transform` - The transformation to be applied to the surrogate (on the fly)
    /// * `filter` - Hazard filter to be applied
    pub fn detect_surrogate_collision(
        &self,
        base_surrogate: &SPSurrogate,
        transform: &Transformation,
        filter: &impl HazardFilter<E>,
    ) -> bool {
        let exterior = self.hazards_map[self.hkey_exterior].entity();
        let check_exterior = !filter.is_irrelevant(self.hkey_exterior, &exterior);
        for pole in base_surrogate.ff_poles() {
            let pole = pole.transform_clone(transform);
            if check_exterior && self.pole_collides_with_exterior(&pole) {
                return true;
            }
            let hit = self.index.any_overlapping(pole.bbox(), |hkey| {
                let haz = &self.hazards_map[hkey];
                !filter.is_irrelevant(hkey, &haz.entity()) && Self::pole_collides_with(&pole, haz)
            });
            if hit {
                return true;
            }
        }
        false
    }

    /// Whether `shape` collides with the interior hazard `haz`, given that their bounding boxes overlap.
    fn poly_collides_with(shape: &SPolygon, haz: &Hazard<E>) -> bool {
        let r = narrow_phase::overlap(shape.bbox, haz.shape().bbox)
            .expect("the index reports overlapping bounding boxes");
        narrow_phase::edges_intersect(shape, r, haz)
            || Self::detect_containment_collision(shape, haz.shape(), haz.entity())
    }

    /// Whether `pole` collides with the interior hazard `haz`.
    fn pole_collides_with(pole: &Circle, haz: &Hazard<E>) -> bool {
        // A pole touching no edge collides only if it lies inside the hazard,
        // which requires its bounding box to lie inside the hazard's
        narrow_phase::circle_touches_edges(pole, haz)
            || (haz.shape().bbox.relation_to(pole.bbox()) == GeoRelation::Surrounding
                && haz.shape().collides_with(&pole.center))
    }

    /// Check for collision by containment between a shape and a hazard.
    /// This only guarantees to detect collisions caused by full containment of one shape in another.
    /// # Arguments
    /// * `shape` - The shape to be checked for containment
    /// * `haz_shape` - The shape of the respective hazard
    /// * `haz_entity` - The entity inducing the hazard
    #[must_use]
    fn detect_containment_collision(shape: &SPolygon, haz_shape: &SPolygon, haz_entity: E) -> bool {
        //Due to possible fp issues, we check if the bboxes are "almost" related --
        //meaning that, when edges are very close together, they are considered equal.
        //Some relations which would normally be seen as `Intersecting` are now being considered `Enclosed`/`Surrounding` (which triggers the containment check).
        let haz_to_shape_bbox_relation = haz_shape.bbox.almost_relation_to(shape.bbox);

        //If the bounding boxes are contained, we have to check the actual shapes for containment.
        //This can be done by testing whether a single point of the smaller shape is contained in the larger shape.
        let contained = match haz_to_shape_bbox_relation {
            GeoRelation::Surrounding => haz_shape.collides_with(&shape.poi.center),
            GeoRelation::Enclosed => shape.collides_with(&haz_shape.poi.center),
            GeoRelation::Disjoint | GeoRelation::Intersecting => false,
        };

        //Depending on the scope of the hazard this results a collision or not
        match (haz_entity.scope(), contained) {
            (GeoPosition::Interior, true) | (GeoPosition::Exterior, false) => true,
            (GeoPosition::Interior, false) | (GeoPosition::Exterior, true) => false,
        }
    }

    /// Collects all hazards with which the polygon collides and reports them to the collector.
    /// # Arguments
    /// * `shape` - The shape to be checked for collisions
    /// * `filter` - Hazards to ignore
    /// * `collector` - The collector to which the hazards are reported
    pub fn collect_poly_collisions(
        &self,
        shape: &SPolygon,
        filter: &impl HazardFilter<E>,
        collector: &mut impl HazardCollector<Entity = E>,
    ) {
        let _ = self.collect_poly_collisions_until(shape, filter, collector, |_| false);
    }

    /// Collects polygon collisions until `stop_after_collision` requests an early return.
    ///
    /// The callback receives each colliding hazard's entity after it is added to the collector for
    /// the first time. Returning `true` stops traversal and leaves the collector with only the
    /// hazards found up to that point. Collision order is unspecified.
    #[must_use]
    pub fn collect_poly_collisions_until<H, C, F>(
        &self,
        shape: &SPolygon,
        filter: &H,
        collector: &mut C,
        mut stop_after_collision: F,
    ) -> bool
    where
        H: HazardFilter<E>,
        C: HazardCollector<Entity = E>,
        F: FnMut(E) -> bool,
    {
        let exterior = self.hazards_map[self.hkey_exterior].entity();
        if !collector.contains_key(self.hkey_exterior)
            && !filter.is_irrelevant(self.hkey_exterior, &exterior)
            && (self.bbox.relation_to(shape.bbox) != GeoRelation::Surrounding
                || self.collides_with_exterior(shape))
        {
            collector.insert(self.hkey_exterior, exterior);
            if stop_after_collision(exterior) {
                return true;
            }
        }

        // Every hazard whose bounding box overlaps the shape's is checked once
        self.index.any_overlapping(shape.bbox, |hkey| {
            if collector.contains_key(hkey) {
                return false;
            }
            let haz = &self.hazards_map[hkey];
            if filter.is_irrelevant(hkey, &haz.entity()) {
                return false;
            }
            if Self::poly_collides_with(shape, haz) {
                collector.insert(hkey, haz.entity());
                return stop_after_collision(haz.entity());
            }
            false
        })
    }

    /// Collects collisions found by the surrogate screening pass.
    ///
    /// This checks a prefix of the largest poles, not the complete polygon. Follow it with
    /// [`Self::collect_poly_collisions`] when every collision must be collected.
    pub fn collect_surrogate_collisions(
        &self,
        shape: &SPolygon,
        filter: &impl HazardFilter<E>,
        collector: &mut impl HazardCollector<Entity = E>,
    ) {
        let _ = self.collect_surrogate_collisions_until(shape, filter, collector, |_| false);
    }

    /// Collects collisions found by the surrogate screening pass until
    /// `stop_after_collision` requests an early return.
    ///
    /// The callback contract matches [`Self::collect_poly_collisions_until`]. The configured
    /// fail-fast pole prefix is checked largest-first. This is not a complete polygon collision
    /// query; follow it with [`Self::collect_poly_collisions_until`] if the callback does not stop
    /// the screening pass.
    #[must_use]
    pub fn collect_surrogate_collisions_until<H, C, F>(
        &self,
        shape: &SPolygon,
        filter: &H,
        collector: &mut C,
        mut stop_after_collision: F,
    ) -> bool
    where
        H: HazardFilter<E>,
        C: HazardCollector<Entity = E>,
        F: FnMut(E) -> bool,
    {
        let Some(surrogate) = &shape.surrogate else {
            return false;
        };
        let poles = surrogate.ff_poles();
        let exterior = self.hazards_map[self.hkey_exterior].entity();
        for pole in poles {
            if !collector.contains_key(self.hkey_exterior)
                && !filter.is_irrelevant(self.hkey_exterior, &exterior)
                && self.pole_collides_with_exterior(pole)
            {
                collector.insert(self.hkey_exterior, exterior);
                if stop_after_collision(exterior) {
                    return true;
                }
            }
        }

        // One scan of the index with the poles' combined bounding box
        let Some(union) = poles.iter().map(Circle::bbox).reduce(Rect::bounding_rect) else {
            return false;
        };
        self.index
            .any_overlapping_each(union, &poles.iter().map(Circle::bbox), |i, hkey| {
                if collector.contains_key(hkey) {
                    return false;
                }
                let haz = &self.hazards_map[hkey];
                if filter.is_irrelevant(hkey, &haz.entity()) {
                    return false;
                }
                if Self::pole_collides_with(&poles[i], haz) {
                    collector.insert(hkey, haz.entity());
                    return stop_after_collision(haz.entity());
                }
                false
            })
    }

    /// The exterior hazard and its geometry.
    fn exterior(&self) -> (&Hazard<E>, &EdgeChains, bool) {
        let haz = &self.hazards_map[self.hkey_exterior];
        match haz.geometry() {
            HazardGeometry::Exterior { chains, is_rect } => (haz, chains, *is_rect),
            HazardGeometry::Interior(_) => unreachable!("the exterior has exterior geometry"),
        }
    }

    /// Whether `shape` collides with the exterior hazard.
    fn collides_with_exterior(&self, shape: &SPolygon) -> bool {
        let (haz, chains, is_rect) = self.exterior();
        let ext_bbox = haz.shape().bbox;
        if is_rect && strictly_inside(shape.bbox, ext_bbox) {
            return false;
        }
        let Some(r) = narrow_phase::overlap(shape.bbox, ext_bbox) else {
            return true;
        };
        narrow_phase::edges_intersect(shape, r, haz)
            || match ext_bbox.almost_relation_to(shape.bbox) {
                // No edges cross: the shape lies inside or outside the container as a whole
                GeoRelation::Surrounding => !chains.contains(shape.poi.center),
                _ => true,
            }
    }

    /// Whether `pole` collides with the exterior hazard.
    fn pole_collides_with_exterior(&self, pole: &Circle) -> bool {
        let (haz, chains, is_rect) = self.exterior();
        if is_rect && strictly_inside(pole.bbox(), haz.shape().bbox) {
            return false;
        }
        narrow_phase::circle_touches_edges(pole, haz) || !chains.contains(pole.center)
    }

    /// Shapes reaching outside this rectangle collide with the exterior.
    #[must_use]
    pub fn bbox(&self) -> Rect {
        self.bbox
    }

    /// Returns the key of the hazard induced by `entity`, if registered.
    #[must_use]
    pub fn haz_key(&self, entity: &E) -> Option<HazKey> {
        self.hazards_map
            .iter()
            .find(|(_, hazard)| hazard.entity() == *entity)
            .map(|(key, _)| key)
    }

    /// All hazards registered in the CDE (active and inactive)
    #[must_use]
    pub fn hazards_map(&self) -> &SlotMap<HazKey, Hazard<E>> {
        &self.hazards_map
    }

    /// Returns the registered hazard, or `None` if the key is no longer valid.
    #[inline]
    #[must_use]
    pub fn hazard(&self, key: HazKey) -> Option<&Hazard<E>> {
        self.hazards_map.get(key)
    }

    /// Configuration of the CDE
    #[must_use]
    pub fn config(&self) -> CDEConfig {
        self.config
    }
}

/// Whether `inner` lies inside `outer` without touching its sides.
fn strictly_inside(inner: Rect, outer: Rect) -> bool {
    outer.x_min < inner.x_min
        && inner.x_max < outer.x_max
        && outer.y_min < inner.y_min
        && inner.y_max < outer.y_max
}

///Configuration of the [`CDEngine`]
#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub struct CDEConfig {
    ///Configuration of the surrogate generation for items
    pub item_surrogate_config: SPSurrogateConfig,
}

/// Snapshot of the dynamic hazards of a [`CDEngine`], those registered after its construction,
/// to restore them later.
#[derive(Clone, Debug)]
pub struct CDESnapshot<E = BasicHazardEntity> {
    pub(crate) dynamic_hazards: Vec<Hazard<E>>,
}

impl<E> CDESnapshot<E> {
    /// The saved hazards, those registered after the engine's construction.
    #[must_use]
    pub fn dynamic_hazards(&self) -> &[Hazard<E>] {
        &self.dynamic_hazards
    }
}
