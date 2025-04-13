use crate::collision_detection::CDEngine;
use crate::collision_detection::hazards::Hazard;
use crate::collision_detection::hazards::HazardEntity;
use crate::collision_detection::hazards::filter;
use crate::collision_detection::hazards::filter::CombinedHazardFilter;
use crate::collision_detection::hazards::filter::EntityHazardFilter;
use crate::collision_detection::hpg::hazard_proximity_grid::HazardProximityGrid;
use crate::collision_detection::hpg::hpg_cell::HPGCellUpdate;
use crate::collision_detection::quadtree::QTHazPresence;
use crate::collision_detection::quadtree::QTHazard;
use crate::collision_detection::quadtree::QTNode;
use crate::entities::bin_packing::BPProblem;
use crate::entities::bin_packing::BPSolution;
use crate::entities::general::Bin;
use crate::entities::general::Item;
use crate::entities::general::Layout;
use crate::entities::general::LayoutSnapshot;
use crate::entities::strip_packing::SPProblem;
use crate::entities::strip_packing::SPSolution;
use crate::geometry::Transformation;
use crate::geometry::geo_traits::{Shape, Transformable};
use crate::geometry::primitives::AARectangle;
use crate::fsize;
use float_cmp::approx_eq;
use itertools::Itertools;
use log::error;
use std::collections::HashSet;
//Various checks to verify correctness of the state of the system
//Used in debug_assertion!() blocks

pub fn instance_item_bin_ids_correct(items: &[(Item, usize)], bins: &[(Bin, usize)]) -> bool {
    items
        .iter()
        .enumerate()
        .all(|(i, (item, _qty))| item.id == i)
        && bins.iter().enumerate().all(|(i, (bin, _qty))| bin.id == i)
}

pub fn spproblem_matches_solution(spp: &SPProblem, sol: &SPSolution) -> bool {
    let SPSolution {
        strip_width,
        layout_snapshot,
        usage,
        time_stamp: _,
    } = sol;

    assert_eq!(*strip_width, spp.strip_width());
    assert_eq!(*usage, spp.usage());
    assert!(layouts_match(&spp.layout, layout_snapshot));

    true
}

pub fn bpproblem_matches_solution(bpp: &BPProblem, sol: &BPSolution) -> bool {
    let BPSolution {
        layout_snapshots,
        usage,
        placed_item_qtys,
        target_item_qtys: _,
        bin_qtys,
        time_stamp: _,
    } = sol;

    assert_eq!(*usage, bpp.usage());
    assert_eq!(*placed_item_qtys, bpp.placed_item_qtys().collect_vec());
    assert_eq!(bin_qtys, &bpp.bin_qtys);

    for (lkey, l) in &bpp.layouts {
        let ls = &layout_snapshots[lkey];
        if !layouts_match(l, ls) {
            return false;
        }
    }

    true
}

pub fn layouts_match(layout: &Layout, layout_snapshot: &LayoutSnapshot) -> bool {
    if layout.bin.id != layout_snapshot.bin.id {
        return false;
    }
    for placed_item in layout_snapshot.placed_items.values() {
        if !layout
            .placed_items()
            .values()
            .any(|pi| pi.item_id == placed_item.item_id && pi.d_transf == placed_item.d_transf)
        {
            return false;
        }
    }
    //TODO: add dotgrid check, check if quadtree does not contain any more uncommitted removals
    true
}

pub fn collision_hazards_sorted_correctly(hazards: &[QTHazard]) -> bool {
    let mut partial_hazard_detected = false;
    for hazard in hazards.iter() {
        match hazard.presence {
            QTHazPresence::Partial(_) => {
                partial_hazard_detected = true;
            }
            QTHazPresence::Entire => {
                if partial_hazard_detected {
                    return false;
                }
            }
            QTHazPresence::None => {
                panic!("None hazard in collision hazard vec");
            }
        };
    }
    true
}

pub fn all_bins_and_items_centered(items: &[(Item, usize)], bins: &[(Bin, usize)]) -> bool {
    items
        .iter()
        .map(|(i, _)| i.shape.centroid())
        .chain(bins.iter().map(|(b, _)| b.outer.centroid()))
        .all(|c| approx_eq!(fsize, c.0, 0.0) && approx_eq!(fsize, c.1, 0.0))
}

pub fn item_to_place_does_not_collide(
    item: &Item,
    transformation: &Transformation,
    layout: &Layout,
) -> bool {
    let haz_filter = &item.hazard_filter;

    let shape = item.shape.as_ref();
    let t_shape = shape.transform_clone(transformation);

    let entities_to_ignore = haz_filter.as_ref().map_or(vec![], |f| {
        filter::generate_irrelevant_hazards(f, layout.cde().all_hazards())
    });

    if layout
        .cde()
        .surrogate_collides(shape.surrogate(), transformation, &entities_to_ignore)
        || layout.cde().poly_collides(&t_shape, &entities_to_ignore)
    {
        return false;
    }
    true
}

pub fn layout_is_collision_free(layout: &Layout) -> bool {
    for (pk, pi) in layout.placed_items().iter() {
        let ehf = EntityHazardFilter(vec![(pk, pi).into()]);

        let combo_filter = match &pi.hazard_filter {
            None => CombinedHazardFilter {
                filters: vec![Box::new(&ehf)],
            },
            Some(hf) => CombinedHazardFilter {
                filters: vec![Box::new(&ehf), Box::new(hf)],
            },
        };
        let entities_to_ignore =
            filter::generate_irrelevant_hazards(&combo_filter, layout.cde().all_hazards());

        if layout.cde().poly_collides(&pi.shape, &entities_to_ignore) {
            println!("Collision detected for item {:.?}", pi.item_id);
            print_layout(layout);
            return false;
        }
    }
    true
}

pub fn qt_node_contains_no_deactivated_hazards<'a>(
    node: &'a QTNode,
    mut stacktrace: Vec<&'a QTNode>,
) -> (bool, Vec<&'a QTNode>) {
    stacktrace.push(node);
    let deactivated_hazard = node.hazards.all_hazards().iter().find(|h| !h.active);
    if deactivated_hazard.is_some() {
        println!("Deactivated hazard found");
        dbg!(&stacktrace);
        return (false, stacktrace);
    }

    if let Some(children) = &node.children {
        for child in children.as_ref() {
            let result = qt_node_contains_no_deactivated_hazards(child, stacktrace);
            stacktrace = result.1;
            let contains_no_deactivated_hazards = result.0;
            if !contains_no_deactivated_hazards {
                return (false, stacktrace);
            }
        }
    }

    stacktrace.pop();
    (true, stacktrace)
}

pub fn qt_contains_no_dangling_hazards(cde: &CDEngine) -> bool {
    if let Some(children) = &cde.quadtree().children {
        for child in children.as_ref() {
            if !qt_node_contains_no_dangling_hazards(child, cde.quadtree()) {
                return false;
            }
        }
    }
    true
}

fn qt_node_contains_no_dangling_hazards(node: &QTNode, parent: &QTNode) -> bool {
    let parent_h_entities = parent
        .hazards
        .all_hazards()
        .iter()
        .map(|h| &h.entity)
        .unique()
        .collect_vec();

    let dangling_hazards = node
        .hazards
        .all_hazards()
        .iter()
        .any(|h| !parent_h_entities.contains(&&h.entity));
    if dangling_hazards {
        println!("Node contains dangling hazard");
        return false;
    }

    if let Some(children) = &node.children {
        for child in children.as_ref() {
            if !qt_node_contains_no_dangling_hazards(child, node) {
                return false;
            }
        }
    }

    true
}

pub fn qt_hz_entity_activation_consistent(cde: &CDEngine) -> bool {
    for (active, hz_entity) in cde
        .quadtree()
        .hazards
        .all_hazards()
        .iter()
        .map(|h| (h.active, &h.entity))
        .unique()
    {
        if !hz_entity_same_everywhere(cde.quadtree(), hz_entity, active) {
            return false;
        }
    }
    true
}

pub fn hz_entity_same_everywhere(qt_node: &QTNode, hz_entity: &HazardEntity, active: bool) -> bool {
    if let Some(h) = qt_node
        .hazards
        .all_hazards()
        .iter()
        .find(|h| &h.entity == hz_entity)
    {
        if h.active != active {
            println!("Hazard entity activation inconsistent");
            return false;
        }
    }
    if let Some(children) = &qt_node.children {
        for child in children.as_ref() {
            if !hz_entity_same_everywhere(child, hz_entity, active) {
                return false;
            }
        }
    }

    true
}

pub fn layout_qt_matches_fresh_qt(layout: &Layout) -> bool {
    //check if every placed item is correctly represented in the quadtree

    //rebuild the quadtree
    let bin = &layout.bin;
    let mut fresh_cde = bin.base_cde.as_ref().clone();
    for (pk, pi) in layout.placed_items().iter() {
        let hazard = Hazard::new((pk, pi).into(), pi.shape.clone());
        fresh_cde.register_hazard(hazard);
    }

    qt_nodes_match(Some(layout.cde().quadtree()), Some(fresh_cde.quadtree()))
        && hazards_match(layout.cde().dynamic_hazards(), fresh_cde.dynamic_hazards())
}

fn qt_nodes_match(qn1: Option<&QTNode>, qn2: Option<&QTNode>) -> bool {
    match (qn1, qn2) {
        (Some(qn1), Some(qn2)) => {
            //if both nodes exist
            let hv1 = &qn1.hazards;
            let hv2 = &qn2.hazards;

            //collect active hazards to hashsets
            let active_haz_1 = hv1
                .active_hazards()
                .iter()
                .map(|h| (&h.entity, h.active, (&h.presence).into()))
                .collect::<HashSet<(&HazardEntity, bool, u8)>>();

            let active_haz_2 = hv2
                .active_hazards()
                .iter()
                .map(|h| (&h.entity, h.active, (&h.presence).into()))
                .collect::<HashSet<(&HazardEntity, bool, u8)>>();

            let active_in_1_but_not_2 = active_haz_1
                .difference(&active_haz_2)
                .collect::<HashSet<_>>();
            let active_in_2_but_not_1 = active_haz_2
                .difference(&active_haz_1)
                .collect::<HashSet<_>>();

            if !(active_in_1_but_not_2.is_empty() && active_in_2_but_not_1.is_empty()) {
                let from_1 = **active_in_1_but_not_2.iter().next().unwrap();
                let from_2 = **active_in_2_but_not_1.iter().next().unwrap();
                println!("{}", from_1 == from_2);
                error!(
                    "Active hazards don't match {:?} vs {:?}",
                    active_in_1_but_not_2, active_in_2_but_not_1
                );
                return false;
            }
        }
        (Some(qn1), None) => {
            if qn1.hazards.active_hazards().iter().next().is_some() {
                error!("qn1 contains active hazards while other qn2 does not exist");
                return false;
            }
        }
        (None, Some(qn2)) => {
            if qn2.hazards.active_hazards().iter().next().is_some() {
                error!("qn2 contains active hazards while other qn1 does not exist");
                return false;
            }
        }
        (None, None) => panic!("Both nodes are none"),
    }

    //Check children
    match (
        qn1.map_or(&None, |qn| &qn.children),
        qn2.map_or(&None, |qn| &qn.children),
    ) {
        (None, None) => true,
        (Some(c1), None) => {
            let qn1_has_partial_hazards = qn1.map_or(false, |qn| {
                qn.hazards
                    .active_hazards()
                    .iter()
                    .any(|h| matches!(h.presence, QTHazPresence::Partial(_)))
            });
            if qn1_has_partial_hazards {
                for child in c1.as_ref() {
                    if !qt_nodes_match(Some(child), None) {
                        return false;
                    }
                }
            }
            true
        }
        (None, Some(c2)) => {
            let qn2_has_partial_hazards = qn2.map_or(false, |qn| {
                qn.hazards
                    .active_hazards()
                    .iter()
                    .any(|h| matches!(h.presence, QTHazPresence::Partial(_)))
            });
            if qn2_has_partial_hazards {
                for child in c2.as_ref() {
                    if !qt_nodes_match(None, Some(child)) {
                        return false;
                    }
                }
            }
            true
        }
        (Some(c1), Some(c2)) => {
            for (child1, child2) in c1.as_ref().iter().zip(c2.as_ref().iter()) {
                if !qt_nodes_match(Some(child1), Some(child2)) {
                    return false;
                }
            }
            true
        }
    }
}

fn hazards_match(chv1: &[Hazard], chv2: &[Hazard]) -> bool {
    let chv1_active_hazards = chv1
        .iter()
        .filter(|h| h.active)
        .map(|h| &h.entity)
        .collect::<HashSet<_>>();

    let chv2_active_hazards = chv2
        .iter()
        .filter(|h| h.active)
        .map(|h| &h.entity)
        .collect::<HashSet<_>>();

    if chv1_active_hazards != chv2_active_hazards {
        println!("Hazard vecs don't match");
        return false;
    }
    true
}

pub fn hpg_update_no_affected_cells_remain(
    to_register: &Hazard,
    hpg: &mut HazardProximityGrid,
) -> bool {
    //To ensure the boundary fill algorithm did not miss any cells, we check all the cells to make sure no cells were affected
    let old_cells = hpg.grid.cells.clone();

    //do a full sweep of the grid, and collect the affected cells
    let undetected_cells_indices = hpg
        .grid
        .cells
        .iter_mut()
        .enumerate()
        .flat_map(|(i, cell)| cell.as_mut().map(|cell| (i, cell)))
        .map(|(i, cell)| (i, cell.register_hazard(to_register)))
        .filter(|(_i, res)| res == &HPGCellUpdate::Affected)
        .map(|(i, _res)| i)
        .collect_vec();

    if !undetected_cells_indices.is_empty() {
        //print the affected cells by row and col
        let undetected_row_cols = undetected_cells_indices
            .iter()
            .map(|i| hpg.grid.to_row_col(*i).unwrap())
            .collect_vec();
        println!(
            "{} detected affected cells, radius: {}",
            undetected_cells_indices.len(),
            hpg.cell_radius
        );
        for (&i, (row, col)) in undetected_cells_indices
            .iter()
            .zip(undetected_row_cols.iter())
        {
            println!(
                "cell [{},{}] with {} neighbors",
                row,
                col,
                hpg.grid.get_neighbors(i).map(|j| j != i).iter().count()
            );
            println!("old {:?}", &old_cells[i].as_ref().unwrap().uni_prox);
            println!("new {:?}", &hpg.grid.cells[i].as_ref().unwrap().uni_prox);
            println!()
        }
        false
    } else {
        true
    }
}

/// Checks if the quadrants follow the layout set in [AARectangle::QUADRANT_NEIGHBOR_LAYOUT]
pub fn quadrants_have_valid_layout(quadrants: &[&AARectangle; 4]) -> bool {
    let layout = AARectangle::QUADRANT_NEIGHBOR_LAYOUT;
    for (idx, q) in quadrants.iter().enumerate() {
        //make sure they share two points (an edge) with each neighbor
        let [n_0, n_1] = layout[idx];
        let q_corners = q.corners();
        let n_0_corners = quadrants[n_0].corners();
        let n_1_corners = quadrants[n_1].corners();

        assert_eq!(
            2,
            n_0_corners
                .iter()
                .filter(|c| q_corners.iter().find(|qc| qc == c).is_some())
                .count()
        );
        assert_eq!(
            2,
            n_1_corners
                .iter()
                .filter(|c| q_corners.iter().find(|qc| qc == c).is_some())
                .count()
        );
    }
    true
}

///Prints code to rebuild a layout. Intended for debugging purposes.
pub fn print_layout(layout: &Layout) {
    println!(
        "let mut layout = Layout::new(0, instance.bin({}).clone());",
        layout.bin.id
    );
    println!();

    for pi in layout.placed_items().values() {
        let transformation_str = {
            let t_decomp = &pi.d_transf;
            let (tr, (tx, ty)) = (t_decomp.rotation(), t_decomp.translation());
            format!("&DTransformation::new({:.6},({:.6},{:.6}))", tr, tx, ty)
        };

        println!(
            "layout.place_item(instance.item({}), {});",
            pi.item_id, transformation_str
        );
    }
}