use crate::entities::bin_packing::{BPInstance, BPSolution};
use crate::entities::general::{Instance, PlacedItem};
use crate::entities::strip_packing::{SPInstance, SPSolution};
use crate::geometry::{DTransformation, Transformation};
use crate::io::json_solution::{
    JsonContainer, JsonLayout, JsonLayoutStats, JsonPlacedItem, JsonSolution,
};
use itertools::Itertools;
use std::time::Instant;

/// Exports [`SPSolution`] by composing a [`JsonSolution`] from it.
pub fn export_spp_solution(
    solution: &SPSolution,
    instance: &SPInstance,
    epoch: Instant,
) -> JsonSolution {
    let json_layout = JsonLayout {
        container: JsonContainer::Strip {
            width: solution.strip_width,
            height: instance.strip_height,
        },
        placed_items: export_json_placed_items(
            solution.layout_snapshot.placed_items.values(),
            instance,
        ),
        statistics: JsonLayoutStats {
            density: solution.density(instance),
        },
    };

    JsonSolution {
        layouts: vec![json_layout],
        density: solution.density(instance),
        run_time_sec: solution.time_stamp.duration_since(epoch).as_secs(),
    }
}

/// Exports [`BPSolution`] by composing a [`JsonSolution`] from it.
pub fn export_bpp_solution(
    solution: &BPSolution,
    instance: &BPInstance,
    epoch: Instant,
) -> JsonSolution {
    let layouts = solution
        .layout_snapshots
        .iter()
        .map(|(_, sl)| JsonLayout {
            container: JsonContainer::Bin { index: sl.bin.id },
            placed_items: export_json_placed_items(sl.placed_items.values(), instance),
            statistics: JsonLayoutStats {
                density: sl.density(instance),
            },
        })
        .collect_vec();

    JsonSolution {
        layouts,
        density: solution.density(instance),
        run_time_sec: solution.time_stamp.duration_since(epoch).as_secs(),
    }
}

/// Exports a set of placed items to a vector of [`JsonPlacedItem`].
pub fn export_json_placed_items<'a>(
    placed_items: impl Iterator<Item = &'a PlacedItem>,
    instance: &impl Instance,
) -> Vec<JsonPlacedItem> {
    placed_items
        .map(|pi| {
            let item_index = pi.item_id;
            let item = instance.item(item_index);

            let abs_transf =
                int_to_ext_transformation(&pi.d_transf, &item.shape_orig.pre_transform);

            JsonPlacedItem {
                index: item_index,
                transformation: abs_transf.into(),
            }
        })
        .collect()
}

/// Converts an internal (used within `jagua-rs`) transformation to an external transformation (applicable to the original shapes).
///
/// * `int_transf` - The internal transformation.
/// * `pre_transf` - The transformation that was applied to the original shape to derive the internal representation.
pub fn int_to_ext_transformation(
    int_transf: &DTransformation,
    pre_transf: &DTransformation,
) -> DTransformation {
    //1. apply the pre-transform
    //2. apply the internal transformation

    Transformation::empty()
        .transform_from_decomposed(pre_transf)
        .transform_from_decomposed(int_transf)
        .decompose()
}

/// Converts an external transformation (applicable to the original shapes) to an internal transformation (used within `jagua-rs`).
///
/// * `ext_transf` - The external transformation.
/// * `pre_transf` - The transformation that was applied to the original shape to derive the internal representation.
pub fn ext_to_int_transformation(
    ext_transf: &DTransformation,
    pre_transf: &DTransformation,
) -> DTransformation {
    //1. undo pre-transform
    //2. do the absolute transformation

    Transformation::empty()
        .transform(&pre_transf.compose().inverse())
        .transform_from_decomposed(ext_transf)
        .decompose()
}
