//! SVG nesting utilities for jagua-rs
//!
//! This module provides utilities for nesting SVG parts into bins using the jagua-rs
//! collision detection engine and LBF optimizer.

use anyhow::Result;
use jagua_rs::collision_detection::CDEConfig;
use jagua_rs::entities::{Container, Item};
use jagua_rs::geometry::DTransformation;
use jagua_rs::geometry::OriginalShape;
use jagua_rs::geometry::fail_fast::SPSurrogateConfig;
use jagua_rs::geometry::geo_enums::RotationRange;
use jagua_rs::geometry::primitives::{Point, Rect, SPolygon};
use jagua_rs::geometry::shape_modification::ShapeModifyMode;
use jagua_rs::io::import::Importer;
use jagua_rs::io::svg::{SvgDrawOptions, s_layout_to_svg};
use jagua_rs::probs::bpp::entities::{BPInstance, Bin};
use lbf::config::LBFConfig;
use lbf::opt::lbf_bpp::LBFOptimizerBP;
use rand::SeedableRng;
use rand::prelude::SmallRng;

/// Helper function to parse a coordinate pair from SVG path tokens
/// Handles both "x,y" format and "x y" format
/// Returns (x, y, tokens_to_advance) where tokens_to_advance is the number of tokens to advance
/// (not including the current command token)
fn parse_coordinate_pair(
    coord_str: &str,
    start_idx: usize,
    tokens: &[&str],
) -> Result<(f32, f32, usize)> {
    if let Some(comma_idx) = coord_str.find(',') {
        let x: f32 = coord_str[..comma_idx].parse()?;
        let y_str = &coord_str[comma_idx + 1..];
        let y: f32 = y_str.trim_end_matches(',').parse()?;
        Ok((x, y, 1))
    } else {
        let x: f32 = coord_str.trim_end_matches(',').parse()?;
        if start_idx + 1 < tokens.len() {
            let y: f32 = tokens[start_idx + 1].trim_end_matches(',').parse()?;
            Ok((x, y, 2))
        } else {
            anyhow::bail!("Incomplete coordinate pair at token {}", start_idx);
        }
    }
}

/// Converts a circle to SVG path data (approximated as a polygon)
/// Uses 32 segments for a smooth circle approximation
fn circle_to_path(cx: f32, cy: f32, r: f32) -> String {
    let segments = 32;
    let mut path = String::new();
    for i in 0..=segments {
        let angle = 2.0 * std::f32::consts::PI * (i as f32) / (segments as f32);
        let x = cx + r * angle.cos();
        let y = cy + r * angle.sin();
        if i == 0 {
            path.push_str(&format!("M {},{}", x, y));
        } else {
            path.push_str(&format!(" L {},{}", x, y));
        }
    }
    path.push_str(" z");
    path
}

/// Extracts circle attributes from a circle element
fn extract_circle_attributes(circle_str: &str) -> Option<(f32, f32, f32)> {
    let mut cx = None;
    let mut cy = None;
    let mut r = None;

    // Helper to extract attribute value
    let extract_attr = |attr_name: &str, text: &str| -> Option<f32> {
        // Try with double quotes
        let pattern1 = format!("{}=\"", attr_name);
        if let Some(start) = text.find(&pattern1) {
            let start = start + pattern1.len();
            if let Some(end) = text[start..].find("\"") {
                let val_str = text[start..start + end].trim();
                if let Ok(val) = val_str.parse::<f32>() {
                    return Some(val);
                }
            }
        }
        // Try with single quotes
        let pattern2 = format!("{}='", attr_name);
        if let Some(start) = text.find(&pattern2) {
            let start = start + pattern2.len();
            if let Some(end) = text[start..].find("'") {
                let val_str = text[start..start + end].trim();
                if let Ok(val) = val_str.parse::<f32>() {
                    return Some(val);
                }
            }
        }
        // Try without quotes (space-separated)
        let pattern3 = format!("{}= ", attr_name);
        if let Some(start) = text.find(&pattern3) {
            let start = start + pattern3.len();
            let remaining = &text[start..];
            if let Some(end) = remaining.find(|c: char| c == ' ' || c == '/' || c == '>') {
                let val_str = remaining[..end].trim();
                if let Ok(val) = val_str.parse::<f32>() {
                    return Some(val);
                }
            }
        }
        None
    };

    cx = extract_attr("cx", circle_str);
    cy = extract_attr("cy", circle_str);
    r = extract_attr("r", circle_str);

    if let (Some(cx_val), Some(cy_val), Some(r_val)) = (cx, cy, r) {
        Some((cx_val, cy_val, r_val))
    } else {
        None
    }
}

/// Extracts path data from SVG XML bytes
/// Supports both <path> elements and <circle> elements
fn extract_path_from_svg_bytes(svg_bytes: &[u8]) -> Result<String> {
    let svg_str = std::str::from_utf8(svg_bytes)?;

    // First, try to find path elements
    if let Some(path_start) = svg_str.find("<path") {
        if let Some(d_start) = svg_str[path_start..].find("d=\"") {
            let d_start = path_start + d_start + 3;
            if let Some(d_end) = svg_str[d_start..].find("\"") {
                let path_data = &svg_str[d_start..d_start + d_end];
                return Ok(path_data.to_string());
            }
        }
        if let Some(d_start) = svg_str[path_start..].find("d='") {
            let d_start = path_start + d_start + 3;
            if let Some(d_end) = svg_str[d_start..].find("'") {
                let path_data = &svg_str[d_start..d_start + d_end];
                return Ok(path_data.to_string());
            }
        }
    }

    // If no path found, try to extract circles and convert them to paths
    let mut circles = Vec::new();
    let mut search_start = 0;
    
    while let Some(circle_start) = svg_str[search_start..].find("<circle") {
        let absolute_start = search_start + circle_start;
        if let Some(circle_end) = svg_str[absolute_start..].find("/>") {
            let circle_str = &svg_str[absolute_start..absolute_start + circle_end + 2];
            if let Some((cx, cy, r)) = extract_circle_attributes(circle_str) {
                circles.push((cx, cy, r));
            }
            search_start = absolute_start + circle_end + 2;
        } else if let Some(circle_end) = svg_str[absolute_start..].find("</circle>") {
            let circle_str = &svg_str[absolute_start..absolute_start + circle_end];
            if let Some((cx, cy, r)) = extract_circle_attributes(circle_str) {
                circles.push((cx, cy, r));
            }
            search_start = absolute_start + circle_end + 9;
        } else {
            break;
        }
    }

    if !circles.is_empty() {
        // Sort circles by radius (largest first)
        circles.sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap_or(std::cmp::Ordering::Equal));
        
        // Use the largest circle as the outer boundary for nesting
        // Smaller circles are part of the design but we nest based on the outer shape
        let (cx, cy, r) = circles[0];
        let outer_path = circle_to_path(cx, cy, r);
        
        // Add smaller circles as holes if they are contained within the largest circle
        let mut combined_path = outer_path;
        for (hole_cx, hole_cy, hole_r) in circles.iter().skip(1) {
            let distance = ((cx - hole_cx).powi(2) + (cy - hole_cy).powi(2)).sqrt();
            // Check if the smaller circle is completely inside the larger one
            if distance + hole_r <= r {
                // Add as a new sub-path starting with M
                let hole_path = circle_to_path(*hole_cx, *hole_cy, *hole_r);
                combined_path.push_str(" ");
                combined_path.push_str(&hole_path);
            }
        }
        
        return Ok(combined_path);
    }

    anyhow::bail!("Could not find path data or circles in SVG bytes");
}

/// Parses a single sub-path from SVG path tokens
/// Returns (points, next_index) where next_index is the index to continue parsing from
fn parse_subpath(tokens: &[&str], start_idx: usize) -> Result<(Vec<Point>, usize)> {
    let mut points = Vec::new();
    let mut current_x = 0.0;
    let mut current_y = 0.0;
    let mut start_x = 0.0;
    let mut start_y = 0.0;
    let mut in_path = false;
    let mut i = start_idx;

    while i < tokens.len() {
        let token = tokens[i];

        match token {
            "M" | "m" => {
                if i + 1 < tokens.len() {
                    let coord_str = tokens[i + 1];
                    let (x, y, consumed) = parse_coordinate_pair(coord_str, i + 1, tokens)?;
                    if token == "M" {
                        current_x = x;
                        current_y = y;
                    } else {
                        current_x += x;
                        current_y += y;
                    }
                    start_x = current_x;
                    start_y = current_y;
                    points.push(Point(current_x, current_y));
                    in_path = true;
                    i += 1 + consumed;
                } else {
                    i += 1;
                }
            }
            "L" | "l" => {
                if i + 1 < tokens.len() {
                    let coord_str = tokens[i + 1];
                    let (x, y, consumed) = parse_coordinate_pair(coord_str, i + 1, tokens)?;
                    if token == "L" {
                        current_x = x;
                        current_y = y;
                    } else {
                        current_x += x;
                        current_y += y;
                    }
                    points.push(Point(current_x, current_y));
                    i += 1 + consumed;
                } else {
                    i += 1;
                }
            }
            "z" | "Z" => {
                if !points.is_empty() {
                    if points[0] != points[points.len() - 1] {
                        points.push(Point(start_x, start_y));
                    }
                }
                i += 1;
                break; // End of this sub-path
            }
            _ => {
                // Check if this is a new sub-path starting with "M" or "m"
                if token == "M" || token == "m" {
                    break; // New sub-path starts, return current points
                }

                let parts: Vec<&str> = token.split(',').collect();
                if parts.len() == 2 {
                    if let (Ok(x), Ok(y)) = (parts[0].parse::<f32>(), parts[1].parse::<f32>()) {
                        current_x = x;
                        current_y = y;
                        if !in_path {
                            start_x = current_x;
                            start_y = current_y;
                            in_path = true;
                        }
                        points.push(Point(current_x, current_y));
                        i += 1;
                    } else {
                        i += 1;
                    }
                } else if let Ok(x) = token.trim_end_matches(',').parse::<f32>() {
                    if i + 1 < tokens.len() {
                        if let Ok(y) = tokens[i + 1].trim_end_matches(',').parse::<f32>() {
                            current_x = x;
                            current_y = y;
                            if !in_path {
                                start_x = current_x;
                                start_y = current_y;
                                in_path = true;
                            }
                            points.push(Point(current_x, current_y));
                            i += 2;
                        } else {
                            i += 1;
                        }
                    } else {
                        i += 1;
                    }
                } else {
                    i += 1;
                }
            }
        }
    }

    let mut cleaned_points = Vec::new();
    for point in points {
        if cleaned_points.is_empty() || cleaned_points[cleaned_points.len() - 1] != point {
            cleaned_points.push(point);
        }
    }

    if cleaned_points.len() > 1 && cleaned_points[0] == cleaned_points[cleaned_points.len() - 1] {
        cleaned_points.pop();
    }

    Ok((cleaned_points, i))
}

/// Parses SVG path data and extracts polygon coordinates
/// The SVG path may contain multiple sub-paths (outer boundary and inner holes)
/// Returns (outer_boundary, holes) where outer_boundary is the first sub-path and holes are subsequent sub-paths
fn parse_svg_path(path_data: &str) -> Result<(Vec<Point>, Vec<Vec<Point>>)> {
    let tokens: Vec<&str> = path_data
        .split_whitespace()
        .filter(|s| !s.is_empty())
        .collect();

    let mut outer_boundary = Vec::new();
    let mut holes = Vec::new();
    let mut i = 0;

    // Parse first sub-path as outer boundary
    if i < tokens.len() && (tokens[i] == "M" || tokens[i] == "m") {
        let (points, next_idx) = parse_subpath(&tokens, i)?;
        if !points.is_empty() {
            outer_boundary = points;
        }
        i = next_idx;
    }

    // Parse remaining sub-paths as holes
    while i < tokens.len() {
        if tokens[i] == "M" || tokens[i] == "m" {
            let (points, next_idx) = parse_subpath(&tokens, i)?;
            if !points.is_empty() {
                holes.push(points);
            }
            i = next_idx;
        } else {
            i += 1;
        }
    }

    Ok((outer_boundary, holes))
}

/// Calculates the signed area of a polygon using the shoelace formula
/// Positive area = counter-clockwise, negative area = clockwise
fn calculate_signed_area(points: &[Point]) -> f32 {
    if points.len() < 3 {
        return 0.0;
    }
    let mut sigma: f32 = 0.0;
    for i in 0..points.len() {
        let j = (i + 1) % points.len();
        let (x_i, y_i) = points[i].into();
        let (x_j, y_j) = points[j].into();
        sigma += (y_i + y_j) * (x_i - x_j);
    }
    0.5 * sigma
}

/// Reverses the winding direction of a polygon
fn reverse_winding(points: &[Point]) -> Vec<Point> {
    points.iter().rev().cloned().collect()
}

/// Converts points to SVG path data
fn points_to_svg_path(points: &[Point]) -> String {
    if points.is_empty() {
        return String::new();
    }
    let mut path = format!("M {},{}", points[0].0, points[0].1);
    for point in points.iter().skip(1) {
        path.push_str(&format!(" L {},{}", point.0, point.1));
    }
    path.push_str(" z");
    path
}

/// Nest SVG parts into a bin and return the best placement
///
/// # Arguments
/// * `bin_width` - Width of the bin
/// * `bin_height` - Height of the bin
/// * `spacing` - Minimum spacing between parts
/// * `svg_part_bytes` - SVG part as bytes (should contain a path element)
/// * `amount_of_parts` - Number of parts to place
/// * `amount_of_rotations` - Number of discrete rotations to allow (0 = no rotation, 1 = 0°, 2 = 0° and 180°, etc.)
/// * `loops` - Number of optimization loops to run
/// * `placements` - Number of placement samples per loop
///
/// # Returns
/// * [`NestingResult`] containing SVG bytes and placement metadata
/// Post-processes SVG to add holes to items and adjust colors
/// - Adds holes to each item's path (with opposite winding direction)
/// - Changes item fill color to white
/// - Changes stroke color to gray
/// - Makes holes transparent
/// Note: Holes should be in the original coordinate system (same as outer boundary in item definition)
fn post_process_svg(svg_str: &str, holes: &[Vec<Point>]) -> String {
    use regex::Regex;

    let mut result = svg_str.to_string();

    // Change item fill color to white and remove fill-opacity (make fully opaque)
    let re_fill = Regex::new(r##"fill="#FFC879""##).unwrap();
    result = re_fill
        .replace_all(&result, r##"fill="white""##)
        .to_string();

    // Remove fill-opacity from item paths (make fully opaque white)
    let re_fill_opacity = Regex::new(
        r##"(<g id="item_\d+">\s*<path[^>]*fill="white")[^>]*fill-opacity="[^"]*"([^>]*/>)"##,
    )
    .unwrap();
    result = re_fill_opacity
        .replace_all(&result, r##"${1}${2}"##)
        .to_string();

    // Change stroke color to gray for item paths
    // Match stroke="black" in item paths
    let re_stroke = Regex::new(r##"(<g id="item_\d+">\s*<path[^>]*stroke=")black(")"##).unwrap();
    result = re_stroke
        .replace_all(&result, r##"${1}gray${2}"##)
        .to_string();

    // Make container/bin transparent (remove fill or set to transparent)
    // Match: <g id="container_0"><path d="..." fill="#CC824A" ... />
    let re_container_fill =
        Regex::new(r##"(<g id="container_\d+">\s*<path[^>]*fill=")#CC824A(")"##).unwrap();
    result = re_container_fill
        .replace_all(&result, r##"${1}transparent${2}"##)
        .to_string();

    // If no holes, just return with color change
    if holes.is_empty() {
        return result;
    }

    // Note: The SVG generator applies the inverse of pre_transform to the original shape,
    // so the outer boundary in the item definition is in the original coordinate system.
    // Therefore, holes should also be in the original coordinate system (no transformation needed).

    // For each item definition, add holes to the path
    // Match: <g id="item_N"><path d="PATH_DATA" ... />
    let re_item = Regex::new(r##"(<g id="item_\d+">\s*<path d=")([^"]+)(" [^>]*/>)"##).unwrap();

    let mut matches_found = 0;
    result = re_item
        .replace_all(&result, |caps: &regex::Captures| -> String {
            matches_found += 1;
            let item_start = caps.get(1).unwrap().as_str();
            let outer_path = caps.get(2).unwrap().as_str();
            let item_end = caps.get(3).unwrap().as_str();

            // Build the combined path with outer boundary and holes
            let mut combined_path = outer_path.to_string();

            // Add holes with opposite winding direction (they'll be cut out)
            // Holes are in the original coordinate system, same as the outer boundary
            for (i, hole) in holes.iter().enumerate() {
                let hole_path = points_to_svg_path(hole);
                combined_path.push_str(&format!(" {}", hole_path));
                log::debug!("  Added hole {} to item path ({} points)", i, hole.len());
            }

            format!("{}{}{}", item_start, combined_path, item_end)
        })
        .to_string();

    log::debug!("Added holes to {} item definitions", matches_found);

    result
}

/// Configuration for adaptive optimization
#[derive(Clone, Debug)]
pub struct AdaptiveConfig {
    /// Minimum number of samples per optimization run
    pub min_samples: usize,
    /// Maximum number of samples per optimization run
    pub max_samples: usize,
    /// Minimum number of loops
    pub min_loops: usize,
    /// Maximum number of loops
    pub max_loops: usize,
    /// Minimum number of placements per loop
    pub min_placements: usize,
    /// Maximum number of placements per loop
    pub max_placements: usize,
    /// Minimum local search fraction
    pub min_ls_frac: f32,
    /// Maximum local search fraction
    pub max_ls_frac: f32,
    /// Number of consecutive attempts without improvement before increasing parameters
    pub increase_after_no_improvement: usize,
    /// Maximum consecutive attempts without improvement before stopping
    pub consecutive_no_improvement_limit: usize,
    /// Maximum calculation time in seconds
    pub max_time_seconds: u64,
}

/// Result data returned after nesting SVG parts.
#[derive(Clone, Debug)]
pub struct NestingResult {
    /// Combined SVG of all pages as bytes.
    pub combined_svg: Vec<u8>,
    /// Individual page SVGs (ordered by container id).
    pub page_svgs: Vec<Vec<u8>>,
    /// Number of parts placed.
    pub parts_placed: usize,
}

impl Default for AdaptiveConfig {
    fn default() -> Self {
        Self {
            min_samples: 50,
            max_samples: 1000,
            min_loops: 1,
            max_loops: 50,
            min_placements: 10,
            max_placements: 500,
            min_ls_frac: 0.1,
            max_ls_frac: 0.5,
            increase_after_no_improvement: 5,
            consecutive_no_improvement_limit: 100,
            max_time_seconds: 600, // 10 minutes
        }
    }
}

pub fn nest_svg_parts(
    bin_width: f32,
    bin_height: f32,
    spacing: f32,
    svg_part_bytes: &[u8],
    amount_of_parts: usize,
    amount_of_rotations: usize,
    _loops: usize,
    _placements: usize,
) -> Result<NestingResult> {
    nest_svg_parts_adaptive::<fn(&[u8], usize) -> bool>(
        bin_width,
        bin_height,
        spacing,
        svg_part_bytes,
        amount_of_parts,
        amount_of_rotations,
        AdaptiveConfig::default(),
        None::<fn(&[u8], usize) -> bool>, // No callback for backward compatibility
    )
}

/// Callback function type for handling intermediate results
///
/// Parameters:
/// - `svg_bytes`: Current best placement SVG as bytes
/// - `parts_placed`: Number of parts placed in the current best solution
///
/// Returns:
/// - `true` if calculation should be canceled (user canceled)
/// - `false` if calculation should continue
pub type SubResultHandler = Box<dyn Fn(&[u8], usize) -> bool + Send + Sync>;

pub fn nest_svg_parts_adaptive<F>(
    bin_width: f32,
    bin_height: f32,
    spacing: f32,
    svg_part_bytes: &[u8],
    amount_of_parts: usize,
    amount_of_rotations: usize,
    config: AdaptiveConfig,
    sub_result_handler: Option<F>,
) -> Result<NestingResult>
where
    F: Fn(&[u8], usize) -> bool,
{
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

    let polygon = SPolygon::new(polygon_points)?;

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

    let rotation_range = if amount_of_rotations == 0 {
        RotationRange::None
    } else if amount_of_rotations == 1 {
        RotationRange::Discrete(vec![0.0])
    } else {
        let rotations: Vec<f32> = (0..amount_of_rotations)
            .map(|i| (i as f32 * 2.0 * std::f32::consts::PI) / (amount_of_rotations as f32))
            .collect();
        RotationRange::Discrete(rotations)
    };

    let item_shape = OriginalShape {
        shape: polygon,
        pre_transform,
        modify_mode: ShapeModifyMode::Inflate,
        modify_config: importer.shape_modify_config,
    };

    let mut items = Vec::new();
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

    let bin_rect = Rect::try_new(0.0, 0.0, bin_width, bin_height)?;
    let bin_polygon = SPolygon::from(bin_rect);
    let container_shape = OriginalShape {
        shape: bin_polygon,
        pre_transform: DTransformation::empty(),
        modify_mode: ShapeModifyMode::Deflate,
        modify_config: importer.shape_modify_config,
    };

    let container = Container::new(0, container_shape, vec![], cde_config.clone())?;

    let bin = Bin::new(container, 1, 0);
    let instance = BPInstance::new(items, vec![bin]);

    let mut best_solution = None;
    let mut best_items_placed = 0;
    let mut best_svg_bytes = Vec::new();
    let mut best_page_svgs: Vec<Vec<u8>> = Vec::new();

    // Start with minimal parameters
    let mut current_samples = config.min_samples;
    let mut current_loops = config.min_loops;
    let mut current_placements = config.min_placements;
    let mut current_ls_frac = config.min_ls_frac;

    let mut consecutive_no_improvement = 0;
    let mut overall_attempts = 0;
    let start_time = std::time::Instant::now();

    loop {
        // Check time limit
        if start_time.elapsed().as_secs() >= config.max_time_seconds {
            log::debug!("Time limit reached ({} seconds)", config.max_time_seconds);
            break;
        }

        // Check consecutive no improvement limit
        if consecutive_no_improvement >= config.consecutive_no_improvement_limit {
            log::debug!(
                "Consecutive no improvement limit reached ({} attempts, 0 remaining)",
                config.consecutive_no_improvement_limit
            );
            break;
        }

        // Try optimization with current parameters
        'outer: for loop_idx in 0..current_loops {
            for seed in 0..current_placements {
                overall_attempts += 1;

                // Check time limit before each attempt
                if start_time.elapsed().as_secs() >= config.max_time_seconds {
                    break 'outer;
                }

                // Check consecutive no improvement limit
                if consecutive_no_improvement >= config.consecutive_no_improvement_limit {
                    break 'outer;
                }

                let lbf_config = LBFConfig {
                    cde_config: cde_config.clone(),
                    poly_simpl_tolerance: Some(0.001),
                    min_item_separation: Some(spacing),
                    prng_seed: Some((loop_idx * current_placements + seed) as u64),
                    n_samples: current_samples,
                    ls_frac: current_ls_frac,
                    narrow_concavity_cutoff_ratio: None,
                    svg_draw_options: Default::default(),
                };

                let mut optimizer = LBFOptimizerBP::new(
                    instance.clone(),
                    lbf_config,
                    SmallRng::seed_from_u64((loop_idx * current_placements + seed) as u64),
                );

                let test_solution = optimizer.solve();

                let total_items_placed: usize = test_solution
                    .layout_snapshots
                    .values()
                    .map(|ls| ls.placed_items.len())
                    .sum();

                if total_items_placed > best_items_placed {
                    best_items_placed = total_items_placed;
                    best_solution = Some(test_solution.clone());
                    consecutive_no_improvement = 0; // Reset counter on improvement
                    let remaining_attempts = config.consecutive_no_improvement_limit - consecutive_no_improvement;

                    log::debug!(
                        "Improved to {} parts (samples: {}, loops: {}, placements: {}, ls_frac: {:.2}, attempt: {}, remaining attempts: {})",
                        total_items_placed,
                        current_samples,
                        current_loops,
                        current_placements,
                        current_ls_frac,
                        overall_attempts,
                        remaining_attempts
                    );

                    let svg_options = SvgDrawOptions::default();
                    let mut combined_svg = String::new();
                    let mut page_svgs_local: Vec<Vec<u8>> = Vec::new();

                    let mut layout_entries: Vec<_> =
                        test_solution.layout_snapshots.iter().collect();
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
                        combined_svg.push_str(&processed_svg);
                        page_svgs_local.push(processed_svg.into_bytes());
                    }
                    best_svg_bytes = combined_svg.into_bytes();
                    best_page_svgs = page_svgs_local;

                    // Call sub_result_handler if provided
                    if let Some(ref handler) = sub_result_handler {
                        let should_cancel = handler(&best_svg_bytes, best_items_placed);
                        if should_cancel {
                            log::debug!(
                                "Calculation canceled by user after {} parts placed",
                                best_items_placed
                            );
                            return Ok(NestingResult {
                                combined_svg: best_svg_bytes.clone(),
                                page_svgs: best_page_svgs.clone(),
                                parts_placed: best_items_placed,
                            });
                        }
                    }

                    if total_items_placed == amount_of_parts {
                        log::debug!("All {} parts placed!", amount_of_parts);
                        return Ok(NestingResult {
                            combined_svg: best_svg_bytes.clone(),
                            page_svgs: best_page_svgs.clone(),
                            parts_placed: best_items_placed,
                        });
                    }
                } else {
                    // No improvement in this attempt
                    consecutive_no_improvement += 1;
                    let remaining_attempts = config.consecutive_no_improvement_limit.saturating_sub(consecutive_no_improvement);

                    // Increase parameters if no improvement for increase_after_no_improvement attempts
                    if consecutive_no_improvement >= config.increase_after_no_improvement {
                        // Increase parameters gradually
                        if current_samples < config.max_samples {
                            current_samples = (current_samples * 2).min(config.max_samples);
                        }
                        if current_loops < config.max_loops {
                            current_loops = (current_loops + 1).min(config.max_loops);
                        }
                        if current_placements < config.max_placements {
                            current_placements =
                                (current_placements * 2).min(config.max_placements);
                        }
                        if current_ls_frac < config.max_ls_frac {
                            current_ls_frac = (current_ls_frac + 0.05).min(config.max_ls_frac);
                        }

                        log::debug!(
                            "No improvement for {} attempts, increasing parameters (samples: {}, loops: {}, placements: {}, ls_frac: {:.2}, remaining attempts: {})",
                            consecutive_no_improvement,
                            current_samples,
                            current_loops,
                            current_placements,
                            current_ls_frac,
                            remaining_attempts
                        );

                        // Reset counter after increasing parameters
                        consecutive_no_improvement = 0;
                    }
                }
            }
        }

        let remaining_attempts = config.consecutive_no_improvement_limit.saturating_sub(consecutive_no_improvement);
        log::info!(
            "Attempt {}: best={}, consecutive_no_improvement={}, remaining attempts={}, params: samples={}, loops={}, placements={}, ls_frac={:.2}",
            overall_attempts,
            best_items_placed,
            consecutive_no_improvement,
            remaining_attempts,
            current_samples,
            current_loops,
            current_placements,
            current_ls_frac
        );
    }

    log::debug!(
        "Optimization complete: {} parts placed in {} attempts over {:.2} seconds",
        best_items_placed,
        overall_attempts,
        start_time.elapsed().as_secs_f64()
    );

    Ok(NestingResult {
        combined_svg: best_svg_bytes,
        page_svgs: best_page_svgs,
        parts_placed: best_items_placed,
    })
}
