//! SVG parsing utilities for extracting geometry from SVG files

use anyhow::Result;
use jagua_rs::geometry::primitives::Point;

/// Helper function to parse a coordinate pair from SVG path tokens
/// Handles both "x,y" format and "x y" format
/// Returns (x, y, tokens_to_advance) where tokens_to_advance is the number of tokens to advance
/// (not including the current command token)
pub fn parse_coordinate_pair(
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
pub fn circle_to_path(cx: f32, cy: f32, r: f32) -> String {
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
pub fn extract_circle_attributes(circle_str: &str) -> Option<(f32, f32, f32)> {
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
pub fn extract_path_from_svg_bytes(svg_bytes: &[u8]) -> Result<String> {
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
pub fn parse_subpath(tokens: &[&str], start_idx: usize) -> Result<(Vec<Point>, usize)> {
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
pub fn parse_svg_path(path_data: &str) -> Result<(Vec<Point>, Vec<Vec<Point>>)> {
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
pub fn calculate_signed_area(points: &[Point]) -> f32 {
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
pub fn reverse_winding(points: &[Point]) -> Vec<Point> {
    points.iter().rev().cloned().collect()
}
