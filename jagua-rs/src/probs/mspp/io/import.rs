use crate::geometry::shape_modification::ShapeModifyConfig;
use crate::io::import::{Importer, import_demand_items};
use crate::probs::mspp::entities::{MSPInstance, MSPSolution, Strip};
use crate::probs::mspp::io::ext_repr::ExtMSPInstance;
use anyhow::Result;

/// Imports an instance into the library
pub fn import_instance(importer: &Importer, ext_instance: &ExtMSPInstance) -> Result<MSPInstance> {
    let (items, external_ids) = import_demand_items(
        importer,
        ext_instance
            .items
            .iter()
            .map(|item| (&item.base, item.demand)),
    )?;

    let ext_strip = &ext_instance.strips;

    let base_strip = Strip::new(
        ext_strip.max_width,
        ext_strip.height,
        importer.cde_config,
        ShapeModifyConfig {
            offset: importer.shape_modify_config.offset,
            simplify_tolerance: None,
            narrow_concavity_cutoff: None,
        },
        ext_strip.max_width,
    )?;

    Ok(MSPInstance::new(items, base_strip, external_ids))
}

/// Imports a solution into the library.
#[must_use]
pub fn import_solution(_instance: &MSPInstance, _ext_solution: &ExtMSPInstance) -> MSPSolution {
    unimplemented!("not yet implemented")
}
