use crate::collision_detection::hazards::{Hazard, HazardEntity};
use crate::collision_detection::{CDESnapshot, CDEngine};
use crate::entities::general::Item;
use crate::entities::general::{Bin, Instance};
use crate::entities::general::{PItemKey, PlacedItem};
use crate::geometry::DTransformation;
use crate::util::assertions;
use slotmap::SlotMap;

/// Defines a configuration of [`Item`]s in a [`Bin`].
///It is a mutable representation, and can be modified by placing or removing items.
///Each layout maintains a [`CDEngine`], which can be used to check for collisions before placing items.
#[derive(Clone)]
pub struct Layout {
    /// The bin used for this layout
    pub bin: Bin,
    /// How the items are placed in the bin
    pub placed_items: SlotMap<PItemKey, PlacedItem>,
    /// The collision detection engine for this layout
    cde: CDEngine,
}

impl Layout {
    pub fn new(bin: Bin) -> Self {
        let cde = bin.base_cde.as_ref().clone();
        Layout {
            bin,
            placed_items: SlotMap::with_key(),
            cde,
        }
    }

    pub fn from_snapshot(ls: &LayoutSnapshot) -> Self {
        let mut layout = Layout::new(ls.bin.clone());
        layout.restore(ls);
        layout
    }

    pub fn change_bin(&mut self, bin: Bin) {
        // swap the bin
        self.bin = bin;
        // rebuild the CDE
        self.cde = self.bin.base_cde.as_ref().clone();
        for (pk, pi) in self.placed_items.iter() {
            let hazard = Hazard::new((pk, pi).into(), pi.shape.clone());
            self.cde.register_hazard(hazard);
        }
    }

    pub fn save(&mut self) -> LayoutSnapshot {
        LayoutSnapshot {
            bin: self.bin.clone(),
            placed_items: self.placed_items.clone(),
            cde_snapshot: self.cde.create_snapshot(),
        }
    }

    pub fn restore(&mut self, layout_snapshot: &LayoutSnapshot) {
        assert_eq!(self.bin.id, layout_snapshot.bin.id);
        self.placed_items = layout_snapshot.placed_items.clone();
        self.cde.restore(&layout_snapshot.cde_snapshot);

        debug_assert!(assertions::layout_qt_matches_fresh_qt(self));
        debug_assert!(assertions::layouts_match(self, layout_snapshot))
    }

    pub fn place_item(&mut self, item: &Item, d_transformation: DTransformation) -> PItemKey {
        let pk = self
            .placed_items
            .insert(PlacedItem::new(item, d_transformation));
        let pi = &self.placed_items[pk];
        let hazard = Hazard::new((pk, pi).into(), pi.shape.clone());

        self.cde.register_hazard(hazard);

        debug_assert!(assertions::layout_qt_matches_fresh_qt(self));

        pk
    }

    pub fn remove_item(&mut self, pk: PItemKey, commit_instant: bool) -> PlacedItem {
        let pi = self
            .placed_items
            .remove(pk)
            .expect("key is not valid anymore");

        // update the collision detection engine
        self.cde.deregister_hazard((pk, &pi).into(), commit_instant);

        debug_assert!(assertions::layout_qt_matches_fresh_qt(self));

        pi
    }

    /// True if no items are placed
    pub fn is_empty(&self) -> bool {
        self.placed_items.is_empty()
    }

    pub fn placed_items(&self) -> &SlotMap<PItemKey, PlacedItem> {
        &self.placed_items
    }

    /// The current density of the layout defined as the ratio of the area of the items placed to the area of the bin.
    /// Uses the original shapes of items and bin to calculate the area.
    pub fn density(&self, instance: &impl Instance) -> f32 {
        self.placed_item_area(instance) / self.bin.area()
    }

    /// The sum of the areas of the items placed in the layout (using the original shapes of the items).
    pub fn placed_item_area(&self, instance: &impl Instance) -> f32 {
        self.placed_items
            .iter()
            .map(|(_, pi)| instance.item(pi.item_id))
            .map(|item| item.area())
            .sum::<f32>()
    }

    /// Returns the collision detection engine for this layout
    pub fn cde(&self) -> &CDEngine {
        &self.cde
    }

    /// Returns true if all the items are placed without colliding
    pub fn is_feasible(&self) -> bool {
        self.placed_items.iter().all(|(pk, pi)| {
            let filter = HazardEntity::from((pk, pi));
            !self.cde.poly_collides(&pi.shape, &filter)
        })
    }
}

/// Immutable and compact representation of a [`Layout`].
/// Can be used to restore a [`Layout`] back to a previous state.
#[derive(Clone, Debug)]
pub struct LayoutSnapshot {
    pub bin: Bin,
    pub placed_items: SlotMap<PItemKey, PlacedItem>,
    /// Snapshot of the collision detection engine
    pub cde_snapshot: CDESnapshot,
}

impl LayoutSnapshot {
    /// Equivalent to [`Layout::density`]
    pub fn density(&self, instance: &impl Instance) -> f32 {
        self.placed_item_area(instance) / self.bin.area()
    }

    /// Equivalent to [`Layout::placed_item_area`]
    pub fn placed_item_area(&self, instance: &impl Instance) -> f32 {
        self.placed_items
            .iter()
            .map(|(_, pi)| instance.item(pi.item_id))
            .map(|item| item.area())
            .sum::<f32>()
    }
}
