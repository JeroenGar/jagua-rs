use std::collections::HashSet;

use itertools::Itertools;
use log::error;

use crate::collision_detection::cd_engine::CDEngine;
use crate::collision_detection::hazard_filters::combo_haz_filter::CombinedHazardFilter;
use crate::collision_detection::hazard_filters::entity_haz_filter::EntityHazardFilter;
use crate::collision_detection::hazard_filters::hazard_filter;
use crate::collision_detection::hazard::Hazard;
use crate::collision_detection::hazard::HazardEntity;
use crate::collision_detection::quadtree::qt_hazard::QTHazard;
use crate::collision_detection::quadtree::qt_hazard::QTHazPresence;
use crate::collision_detection::quadtree::qt_node::QTNode;
use crate::entities::bin::Bin;
use crate::entities::instance::PackingType;
use crate::entities::item::Item;
use crate::entities::layout::Layout;
use crate::entities::problems::problem::Problem;
use crate::entities::solution::Solution;
use crate::entities::layout::LayoutSnapshot;
use crate::geometry::geo_traits::{Shape, Transformable};
use crate::geometry::transformation::Transformation;
use crate::util;

///

pub fn instance_item_bin_ids_correct(items: &Vec<(Item, usize)>, packing_type: &PackingType) -> bool {
    let mut id = 0;
    for (parttype, _qty) in items {
        if parttype.id() != id {
            return false;
        }
        id += 1;
    }
    return match packing_type {
        PackingType::StripPacking { .. } => true,
        PackingType::BinPacking(bins) => {
            let mut id = 0;
            for (bin, _qty) in bins {
                if bin.id() != id {
                    return false;
                }
                id += 1;
            }
            true
        }
    };
}

pub fn problem_matches_solution<P: Problem>(problem: &P, solution: &Solution) -> bool {
    for l in problem.layouts() {
        let sl = solution.layout_snapshots.iter().find(|sl| sl.id() == l.id()).unwrap();
        match layouts_match(l, sl) {
            true => continue,
            false => return false,
        }
    }
    true
}

pub fn layouts_match(layout: &Layout, layout_snapshot: &LayoutSnapshot) -> bool {
    if layout.bin().id() != layout_snapshot.bin().id() {
        return false;
    }
    for sp_item in layout_snapshot.placed_items().iter() {
        if layout.placed_items().iter().find(|sp| sp.uid() == sp_item.uid()).is_none() {
            return false;
        }
    }
    //TODO: add dotgrid check, check if quadtree does not contain any more uncommitted removals
    true
}


pub fn collision_hazards_sorted_correctly(hazards: &Vec<QTHazard>) -> bool {
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
    return true;
}

pub fn all_bins_and_items_centered(items: &Vec<(Item, usize)>, bins: &Vec<(Bin, usize)>) -> bool {
    items.iter().map(|(i, _)| i.shape().centroid())
        .chain(bins.iter().map(|(b, _)| b.outer().centroid()))
        .all(|c| almost::zero(c.0) && almost::zero(c.1))
}

pub fn item_to_place_does_not_collide(item: &Item, transformation: &Transformation, layout: &Layout) -> bool {
    let haz_filter = item.hazard_filter();

    let shape = item.shape();
    let t_shape = shape.transform_clone(transformation);

    let entities_to_ignore = haz_filter
        .map_or(vec![], |f| hazard_filter::get_irrelevant_hazard_entities(f, layout.cde().all_hazards()));

    if layout.cde().surrogate_collides(shape.surrogate(), transformation, &entities_to_ignore) ||
        layout.cde().shape_collides(&t_shape, &entities_to_ignore) {
        return false;
    }
    return true;
}

pub fn layout_is_collision_free(layout: &Layout) -> bool {
    for pi in layout.placed_items() {
        let hef = EntityHazardFilter::new().add(pi.into());
        let combo_filter = match pi.haz_filter() {
            None => CombinedHazardFilter::new().add(&hef),
            Some(hf) => CombinedHazardFilter::new().add(&hef).add(hf)
        };
        let entities_to_ignore = hazard_filter::get_irrelevant_hazard_entities(&combo_filter, layout.cde().all_hazards());

        if layout.cde().shape_collides(pi.shape(), &entities_to_ignore) {
            println!("Collision detected for item {:.?}", pi.uid());
            util::print_layout(layout);
            return false;
        }
    }
    return true;
}

pub fn qt_node_contains_no_deactivated_hazards<'a>(node: &'a QTNode, mut stacktrace: Vec<&'a QTNode>) -> (bool, Vec<&'a QTNode>) {
    stacktrace.push(node);
    let deactivated_hazard = node.hazards().all_hazards().iter().find(|h| !h.active);
    if deactivated_hazard.is_some() {
        println!("Deactivated hazard found");
        dbg!(&stacktrace);
        return (false, stacktrace);
    }

    match node.children() {
        Some(children) => {
            for child in children.as_ref() {
                let result = qt_node_contains_no_deactivated_hazards(child, stacktrace);
                stacktrace = result.1;
                let contains_no_deactivated_hazards = result.0;
                if !contains_no_deactivated_hazards {
                    return (false, stacktrace);
                }
            }
        }
        None => {}
    }

    stacktrace.pop();
    (true, stacktrace)
}

pub fn qt_contains_no_dangling_hazards(cde: &CDEngine) -> bool {
    if let Some(children) = cde.quadtree().children() {
        for child in children.as_ref() {
            if !qt_node_contains_no_dangling_hazards(child, cde.quadtree()) {
                return false;
            }
        }
    }
    true
}

fn qt_node_contains_no_dangling_hazards(node: &QTNode, parent: &QTNode) -> bool {
    let parent_h_entities = parent.hazards().all_hazards().iter().map(|h| &h.entity).unique().collect_vec();

    let dangling_hazards = node.hazards().all_hazards().iter().any(|h| !parent_h_entities.contains(&&h.entity));
    if dangling_hazards {
        println!("Node contains dangling hazard");
        return false;
    }

    match node.children() {
        Some(children) => {
            for child in children.as_ref() {
                if !qt_node_contains_no_dangling_hazards(child, node) {
                    return false;
                }
            }
        }
        None => {}
    }

    true
}

pub fn qt_hz_entity_activation_consistent(cde: &CDEngine) -> bool {
    for (active, hz_entity) in cde.quadtree().hazards().all_hazards().iter().map(|h| (h.active, &h.entity)).unique() {
        if !hz_entity_same_everywhere(cde.quadtree(), &hz_entity, active) {
            return false;
        }
    }
    true
}

pub fn hz_entity_same_everywhere(qt_node: &QTNode, hz_entity: &HazardEntity, active: bool) -> bool {
    match qt_node.hazards().all_hazards().iter().find(|h| &h.entity == hz_entity) {
        Some(h) => {
            if h.active != active {
                println!("Hazard entity activation inconsistent");
                return false;
            }
        }
        None => {}
    }
    if let Some(children) = qt_node.children() {
        for child in children.as_ref() {
            if !hz_entity_same_everywhere(child, hz_entity, active) {
                return false;
            }
        }
    }

    return true;
}

pub fn layout_qt_matches_fresh_qt(layout: &Layout) -> bool {
    //check if every placed item is correctly represented in the quadtree

    //rebuild the quadtree
    let bin = layout.bin();
    let mut fresh_cde = bin.base_cde().clone();
    for pi in layout.placed_items() {
        fresh_cde.register_hazard(pi.into());
    }

    qt_nodes_match(Some(layout.cde().quadtree()), Some(fresh_cde.quadtree())) &&
        hazards_match(layout.cde().dynamic_hazards(), fresh_cde.dynamic_hazards())
}

fn qt_nodes_match(qn1: Option<&QTNode>, qn2: Option<&QTNode>) -> bool {
    match (qn1, qn2) {
        (Some(qn1), Some(qn2)) => {
            //if both nodes exist
            let hv1 = qn1.hazards();
            let hv2 = qn2.hazards();

            //collect active hazards to hashsets
            let active_haz_1 = hv1.active_hazards().iter()
                .map(|h| (&h.entity, h.active, (&h.presence).into()))
                .collect::<HashSet<(&HazardEntity, bool, u8)>>();

            let active_haz_2 = hv2.active_hazards().iter()
                .map(|h| (&h.entity, h.active, (&h.presence).into()))
                .collect::<HashSet<(&HazardEntity, bool, u8)>>();

            let active_in_1_but_not_2 = active_haz_1.difference(&active_haz_2).collect::<HashSet<_>>();
            let active_in_2_but_not_1 = active_haz_2.difference(&active_haz_1).collect::<HashSet<_>>();

            if !(active_in_1_but_not_2.is_empty() && active_in_2_but_not_1.is_empty()) {
                let from_1 = **active_in_1_but_not_2.iter().next().unwrap();
                let from_2 = **active_in_2_but_not_1.iter().next().unwrap();
                println!("{}", from_1 == from_2);
                error!("Active hazards don't match {:?} vs {:?}", active_in_1_but_not_2, active_in_2_but_not_1);
                return false;
            }
        }
        (Some(qn1), None) => {
            if qn1.hazards().active_hazards().iter().next().is_some() {
                error!("qn1 contains active hazards while other qn2 does not exist");
                return false;
            }
        }
        (None, Some(qn2)) => {
            if qn2.hazards().active_hazards().iter().next().is_some() {
                error!("qn2 contains active hazards while other qn1 does not exist");
                return false;
            }
        }
        (None, None) => panic!("Both nodes are none"),
    }

    //Check children
    match (qn1.map_or(&None, |qn| qn.children()), qn2.map_or(&None, |qn| qn.children())) {
        (None, None) => true,
        (Some(c1), None) => {
            let qn1_has_partial_hazards =
                qn1.map_or(
                    false,
                    |qn| {
                        qn.hazards().active_hazards().iter()
                            .any(|h| matches!(h.presence, QTHazPresence::Partial(_)))
                    },
                );
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
            let qn2_has_partial_hazards =
                qn2.map_or(
                    false,
                    |qn| qn.hazards().active_hazards().iter()
                        .any(|h| matches!(h.presence, QTHazPresence::Partial(_))),
                );
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
    let chv1_active_hazards = chv1.iter()
        .filter(|h| h.active)
        .map(|h| &h.entity)
        .collect::<HashSet<_>>();

    let chv2_active_hazards = chv2.iter()
        .filter(|h| h.active)
        .map(|h| &h.entity)
        .collect::<HashSet<_>>();

    if chv1_active_hazards != chv2_active_hazards {
        println!("Hazard vecs don't match");
        return false;
    }
    true
}