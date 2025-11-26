//! Simple single-run nesting strategy

use crate::svg_nesting::{
    parsing::{
        calculate_signed_area, extract_path_from_svg_bytes, parse_svg_path, reverse_winding,
    },
    strategy::NestingStrategy,
    svg_generation::{combine_svg_documents, NestingResult, post_process_svg},
};
use anyhow::Result;
use jagua_rs::collision_detection::CDEConfig;
use jagua_rs::entities::{Container, Item, Layout};
use jagua_rs::geometry::DTransformation;
use jagua_rs::geometry::OriginalShape;
use jagua_rs::geometry::fail_fast::SPSurrogateConfig;
use jagua_rs::geometry::geo_enums::RotationRange;
use jagua_rs::geometry::primitives::{Rect, SPolygon};
use jagua_rs::geometry::shape_modification::ShapeModifyMode;
use jagua_rs::io::import::Importer;
use jagua_rs::io::svg::{SvgDrawOptions, s_layout_to_svg};
use jagua_rs::probs::bpp::entities::{BPInstance, Bin};
use lbf::config::LBFConfig;
use lbf::opt::lbf_bpp::LBFOptimizerBP;
use rand::SeedableRng;
use rand::prelude::SmallRng;

/// Simple nesting strategy that runs the optimizer once with default parameters
pub struct SimpleNestingStrategy;

impl SimpleNestingStrategy {
    pub fn new() -> Self {
        Self
    }
}

impl Default for SimpleNestingStrategy {
    fn default() -> Self {
        Self::new()
    }
}

impl NestingStrategy for SimpleNestingStrategy {
    fn nest(
        &self,
        bin_width: f32,
        bin_height: f32,
        spacing: f32,
        svg_part_bytes: &[u8],
        amount_of_parts: usize,
        amount_of_rotations: usize,
    ) -> Result<NestingResult> {
        // Parse SVG
        let path_data = extract_path_from_svg_bytes(svg_part_bytes)?;
        let (polygon_points, holes) = parse_svg_path(&path_data)?;

        log::debug!(
            "Parsed SVG path: {} outer boundary points, {} holes",
            polygon_points.len(),
            holes.len()
        );
        for (i, hole) in holes.iter().enumerate() {
            log::debug!("  Hole {}: {} points", i, hole.len());
        }

        // Ensure outer boundary is counter-clockwise (positive area)
        let outer_area = calculate_signed_area(&polygon_points);
        let polygon_points = if outer_area < 0.0 {
            reverse_winding(&polygon_points)
        } else {
            polygon_points
        };

        // Ensure holes are clockwise (negative area) - opposite of outer boundary
        let mut processed_holes = Vec::new();
        for (i, hole) in holes.iter().enumerate() {
            let hole_area = calculate_signed_area(hole);
            let processed_hole = if hole_area > 0.0 {
                log::debug!(
                    "  Reversing hole {} (was counter-clockwise, area: {})",
                    i,
                    hole_area
                );
                reverse_winding(hole)
            } else {
                log::debug!("  Hole {} is clockwise (area: {})", i, hole_area);
                hole.clone()
            };
            processed_holes.push(processed_hole);
        }

        log::debug!("Processed {} holes for SVG output", processed_holes.len());

        // Build geometry
        let polygon = SPolygon::new(polygon_points.clone())?;
        let centroid = polygon.centroid();
        let pre_transform = DTransformation::new(0.0, (-centroid.x(), -centroid.y()));

        let cde_config = CDEConfig {
            quadtree_depth: 5,
            cd_threshold: 16,
            item_surrogate_config: SPSurrogateConfig {
                n_pole_limits: [(100, 0.0), (20, 0.75), (10, 0.90)],
                n_ff_poles: 2,
                n_ff_piers: 0,
            },
        };

        let importer = Importer::new(cde_config.clone(), Some(0.001), Some(spacing), None);

        let item_shape = OriginalShape {
            shape: polygon,
            pre_transform,
            modify_mode: ShapeModifyMode::Inflate,
            modify_config: importer.shape_modify_config,
        };

        let bin_rect = Rect::try_new(0.0, 0.0, bin_width, bin_height)?;
        let bin_polygon = SPolygon::from(bin_rect);
        let container_shape = OriginalShape {
            shape: bin_polygon,
            pre_transform: DTransformation::empty(),
            modify_mode: ShapeModifyMode::Deflate,
            modify_config: importer.shape_modify_config,
        };

        let container_template = Container::new(0, container_shape, vec![], cde_config.clone())?;

        // Build instance with requested rotations
        let rotation_count = if amount_of_rotations == 0 {
            0
        } else {
            amount_of_rotations.max(1)
        };

        let rotation_range = if rotation_count == 0 {
            RotationRange::None
        } else if rotation_count == 1 {
            RotationRange::Discrete(vec![0.0])
        } else {
            let rotations: Vec<f32> = (0..rotation_count)
                .map(|i| (i as f32 * 2.0 * std::f32::consts::PI) / (rotation_count as f32))
                .collect();
            RotationRange::Discrete(rotations)
        };

        let mut items = Vec::with_capacity(amount_of_parts);
        for i in 0..amount_of_parts {
            let item = Item::new(
                i,
                item_shape.clone(),
                rotation_range.clone(),
                None,
                cde_config.item_surrogate_config,
            )?;
            items.push((item, 1));
        }
        // Limit to single bin to ensure realistic packing (multiple bins might allow invalid placements)
        let bin = Bin::new(container_template.clone(), 1, 0);
        let instance = BPInstance::new(items, vec![bin]);

        // Run optimizer with increased samples for better results
        // Try multiple runs with different seeds and take the best result
        let mut best_solution = None;
        let mut best_placed = 0;
        
        for seed in 0..10 {
            let lbf_config = LBFConfig {
                cde_config: cde_config.clone(),
                poly_simpl_tolerance: Some(0.001),
                min_item_separation: Some(spacing),
                prng_seed: Some(seed),
                n_samples: 200000, // Increased to find better packing solutions
                ls_frac: 0.2,
                narrow_concavity_cutoff_ratio: None,
                svg_draw_options: Default::default(),
            };

            let mut optimizer =
                LBFOptimizerBP::new(instance.clone(), lbf_config, SmallRng::seed_from_u64(seed));
            let solution = optimizer.solve();
            
            let placed: usize = solution
                .layout_snapshots
                .values()
                .map(|ls| ls.placed_items.len())
                .sum();
            
            if placed > best_placed {
                best_placed = placed;
                best_solution = Some(solution);
            }
            
            // If we've placed all items, no need to continue
            if placed >= amount_of_parts {
                break;
            }
        }
        
        let solution = best_solution.expect("At least one optimization run should succeed");

        // Count items directly from the solution - this should match what's in the SVG
        let total_items_placed: usize = solution
            .layout_snapshots
            .values()
            .map(|ls| ls.placed_items.len())
            .sum();

        log::debug!("Optimization complete: {} parts placed", total_items_placed);
        log::debug!(
            "Solution has {} layout snapshots, best_placed was {}",
            solution.layout_snapshots.len(),
            best_placed
        );
        for (key, ls) in solution.layout_snapshots.iter() {
            log::debug!(
                "Layout {:?} (container id: {}) has {} placed items",
                key,
                ls.container.id,
                ls.placed_items.len()
            );
        }
        
        // Verify count matches what will be rendered in SVG
        let total_in_snapshots: usize = solution
            .layout_snapshots
            .values()
            .map(|ls| ls.placed_items.len())
            .sum();
        
        if total_items_placed != total_in_snapshots {
            log::warn!(
                "Count mismatch: total_items_placed={}, total_in_snapshots={}",
                total_items_placed,
                total_in_snapshots
            );
        }

        // Generate SVG output
        let svg_options = SvgDrawOptions::default();
        let mut page_svg_strings: Vec<String> = Vec::new();
        let mut page_svgs: Vec<Vec<u8>> = Vec::new();

        let mut layout_entries: Vec<_> = solution.layout_snapshots.iter().collect();
        layout_entries.sort_by_key(|(_, layout_snapshot)| layout_snapshot.container.id);

        for (layout_key, layout_snapshot) in layout_entries {
            let svg_doc = s_layout_to_svg(
                layout_snapshot,
                &instance,
                svg_options,
                &format!("Layout {:?} - {} items", layout_key, total_items_placed),
            );
            let svg_str = svg_doc.to_string();
            let processed_svg = post_process_svg(&svg_str, &processed_holes);
            page_svg_strings.push(processed_svg.clone());
            page_svgs.push(processed_svg.into_bytes());
        }

        // Combine all page SVGs into a single valid SVG document
        let combined_svg = combine_svg_documents(&page_svg_strings, bin_width, bin_height);

        // Verify the count matches what's actually in the SVG
        // Count <use> tags that reference items (not definitions in <defs>)
        // Pattern: <use href="#item_0" or <use xlink:href="#item_0" or similar
        // Use regex to match <use> tags with href="#item_" pattern
        use regex::Regex;
        let re_item_use = Regex::new(r##"<use[^>]*href=["']#item_\d+["']"##).unwrap();
        let items_in_svg = re_item_use.find_iter(&combined_svg).count();
        
        // Also count items per page to detect duplicates
        let mut items_per_page: Vec<usize> = Vec::new();
        for page_svg_str in &page_svg_strings {
            let page_count = re_item_use.find_iter(page_svg_str).count();
            items_per_page.push(page_count);
        }
        
        log::debug!(
            "SVG verification: {} item <use> tags found across {} pages (page counts: {:?}), expected {}",
            items_in_svg,
            page_svg_strings.len(),
            items_per_page,
            total_items_placed
        );
        
        // Use the actual count from SVG as the source of truth
        // The optimizer might report more items than actually fit
        let corrected_count = items_in_svg;
        
        if corrected_count != total_items_placed {
            log::warn!(
                "Count mismatch detected: SVG contains {} item <use> tags, but optimizer reports {}",
                corrected_count,
                total_items_placed
            );
            log::warn!("Using SVG count ({}) as source of truth", corrected_count);
        }

        // Generate SVG for unplaced parts if any (use corrected count)
        let unplaced_count = amount_of_parts.saturating_sub(corrected_count);
        log::debug!(
            "Unplaced parts calculation: {} requested - {} placed = {} unplaced",
            amount_of_parts,
            corrected_count,
            unplaced_count
        );
        
        let unplaced_parts_svg = if unplaced_count > 0 {
            // Use the same rendering logic as placed parts - create a layout with unplaced items in a grid
            use jagua_rs::entities::Instance;
            
            // Create a new layout with the same container
            let mut unplaced_layout = Layout::new(container_template.clone());
            
            // Calculate grid layout for unplaced parts
            let part_bbox = &item_shape.shape.bbox;
            let part_width = part_bbox.width();
            let part_height = part_bbox.height();
            
            let cols = ((bin_width - spacing) / (part_width + spacing)).floor().max(1.0) as usize;
            let rows = ((unplaced_count as f32 / cols as f32).ceil()) as usize;
            
            let total_grid_width = (cols as f32 * part_width) + ((cols.saturating_sub(1)) as f32 * spacing);
            let total_grid_height = (rows as f32 * part_height) + ((rows.saturating_sub(1)) as f32 * spacing);
            let offset_x = (bin_width - total_grid_width) / 2.0;
            let offset_y = (bin_height - total_grid_height) / 2.0;
            
            // Get the item to place (use item 0 as template)
            let item_template = instance.item(0);
            
            // Place unplaced items in grid layout
            for i in 0..unplaced_count {
                let row = i / cols;
                let col = i % cols;
                
                // Calculate grid position (center of grid cell) in bin coordinates
                let grid_x = offset_x + (col as f32 * (part_width + spacing)) + part_width / 2.0;
                let grid_y = offset_y + (row as f32 * (part_height + spacing)) + part_height / 2.0;
                
                // Create transformation: translate to grid position (no rotation)
                // The item's shape_orig has pre_transform that centers it, so we need to account for that
                // When placing, the transform is applied after the pre_transform, so we just translate to grid position
                let d_transf = DTransformation::new(0.0, (grid_x, grid_y));
                
                // Place the item in the layout
                unplaced_layout.place_item(item_template, d_transf);
            }
            
            // Create layout snapshot and render using the same logic as placed parts
            let unplaced_snapshot = unplaced_layout.save();
            // Disable CD shape highlighting to avoid green dashed lines
            let mut svg_options = SvgDrawOptions::default();
            svg_options.highlight_cd_shapes = false;
            let unplaced_svg_doc = s_layout_to_svg(
                &unplaced_snapshot,
                &instance,
                svg_options,
                &format!("Unplaced parts: {}", unplaced_count),
            );
            let unplaced_svg_str = unplaced_svg_doc.to_string();
            let processed_unplaced_svg = post_process_svg(&unplaced_svg_str, &processed_holes);
            
            log::debug!("Generated unplaced parts SVG with {} parts using same rendering logic", unplaced_count);
            Some(processed_unplaced_svg.into_bytes())
        } else {
            log::debug!("No unplaced parts, unplaced_parts_svg is None");
            None
        };

        // Always use SVG count as source of truth since optimizer might report incorrect counts
        Ok(NestingResult {
            combined_svg: combined_svg.into_bytes(),
            page_svgs,
            parts_placed: corrected_count,
            total_parts_requested: amount_of_parts,
            unplaced_parts_svg,
        })
    }
}
