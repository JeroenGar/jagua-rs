use crate::geometry::shape_modification::ShapeModifyConfig;
use crate::io::import::Importer;
use crate::probs::mspp::entities::{MSPInstance, MSPSolution, Strip};
use crate::probs::mspp::io::ext_repr::ExtMSPInstance;
use anyhow::{Result, ensure};
use itertools::Itertools;
use rayon::prelude::*;

/// Imports an instance into the library
pub fn import_instance(importer: &Importer, ext_instance: &ExtMSPInstance) -> Result<MSPInstance> {
    ensure!(
        ext_instance
            .items
            .iter()
            .map(|item| item.base.id)
            .all_unique(),
        "item IDs must be unique"
    );
    let items = ext_instance
        .items
        .iter()
        .filter(|item| item.demand > 0)
        .collect_vec()
        .into_par_iter()
        .enumerate()
        .map(|(idx, item)| {
            Ok((
                importer.import_item(&item.base, idx)?,
                usize::try_from(item.demand)?,
            ))
        })
        .collect::<Result<Vec<_>>>()?;
    ensure!(!items.is_empty(), "instance must have positive item demand");

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

    Ok(MSPInstance::new(items, base_strip))
}

/// Imports a solution into the library.
#[must_use]
pub fn import_solution(_instance: &MSPInstance, _ext_solution: &ExtMSPInstance) -> MSPSolution {
    unimplemented!("not yet implemented")
}
