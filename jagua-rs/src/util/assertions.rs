use crate::collision_detection::CDEngine;
use crate::collision_detection::hazards::Hazard;
use crate::collision_detection::hazards::{BasicHazardEntity, HazardEntity};
use crate::entities::Layout;
use crate::entities::LayoutSnapshot;
use crate::geometry::fail_fast::SPSurrogate;
use crate::geometry::geo_enums::GeoPosition;
use itertools::Itertools;
use std::collections::HashSet;
//Various checks to verify correctness of the state of the system
//Used in debug_assertion!() blocks

#[must_use]
pub fn snapshot_matches_layout(layout: &Layout, layout_snapshot: &LayoutSnapshot) -> bool {
    if !std::sync::Arc::ptr_eq(
        &layout.container.base_cde,
        &layout_snapshot.container.base_cde,
    ) {
        return false;
    }
    for placed_item in layout_snapshot.placed_items.values() {
        if !layout
            .placed_items
            .values()
            .any(|pi| pi.item.idx == placed_item.item.idx && pi.d_transf == placed_item.d_transf)
        {
            return false;
        }
    }
    true
}

/// Checks that the engine's index holds exactly the bounding boxes of all hazards except the
/// exterior, sorted by `x_min`, and that its chunk bounds are the largest `x_max` of each chunk.
#[must_use]
pub fn index_matches_hazards<E: HazardEntity>(cde: &CDEngine<E>) -> bool {
    let indexed = cde.index.iter().collect_vec();
    let sorted = indexed.is_sorted_by(|(_, a), (_, b)| a.x_min <= b.x_min);
    let chunk_bounds = indexed
        .chunks(64)
        .map(|chunk| {
            chunk
                .iter()
                .map(|(_, bbox)| bbox.x_max)
                .fold(f32::NEG_INFINITY, f32::max)
        })
        .eq(cde.index.chunk_x_max().iter().copied());
    let interior = cde
        .hazards_map
        .iter()
        .filter(|(_, h)| h.entity().scope() == GeoPosition::Interior)
        .map(|(k, h)| (k, h.shape().bbox))
        .sorted_by_key(|(k, _)| *k)
        .collect_vec();
    let by_key = indexed.into_iter().sorted_by_key(|(k, _)| *k).collect_vec();
    sorted && chunk_bounds && by_key == interior
}

/// Checks that the layout's engine holds the container's hazards and those of its placed items.
#[must_use]
pub fn layout_cde_matches_fresh(layout: &Layout) -> bool {
    let hazards = layout
        .placed_items
        .iter()
        .map(|(pk, pi)| Hazard::new(BasicHazardEntity::from((pk, pi)), pi.shape.clone()));
    cde_matches_fresh(layout.cde(), &layout.container.base_cde, hazards)
}

/// Checks that `cde` matches a fresh copy of `base` with `dynamic_hazards` registered:
/// the same hazards, and an index that matches them.
#[must_use]
pub fn cde_matches_fresh<E: HazardEntity>(
    cde: &CDEngine<E>,
    base: &CDEngine<E>,
    dynamic_hazards: impl IntoIterator<Item = Hazard<E>>,
) -> bool {
    let mut fresh_cde = base.clone();
    for hazard in dynamic_hazards {
        fresh_cde.register_hazard(hazard);
    }
    index_matches_hazards(cde) && hazards_match(cde.hazards(), fresh_cde.hazards())
}

fn hazards_match<'a, E: HazardEntity + 'a>(
    chv1: impl Iterator<Item = &'a Hazard<E>>,
    chv2: impl Iterator<Item = &'a Hazard<E>>,
) -> bool {
    let chv1_active_hazards = chv1.map(Hazard::entity).collect::<HashSet<_>>();

    let chv2_active_hazards = chv2.map(Hazard::entity).collect::<HashSet<_>>();

    if chv1_active_hazards != chv2_active_hazards {
        println!("Hazard vecs don't match");
        return false;
    }
    true
}

///Prints code to rebuild a layout. Intended for debugging purposes.
pub fn print_layout(layout: &Layout) {
    println!("let mut layout = Layout::new(container.clone());");
    println!();

    for pi in layout.placed_items.values() {
        let transformation_str = {
            let t_decomp = &pi.d_transf;
            let (tr, (tx, ty)) = (t_decomp.rotation(), t_decomp.translation());
            format!(
                "&DTransformation::new({tr:.6},({tx:.6},{ty:.6})).with_reflection({})",
                t_decomp.reflected
            )
        };

        println!(
            "layout.place_item(instance.item({}), {});",
            pi.item.idx, transformation_str
        );
    }
}

/// Checks a transformed buffer's hull indices against those recomputed from its reference.
#[must_use]
pub fn hull_indices_match_reference(
    surrogate: &SPSurrogate,
    reference: &SPSurrogate,
    reflected: bool,
    n_vertices: usize,
) -> bool {
    let ref_indices = &reference.convex_hull_indices;
    let expected: Vec<usize> = if reflected {
        ref_indices
            .iter()
            .rev()
            .map(|&i| n_vertices - 1 - i)
            .collect()
    } else {
        ref_indices.clone()
    };
    surrogate.convex_hull_indices == expected
}
