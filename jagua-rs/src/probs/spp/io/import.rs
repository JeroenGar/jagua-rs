use crate::geometry::DTransformation;
use crate::geometry::shape_modification::ShapeModifyConfig;
use crate::io::import::{Importer, ext_to_int_transformation, import_demand_items};
use crate::probs::spp::entities::{SPInstance, SPPlacement, SPProblem, SPSolution, Strip};
use crate::probs::spp::io::ext_repr::{ExtSPInstance, ExtSPSolution};
use anyhow::{Result, anyhow};

/// Imports an instance into the library
#[allow(clippy::cast_precision_loss)]
pub fn import_instance(importer: &Importer, ext_instance: &ExtSPInstance) -> Result<SPInstance> {
    let (items, external_ids) = import_demand_items(
        importer,
        ext_instance
            .items
            .iter()
            .map(|item| (&item.base, item.demand)),
    )?;

    let total_item_area = items
        .iter()
        .map(|(item, demand)| item.area() * *demand as f32)
        .sum::<f32>();

    let fixed_height = ext_instance.strip_height;

    // Initialize the base width for 100% density
    let width = total_item_area / fixed_height;

    let base_strip = Strip::new(
        fixed_height,
        importer.cde_config,
        ShapeModifyConfig {
            offset: importer.shape_modify_config.offset,
            simplify_tolerance: None,
            narrow_concavity_cutoff: None,
        },
        width,
    )?;

    Ok(SPInstance::new(items, base_strip, external_ids))
}

/// Imports a solution into the library.
pub fn import_solution(instance: &SPInstance, ext_solution: &ExtSPSolution) -> Result<SPSolution> {
    let mut prob = SPProblem::new(instance.clone());
    prob.change_strip_width(ext_solution.strip_width);

    for ext_placement in ext_solution.layout.placed_items.iter().cloned() {
        let item_id = instance
            .internal_item_id(ext_placement.item_id)
            .ok_or_else(|| anyhow!("unknown item ID {}", ext_placement.item_id))?;
        let d_transf = {
            let ext_transf = DTransformation::from(ext_placement.transformation);
            let item = &instance.item(item_id);
            ext_to_int_transformation(&ext_transf, &item.shape_orig.pre_transform)
        };
        prob.place_item(SPPlacement { item_id, d_transf });
    }

    Ok(prob.save())
}
