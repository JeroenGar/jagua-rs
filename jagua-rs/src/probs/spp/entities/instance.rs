use crate::entities::{Container, Instance, Item};
use crate::probs::spp::entities::Strip;
use crate::probs::spp::util::assertions;
use std::iter;

#[derive(Debug, Clone)]
/// Instance of the Strip Packing Problem: a set of items to be packed into a single strip with a fixed height and variable width.
pub struct SPInstance {
    /// The items to be packed and their demands
    pub items: Vec<(Item, usize)>,
    /// The height of the strip (fixed)
    pub base_strip: Strip,
}

impl SPInstance {
    pub fn new(items: Vec<(Item, usize)>, base_strip: Strip) -> Self {
        assert!(
            assertions::instance_item_ids_correct(&items),
            "All items should have consecutive IDs starting from 0"
        );

        Self { items, base_strip }
    }

    pub fn item_area(&self) -> f32 {
        self.items
            .iter()
            .map(|(item, qty)| item.shape_orig.area() * *qty as f32)
            .sum()
    }

    pub fn item_qty(&self, id: usize) -> usize {
        self.items[id].1
    }

    pub fn total_item_qty(&self) -> usize {
        self.items.iter().map(|(_, qty)| *qty).sum()
    }
}

impl Instance for SPInstance {
    fn items(&self) -> impl Iterator<Item = &Item> {
        self.items.iter().map(|(item, _qty)| item)
    }

    fn containers(&self) -> impl Iterator<Item = &Container> {
        iter::empty()
    }

    fn item(&self, id: usize) -> &Item {
        &self.items.get(id).unwrap().0
    }

    fn container(&self, _id: usize) -> &Container {
        panic!("no predefined containers for strip packing instances")
    }
}
