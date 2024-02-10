use enum_dispatch::enum_dispatch;
use crate::entities::bin::Bin;
use crate::entities::item::Item;
use crate::geometry::geo_traits::Shape;
use crate::util::assertions;

/// Static representation of a problem instance.
#[derive(Debug, Clone)]
#[enum_dispatch]
pub enum Instance {
    SP(SPInstance),
    BP(BPInstance),
}

#[enum_dispatch(Instance)]
pub trait InstanceVariant {
    fn items(&self) -> &[(Item, usize)];
    fn item_qty(&self, id: usize) -> usize{
        self.items()[id].1
    }
    fn item(&self, id: usize) -> &Item {
        &self.items()[id].0
    }
    fn total_item_qty(&self) -> usize{
        self.items().iter().map(|(_, qty)| qty).sum()
    }

    fn item_area(&self) -> f64;
}


#[derive(Debug, Clone)]
pub struct BPInstance {
    /// Items to be packed in the instance, along with their requested quantities
    pub items: Vec<(Item, usize)>,
    /// Total area of all items in the instance
    pub item_area: f64,

    pub bins: Vec<(Bin, usize)>,
}

#[derive(Debug, Clone)]
pub struct SPInstance {
    pub items: Vec<(Item, usize)>,
    pub item_area: f64,
    pub strip_height: f64,
}

impl BPInstance {
    pub fn new(items: Vec<(Item, usize)>, bins: Vec<(Bin, usize)>) -> Self {
        assert!(assertions::instance_item_bin_ids_correct(&items, &bins));

        let item_area = items.iter().map(|(item, qty)| item.shape().area() * *qty as f64).sum();

        Self { items, item_area, bins }
    }
}

impl SPInstance {
    pub fn new(items: Vec<(Item, usize)>, strip_height: f64) -> Self {
        let item_area = items.iter().map(|(item, qty)| item.shape().area() * *qty as f64).sum();

        Self { items, item_area, strip_height }
    }
}

impl InstanceVariant for BPInstance {
    fn items(&self) -> &[(Item, usize)] {
        &self.items
    }

    fn item_area(&self) -> f64 {
        self.item_area
    }
}

impl InstanceVariant for SPInstance {
    fn items(&self) -> &[(Item, usize)] {
        &self.items
    }

    fn item_area(&self) -> f64 {
        self.item_area
    }
}