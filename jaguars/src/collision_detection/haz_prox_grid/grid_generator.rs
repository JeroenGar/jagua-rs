use std::cmp::Ordering;
use log::{debug};

use crate::collision_detection::hazard::Hazard;
use crate::geometry::geo_traits::{DistanceFrom, Shape};
use crate::geometry::primitives::aa_rectangle::AARectangle;
use crate::geometry::primitives::point::Point;

/// Generates a grid of equal sized square rectangles within a shape.
/// The number of cells is approximately equal to target_n_cells, but can be slightly more or less
pub fn generate(bbox: AARectangle, hazards: &[Hazard], target_n_cells: usize) -> Vec<AARectangle> {
    assert!(bbox.area() > 0.0, "bbox has zero area");

    let mut cells = vec![];

    let mut correction_factor = 1.0;
    let mut step_size = 0.1;
    let mut n_iters = 0;
    let mut previous_attempt = None;

    loop {
        let cell_dim = f64::sqrt(bbox.area() / target_n_cells as f64) * correction_factor; //square cells
        let n_cells_in_x = f64::ceil(bbox.width() / cell_dim) as usize;
        let n_cells_in_y = f64::ceil(bbox.height() / cell_dim) as usize;
        let cell_radius = f64::sqrt(2.0 * (cell_dim / 2.0).powi(2)); //half of the maximum distance between two cell centers

        for i in 0..n_cells_in_x {
            let x_min = bbox.x_min + cell_dim * i as f64;
            let x_max = x_min + cell_dim;
            for j in 0..n_cells_in_y {
                let y_min = bbox.y_min + cell_dim * j as f64;
                let y_max = y_min + cell_dim;
                let rect = AARectangle::new(x_min, y_min, x_max, y_max);
                //test if the cell is relevant
                let distance_to_hazard = distance_to_hazard(&rect.centroid(), hazards.iter());
                if distance_to_hazard + cell_radius > 0.0 {
                    cells.push(rect);
                }
            }
        }
        if n_iters >= 25 {
            debug!("grid generation is taking too long, stopping after 25 iterations ({} cells instead of target {})", cells.len(), target_n_cells);
            break;
        }

        let attempt = cells.len().cmp(&target_n_cells);

        if Some(attempt) != previous_attempt {
            //we are going in the wrong direction, so decrease the step size
            step_size /= 2.0;
        }

        match attempt {
            //Close enough
            Ordering::Equal => break,
            //not enough cells, decrease their size
            Ordering::Less => {
                correction_factor -= step_size;
                cells.clear();
            }
            //too many cells, increase their size
            Ordering::Greater => {
                correction_factor += step_size;
                cells.clear();
            }
        }

        previous_attempt = Some(attempt);
        n_iters += 1;
    }
    cells
}

fn distance_to_hazard<'a, I>(point: &Point, hazards: I) -> f64 where I: Iterator<Item=&'a Hazard> {
    hazards.map(|haz| {
        let (pos, prox) = haz.shape.distance_from_border(point);
        match pos == haz.entity.position() {
            true => -prox, //cell in hazard, negative distance
            false => prox
        }
    }).min_by(|a, b| a.partial_cmp(b).expect("NaN in distance_to_hazard"))
        .unwrap_or(f64::MIN)
}