//! Adaptive nesting strategy that starts with lower parameters and adaptively increases them

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
use jagua_rs::probs::bpp::entities::{BPInstance, Bin, BPSolution};
use lbf::config::LBFConfig;
use lbf::opt::lbf_bpp::LBFOptimizerBP;
use rand::SeedableRng;
use rand::prelude::SmallRng;
use std::time::Instant;

/// Adaptive nesting strategy that starts with lower parameters and adaptively increases them
/// based on results. Sends intermediate improvements via callback.
pub struct AdaptiveNestingStrategy {
    /// Optional function to check if optimization should be cancelled
    cancellation_checker: Option<Box<dyn Fn() -> bool + Send + Sync>>,
}

impl AdaptiveNestingStrategy {
    /// Create a new adaptive nesting strategy
    pub fn new() -> Self {
        Self {
            cancellation_checker: None,
        }
    }

    /// Create a new adaptive nesting strategy with cancellation checking
    pub fn with_cancellation_checker(
        cancellation_checker: Box<dyn Fn() -> bool + Send + Sync>,
    ) -> Self {
        Self {
            cancellation_checker: Some(cancellation_checker),
        }
    }

    /// Check if optimization should be cancelled
    fn is_cancelled(&self) -> bool {
        self.cancellation_checker
            .as_ref()
            .map(|checker| checker())
            .unwrap_or(false)
    }

    /// Run a single optimization run with given parameters
    fn run_single_optimization(
        &self,
        instance: &BPInstance,
        cde_config: &CDEConfig,
        spacing: f32,
        loops: usize,
        placements: usize,
        seed_offset: usize,
    ) -> Result<(usize, BPSolution)> {
        let mut best_solution = None;
        let mut best_placed = 0;

        for loop_idx in 0..loops {
            if self.is_cancelled() {
                log::info!("Cancellation detected, stopping optimization");
                break;
            }

            let seed = (seed_offset * 1000 + loop_idx) as u64;
            let lbf_config = LBFConfig {
                cde_config: cde_config.clone(),
                poly_simpl_tolerance: Some(0.001),
                min_item_separation: Some(spacing),
                prng_seed: Some(seed),
                n_samples: placements,
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
            if placed >= instance.total_item_qty() {
                break;
            }
        }

        Ok((
            best_placed,
            best_solution.expect("At least one optimization run should succeed"),
        ))
    }

    /// Generate SVG from solution
    fn generate_svg_from_solution(
        &self,
        solution: &BPSolution,
        instance: &BPInstance,
        processed_holes: &[Vec<jagua_rs::geometry::primitives::Point>],
        bin_width: f32,
        bin_height: f32,
        amount_of_parts: usize,
    ) -> Result<NestingResult> {
        // Count items directly from the solution
        let total_items_placed: usize = solution
            .layout_snapshots
            .values()
            .map(|ls| ls.placed_items.len())
            .sum();

        log::debug!("Optimization complete: {} parts placed", total_items_placed);

        // Generate SVG output
        let svg_options = SvgDrawOptions::default();
        let mut page_svg_strings: Vec<String> = Vec::new();
        let mut page_svgs: Vec<Vec<u8>> = Vec::new();

        let mut layout_entries: Vec<_> = solution.layout_snapshots.iter().collect();
        layout_entries.sort_by_key(|(_, layout_snapshot)| layout_snapshot.container.id);

        for (layout_key, layout_snapshot) in layout_entries {
            let svg_doc = s_layout_to_svg(
                layout_snapshot,
                instance,
                svg_options,
                &format!("Layout {:?} - {} items", layout_key, total_items_placed),
            );
            let svg_str = svg_doc.to_string();
            let processed_svg = post_process_svg(&svg_str, processed_holes);
            page_svg_strings.push(processed_svg.clone());
            page_svgs.push(processed_svg.into_bytes());
        }

        // Combine all page SVGs into a single valid SVG document
        let combined_svg = combine_svg_documents(&page_svg_strings, bin_width, bin_height);

        // Verify the count matches what's actually in the SVG
        use regex::Regex;
        let re_item_use = Regex::new(r##"<use[^>]*href=["']#item_\d+["']"##).unwrap();
        let items_in_svg = re_item_use.find_iter(&combined_svg).count();

        // Use the actual count from SVG as the source of truth
        let corrected_count = items_in_svg;

        if corrected_count != total_items_placed {
            log::warn!(
                "Count mismatch detected: SVG contains {} item <use> tags, but optimizer reports {}",
                corrected_count,
                total_items_placed
            );
        }

        // Generate SVG for unplaced parts if any
        let unplaced_count = amount_of_parts.saturating_sub(corrected_count);
        let unplaced_parts_svg = if unplaced_count > 0 {
            // For unplaced parts, we'll use the same logic as SimpleNestingStrategy
            // This requires access to container_template and item_shape, which we'll need to pass
            // For now, return None and handle it in the main nest function
            None
        } else {
            None
        };

        Ok(NestingResult {
            combined_svg: combined_svg.into_bytes(),
            page_svgs,
            parts_placed: corrected_count,
            total_parts_requested: amount_of_parts,
            unplaced_parts_svg,
        })
    }
}

impl Default for AdaptiveNestingStrategy {
    fn default() -> Self {
        Self::new()
    }
}

impl NestingStrategy for AdaptiveNestingStrategy {
    fn nest(
        &self,
        bin_width: f32,
        bin_height: f32,
        spacing: f32,
        svg_part_bytes: &[u8],
        amount_of_parts: usize,
        amount_of_rotations: usize,
        improvement_callback: Option<crate::svg_nesting::strategy::ImprovementCallback>,
    ) -> Result<NestingResult> {
        // Parse SVG (same as SimpleNestingStrategy)
        let path_data = extract_path_from_svg_bytes(svg_part_bytes)?;
        let (polygon_points, holes) = parse_svg_path(&path_data)?;

        log::debug!(
            "Parsed SVG path: {} outer boundary points, {} holes",
            polygon_points.len(),
            holes.len()
        );

        // Ensure outer boundary is counter-clockwise (positive area)
        let outer_area = calculate_signed_area(&polygon_points);
        let polygon_points = if outer_area < 0.0 {
            reverse_winding(&polygon_points)
        } else {
            polygon_points
        };

        // Ensure holes are clockwise (negative area)
        let mut processed_holes = Vec::new();
        for hole in holes.iter() {
            let hole_area = calculate_signed_area(hole);
            let processed_hole = if hole_area > 0.0 {
                reverse_winding(hole)
            } else {
                hole.clone()
            };
            processed_holes.push(processed_hole);
        }

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

        // Build instance with requested rotations (capped at 4)
        const MAX_ROTATIONS: usize = 4;
        let rotation_count = if amount_of_rotations == 0 {
            0
        } else {
            amount_of_rotations.max(1).min(MAX_ROTATIONS)
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

        let bin = Bin::new(container_template.clone(), 1, 0);
        let instance = BPInstance::new(items, vec![bin]);

        // Adaptive optimization loop
        let mut loops = 1; // Start with 1 loop
        let mut placements = 10000; // Start with 10000 placements (n_samples)
        let _rotations = amount_of_rotations; // Start with requested rotations (currently not varied)

        let mut best_result: Option<NestingResult> = None;
        let mut best_placed = 0;
        let mut total_runs = 0;
        const MAX_TOTAL_RUNS: usize = 10000;
        const MAX_RUNS_WITHOUT_IMPROVEMENT: usize = 10;
        const MAX_RUN_DURATION_SECONDS: u64 = 60;

        loop {
            if self.is_cancelled() {
                log::info!("Cancellation detected, stopping adaptive optimization");
                break;
            }

            if total_runs >= MAX_TOTAL_RUNS {
                log::info!("Reached maximum total runs ({}), stopping", MAX_TOTAL_RUNS);
                break;
            }

            // Try 10 runs with current parameters
            let mut improved_this_batch = false;
            for batch_run in 0..MAX_RUNS_WITHOUT_IMPROVEMENT {
                if self.is_cancelled() {
                    break;
                }

                if total_runs >= MAX_TOTAL_RUNS {
                    break;
                }

                let run_start = Instant::now();
                total_runs += 1;

                log::info!(
                    "Run {}/{} (batch {}/{}): loops={}, placements={}, rotations={}",
                    total_runs,
                    MAX_TOTAL_RUNS,
                    batch_run + 1,
                    MAX_RUNS_WITHOUT_IMPROVEMENT,
                    loops,
                    placements,
                    amount_of_rotations
                );

                // Run optimization
                let (_placed, solution) = self.run_single_optimization(
                    &instance,
                    &cde_config,
                    spacing,
                    loops,
                    placements,
                    total_runs,
                )?;

                let run_duration = run_start.elapsed();
                if run_duration.as_secs() > MAX_RUN_DURATION_SECONDS {
                    log::warn!(
                        "Run took {} seconds, exceeding limit of {} seconds. Stopping.",
                        run_duration.as_secs(),
                        MAX_RUN_DURATION_SECONDS
                    );
                    break;
                }

                // Generate SVG result
                let mut result = self.generate_svg_from_solution(
                    &solution,
                    &instance,
                    &processed_holes,
                    bin_width,
                    bin_height,
                    amount_of_parts,
                )?;

                // Handle unplaced parts SVG generation
                if result.parts_placed < amount_of_parts {
                    use jagua_rs::entities::Instance;
                    let mut unplaced_layout = Layout::new(container_template.clone());
                    let unplaced_count = amount_of_parts - result.parts_placed;
                    let part_bbox = &item_shape.shape.bbox;
                    let part_width = part_bbox.width();
                    let part_height = part_bbox.height();
                    let cols = ((bin_width - spacing) / (part_width + spacing))
                        .floor()
                        .max(1.0) as usize;
                    let rows = ((unplaced_count as f32 / cols as f32).ceil()) as usize;
                    let total_grid_width =
                        (cols as f32 * part_width) + ((cols.saturating_sub(1)) as f32 * spacing);
                    let total_grid_height =
                        (rows as f32 * part_height) + ((rows.saturating_sub(1)) as f32 * spacing);
                    let offset_x = (bin_width - total_grid_width) / 2.0;
                    let offset_y = (bin_height - total_grid_height) / 2.0;
                    let item_template = instance.item(0);

                    for i in 0..unplaced_count {
                        let row = i / cols;
                        let col = i % cols;
                        let grid_x =
                            offset_x + (col as f32 * (part_width + spacing)) + part_width / 2.0;
                        let grid_y =
                            offset_y + (row as f32 * (part_height + spacing)) + part_height / 2.0;
                        let d_transf = DTransformation::new(0.0, (grid_x, grid_y));
                        unplaced_layout.place_item(item_template, d_transf);
                    }

                    let unplaced_snapshot = unplaced_layout.save();
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
                    result.unplaced_parts_svg = Some(processed_unplaced_svg.into_bytes());
                }

                log::info!(
                    "Run {} completed: {} parts placed in {:.2}s",
                    total_runs,
                    result.parts_placed,
                    run_duration.as_secs_f64()
                );

                // Check if this is an improvement
                if result.parts_placed > best_placed {
                    improved_this_batch = true;
                    let improvement = result.parts_placed - best_placed;
                    best_placed = result.parts_placed;
                    best_result = Some(result.clone());

                    log::info!(
                        "New best result: {} parts placed (improvement of {})",
                        best_placed,
                        improvement
                    );

                    // Send improvement via callback
                    if let Some(ref callback) = improvement_callback {
                        if let Err(e) = callback(result.clone()) {
                            log::warn!("Failed to send improvement callback: {}", e);
                        }
                    }

                    // If we've placed all items, we're done
                    if result.parts_placed >= amount_of_parts {
                        log::info!("All parts placed, stopping optimization");
                        return Ok(best_result.unwrap());
                    }
                }
            }

            // If no improvement after 10 runs, increase parameters
            if !improved_this_batch {
                loops += 1;
                placements = (placements * 2).min(200000); // Double placements, cap at 200000
                log::info!(
                    "No improvement after {} runs, increasing parameters: loops={}, placements={}",
                    MAX_RUNS_WITHOUT_IMPROVEMENT,
                    loops,
                    placements
                );
            }
        }

        // Return best result found
        Ok(best_result.unwrap_or_else(|| {
            // If no result found, return empty result
            NestingResult {
                combined_svg: Vec::new(),
                page_svgs: Vec::new(),
                parts_placed: 0,
                total_parts_requested: amount_of_parts,
                unplaced_parts_svg: None,
            }
        }))
    }
}

