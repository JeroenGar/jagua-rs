use crate::entities::Item;
use crate::geometry::shape_modification::ShapeModifyConfig;
use crate::io::import::Importer;
use crate::probs::spp::entities::{SPInstance, Strip};
use crate::probs::spp::io::ext_repr::ExtSPInstance;
use anyhow::{Result, ensure};
use itertools::Itertools;
use rayon::prelude::*;

/// Imports an instance into the library
pub fn import(importer: &Importer, ext_instance: &ExtSPInstance) -> Result<SPInstance> {
    let items: Vec<(Item, usize)> = {
        let mut items = ext_instance
            .items
            .par_iter()
            .map(|ext_item| {
                let item = importer.import_item(&ext_item.base)?;
                let demand = ext_item.demand as usize;
                Ok((item, demand))
            })
            .collect::<Result<Vec<(Item, usize)>>>()?;

        items.sort_by_key(|(item, _)| item.id);
        ensure!(
            items.iter().enumerate().all(|(i, (item, _))| item.id == i),
            "All items should have consecutive IDs starting from 0. IDs: {:?}",
            items.iter().map(|(item, _)| item.id).sorted().collect_vec()
        );
        items
    };

    let total_item_area = items
        .iter()
        .map(|(item, demand)| item.area() * *demand as f32)
        .sum::<f32>();

    let fixed_height = ext_instance.strip_height;

    // Initialize the base width for 100% density
    let width = total_item_area / fixed_height;

    let base_strip = Strip {
        fixed_height,
        cde_config: importer.cde_config,
        shape_modify_config: ShapeModifyConfig {
            offset: importer.shape_modify_config.offset,
            simplify_tolerance: None,
        },
        width,
    };

    Ok(SPInstance::new(items, base_strip))
}
