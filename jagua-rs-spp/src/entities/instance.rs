use jagua_rs_base::entities::Item;
use jagua_rs_base::geometry::shape_modification::ShapeModifyConfig;
use crate::entities::general::Bin;
use crate::entities::general::Instance;
use crate::entities::general::Item;
use crate::geometry::geo_traits::Shape;
use crate::geometry::shape_modification::ShapeModifyConfig;
use crate::util::assertions;

#[derive(Debug, Clone)]
/// Instance of the Strip Packing Problem: a set of items to be packed into a single strip with a fixed height and variable width.
pub struct SPInstance {
    /// The items to be packed and their quantities
    pub items: Vec<(Item, usize)>,
    /// The total area of the items
    pub item_area: f32,
    /// The (fixed) height of the strip
    pub strip_height: f32,
    /// The config used to modify the shape of the strip
    pub strip_modify_config: ShapeModifyConfig,
}

impl SPInstance {
    pub fn new(
        items: Vec<(Item, usize)>,
        strip_height: f32,
        strip_modify_config: ShapeModifyConfig,
    ) -> Self {
        assert!(assertions::instance_item_bin_ids_correct(&items, &[]));

        let item_area = items
            .iter()
            .map(|(item, qty)| item.shape_orig.area() * *qty as f32)
            .sum();

        Self {
            items,
            item_area,
            strip_height,
            strip_modify_config,
        }
    }
}

impl Instance for SPInstance {
    fn items(&self) -> &[(Item, usize)] {
        &self.items
    }

    fn bins(&self) -> &[(Bin, usize)] {
        &[]
    }
}
