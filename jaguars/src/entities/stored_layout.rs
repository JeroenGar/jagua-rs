use crate::collision_detection::cde_snapshot::CDESnapshot;
use crate::entities::bin::Bin;
use crate::entities::placed_item::PlacedItem;

#[derive(Clone, Debug)]
pub struct StoredLayout {
    id: usize,
    bin: Bin,
    placed_items: Vec<PlacedItem>,
    cde_snapshot: CDESnapshot,
    usage: f64,
}

impl StoredLayout {
    pub fn new(id: usize, bin: Bin, placed_items: Vec<PlacedItem>, cde_snapshot: CDESnapshot, usage: f64) -> Self {
        Self { id, bin, placed_items, cde_snapshot, usage }
    }

    pub fn placed_items(&self) -> &Vec<PlacedItem> {
        &self.placed_items
    }

    pub fn bin(&self) -> &Bin {
        &self.bin
    }

    pub fn usage(&self) -> f64 {
        self.usage
    }

    pub fn id(&self) -> usize {
        self.id
    }

    pub fn cde_snapshot(&self) -> &CDESnapshot {
        &self.cde_snapshot
    }
}