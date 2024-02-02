use crate::collision_detection::cd_engine::{CDEngine, CDESnapshot};
use crate::entities::bin::Bin;
use crate::entities::instance::Instance;
use crate::entities::item::Item;
use crate::entities::placed_item::PlacedItem;
use crate::entities::placed_item::PlacedItemUID;
use crate::geometry::d_transformation::DTransformation;
use crate::geometry::geo_traits::Shape;
use crate::util::assertions;

#[derive(Clone)]
pub struct Layout {
    id: usize,
    bin: Bin,
    placed_items: Vec<PlacedItem>,
    cde: CDEngine
}

impl Layout {
    pub fn new(id: usize, bin: Bin) -> Self {
        let cde = bin.base_cde().clone();
        Layout {
            id,
            bin,
            placed_items: vec![],
            cde
        }
    }

    pub fn new_from_stored(id: usize, layout_snapshot: &LayoutSnapshot, instance: &Instance) -> Self {
        let mut layout = Layout::new(id, layout_snapshot.bin.clone());
        layout.restore(&layout_snapshot, instance);
        layout
    }

    pub fn create_layout_snapshot(&mut self) -> LayoutSnapshot {
        debug_assert!(assertions::layout_is_collision_free(self));

        LayoutSnapshot{
            id: self.id,
            bin: self.bin.clone(),
            placed_items: self.placed_items.clone(),
            cde_snapshot: self.cde.create_snapshot(),
            usage: self.usage()
        }
    }

    pub fn restore(&mut self, layout_snapshot: &LayoutSnapshot, _instance: &Instance) {
        assert_eq!(self.bin.id(), layout_snapshot.bin.id());

        self.placed_items = layout_snapshot.placed_items.clone();
        self.cde.restore(&layout_snapshot.cde_snapshot);

        debug_assert!(assertions::layout_qt_matches_fresh_qt(self));
        debug_assert!(assertions::layouts_match(self, layout_snapshot))
    }

    pub fn clone_with_id(&self, id: usize) -> Self {
        Layout {
            id,
            ..self.clone()
        }
    }

    pub fn place_item(&mut self, item: &Item, d_transformation: &DTransformation) {

        let placed_item = PlacedItem::new(item, d_transformation.clone());
        self.cde.register_hazard((&placed_item).into());
        self.placed_items.push(placed_item);

        debug_assert!(assertions::layout_qt_matches_fresh_qt(self));
    }

    pub fn remove_item(&mut self, pi_uid: &PlacedItemUID, commit_instantly: bool) {

        let pos = self.placed_items.iter().position(|pi| pi.uid() == pi_uid).expect("item not found");
        let placed_item = self.placed_items.swap_remove(pos);
        self.cde.deregister_hazard(&placed_item.uid().clone().into(), commit_instantly);

        debug_assert!(assertions::layout_qt_matches_fresh_qt(self));
    }

    pub fn is_empty(&self) -> bool {
        self.placed_items.is_empty()
    }

    pub fn bin(&self) -> &Bin {
        &self.bin
    }

    pub fn placed_items(&self) -> &Vec<PlacedItem> {
        &self.placed_items
    }

    pub fn usage(&self) -> f64 {
        let bin_area = self.bin().area();
        let item_area = self.placed_items.iter().map(|p_i| p_i.shape().area()).sum::<f64>();

        item_area / bin_area
    }

    pub fn id(&self) -> usize {
        self.id
    }


    pub fn cde(&self) -> &CDEngine {
        &self.cde
    }

    pub fn flush_changes(&mut self) {
        self.cde.flush_changes();
    }
}

#[derive(Clone, Debug)]
pub struct LayoutSnapshot {
    id: usize,
    bin: Bin,
    placed_items: Vec<PlacedItem>,
    cde_snapshot: CDESnapshot,
    usage: f64,
}

impl LayoutSnapshot {
    pub fn id(&self) -> usize {
        self.id
    }

    pub fn bin(&self) -> &Bin {
        &self.bin
    }

    pub fn placed_items(&self) -> &Vec<PlacedItem> {
        &self.placed_items
    }

    pub fn usage(&self) -> f64 {
        self.usage
    }
}