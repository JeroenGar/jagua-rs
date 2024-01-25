use crate::collision_detection::cd_engine::CDEngine;
use crate::entities::bin::Bin;
use crate::entities::instance::Instance;
use crate::entities::item::Item;
use crate::entities::placed_item::PlacedItem;
use crate::entities::placed_item_uid::PlacedItemUID;
use crate::entities::stored_layout::StoredLayout;
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

    pub fn new_from_stored(id: usize, stored_layout: &StoredLayout, instance: &Instance) -> Self {
        let mut layout = Layout::new(id, stored_layout.bin().clone());
        layout.restore(&stored_layout, instance);
        layout
    }

    pub fn create_stored_layout(&mut self) -> StoredLayout {
        let placed_items = self.placed_items.clone();
        let cde_snapshot = self.cde.create_snapshot();
        let usage = self.usage();

        debug_assert!(assertions::layout_is_collision_free(self));

        StoredLayout::new(self.id, self.bin.clone(), placed_items, cde_snapshot, usage)
    }

    pub fn restore(&mut self, stored_layout: &StoredLayout, instance: &Instance) {
        assert_eq!(self.bin.id(), stored_layout.bin().id());

        self.placed_items = stored_layout.placed_items().clone();
        self.cde.restore(&stored_layout.cde_snapshot());

        debug_assert!(assertions::layout_qt_matches_fresh_qt(self));
        debug_assert!(assertions::layouts_match(self, stored_layout))
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
