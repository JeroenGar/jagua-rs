use std::cmp::Ordering;

use crate::collision_detection::hazards::hazard::Hazard;
use crate::geometry::geo_traits::{DistanceFrom, Shape};
use crate::geometry::primitives::aa_rectangle::AARectangle;
use crate::geometry::primitives::point::Point;

pub fn generate(bbox: AARectangle, hazards: &[Hazard], target_n_cells: usize) -> Vec<AARectangle> {
    //generates a grid of equal sized square cells in the shape.
    //the number of cells is approximately equal to target_n_cells, but can be slightly more or less

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
            let x_min = bbox.x_min() + cell_dim * i as f64;
            let x_max = x_min + cell_dim;
            for j in 0..n_cells_in_y {
                let y_min = bbox.y_min() + cell_dim * j as f64;
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
            //warn!("grid generation is taking too long, aborting after 100 iterations ({} cells, instead of {})", cells.len(), target_n_cells);
            break;
        }

        let attempt = cells.len().cmp(&target_n_cells);

        if Some(attempt) != previous_attempt {
            //we are going in the wrong direction, so decrease the step size
            step_size /= 2.0;
        }

        match attempt {
            Ordering::Equal => {
                //just right
                break;
            }
            Ordering::Less => {
                //not enough cells, decrease their size
                correction_factor -= step_size;
                cells.clear();
            }
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
        let (pos, prox) = haz.shape().distance_from_border(point);
        match pos == haz.entity().presence() {
            true => -prox, //cell in hazard, negative distance
            false => prox
        }
    }).min_by(|a, b| a.partial_cmp(b).expect("NaN in distance_to_hazard"))
        .unwrap_or(f64::MIN)
}