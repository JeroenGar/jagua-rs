use crate::collision_detection::cd_engine::{CDESnapshot, CDEngine};
use crate::collision_detection::hazard::Hazard;
use crate::entities::bin::Bin;
use crate::entities::item::Item;
use crate::entities::placed_item::{PItemKey, PlacedItem};
use crate::fsize;
use crate::geometry::d_transformation::DTransformation;
use crate::geometry::geo_traits::Shape;
use crate::util::assertions;
use slotmap::SlotMap;

///A Layout is made out of a [Bin] with a set of [Item]s positioned inside of it in a specific way.
///It is a mutable representation, and can be modified by placing or removing items.
///
///The layout is responsible for maintaining its [CDEngine],
///ensuring that it always reflects the current state of the layout.
#[derive(Clone)]
pub struct Layout {
    /// The unique identifier of the layout, used only to match with a [LayoutSnapshot].
    id: usize,
    /// The bin used for this layout
    bin: Bin,
    /// How the items are placed in the bin
    placed_items: SlotMap<PItemKey, PlacedItem>,
    /// The collision detection engine for this layout
    cde: CDEngine,
}

impl Layout {
    pub fn new(id: usize, bin: Bin) -> Self {
        let cde = bin.base_cde.as_ref().clone();
        Layout {
            id,
            bin,
            placed_items: SlotMap::with_key(),
            cde,
        }
    }

    pub fn from_snapshot(ls: &LayoutSnapshot) -> Self {
        let mut layout = Layout::new(ls.id, ls.bin.clone());
        layout.restore(&ls);
        layout
    }

    pub fn create_snapshot(&mut self) -> LayoutSnapshot {
        debug_assert!(assertions::layout_is_collision_free(self));

        LayoutSnapshot {
            id: self.id,
            bin: self.bin.clone(),
            placed_items: self.placed_items.clone(),
            cde_snapshot: self.cde.create_snapshot(),
            usage: self.usage(),
        }
    }

    pub fn restore(&mut self, layout_snapshot: &LayoutSnapshot) {
        assert_eq!(self.id, layout_snapshot.id);

        self.placed_items = layout_snapshot.placed_items.clone();
        self.cde.restore(&layout_snapshot.cde_snapshot);

        debug_assert!(assertions::layout_qt_matches_fresh_qt(self));
        debug_assert!(assertions::layouts_match(self, layout_snapshot))
    }

    pub fn clone_with_id(&self, id: usize) -> Self {
        Layout { id, ..self.clone() }
    }

    pub fn place_item(&mut self, item: &Item, d_transformation: DTransformation) -> PItemKey {
        let placed_item = PlacedItem::new(item, d_transformation);
        let pi_key = self.placed_items.insert(placed_item);

        let hazard = Hazard::new(pi_key.into(), self.placed_items[pi_key].shape.clone());
        self.cde.register_hazard(hazard);

        debug_assert!(assertions::layout_qt_matches_fresh_qt(self));

        pi_key
    }

    pub fn remove_item(&mut self, key: PItemKey, commit_instant: bool) -> PlacedItem {
        let p_item = self
            .placed_items
            .remove(key)
            .expect("key is not valid anymore");

        // update the collision detection engine
        self.cde.deregister_hazard(key.into(), commit_instant);

        debug_assert!(assertions::layout_qt_matches_fresh_qt(self));

        p_item
    }

    /// True if no items are placed
    pub fn is_empty(&self) -> bool {
        self.placed_items.is_empty()
    }

    pub fn bin(&self) -> &Bin {
        &self.bin
    }

    pub fn placed_items(&self) -> &SlotMap<PItemKey, PlacedItem> {
        &self.placed_items
    }

    /// Returns the usage of the bin with the items placed.
    /// It is the ratio of the area of the items placed to the area of the bin.
    pub fn usage(&self) -> fsize {
        let bin_area = self.bin().area;
        let item_area = self
            .placed_items
            .iter()
            .map(|(_, pi)| pi.shape.area())
            .sum::<fsize>();

        item_area / bin_area
    }

    pub fn id(&self) -> usize {
        self.id
    }

    /// Returns the collision detection engine for this layout
    pub fn cde(&self) -> &CDEngine {
        &self.cde
    }

    /// Makes sure that the collision detection engine is completely updated with the changes made to the layout.
    pub fn flush_changes(&mut self) {
        self.cde.flush_haz_prox_grid();
    }
}

/// Immutable and compact representation of a [Layout].
/// `Layout`s can create `LayoutSnapshot`s, and revert back themselves to a previous state using them.
#[derive(Clone, Debug)]
pub struct LayoutSnapshot {
    /// The unique identifier of the layout, used only to match with a [Layout].
    pub id: usize,
    /// The bin used for this layout
    pub bin: Bin,
    /// How the items are placed in the bin
    pub placed_items: SlotMap<PItemKey, PlacedItem>,
    /// The collision detection engine snapshot for this layout
    pub cde_snapshot: CDESnapshot,
    /// The usage of the bin with the items placed
    pub usage: fsize,
}
