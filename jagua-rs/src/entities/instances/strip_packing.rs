use crate::entities::instances::instance_generic::InstanceGeneric;
use crate::entities::item::Item;
use crate::geometry::geo_traits::Shape;

/// Strip-packing problem instance: a set of items to be packed into a single strip.
/// The items are to be packed in such a way that the total width of the strip used is minimized.
#[derive(Debug, Clone)]
pub struct SPInstance {
    pub items: Vec<(Item, usize)>,
    pub item_area: f64,
    pub strip_height: f64,
}

impl SPInstance {
    pub fn new(items: Vec<(Item, usize)>, strip_height: f64) -> Self {
        let item_area = items.iter().map(|(item, qty)| item.shape.area() * *qty as f64).sum();

        Self { items, item_area, strip_height }
    }
}

impl InstanceGeneric for SPInstance {
    fn items(&self) -> &[(Item, usize)] {
        &self.items
    }

    fn item_area(&self) -> f64 {
        self.item_area
    }
}