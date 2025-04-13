use std::cmp::Ordering;

use itertools::Itertools;
use ordered_float::NotNan;

use crate::collision_detection::hazard::Hazard;
use crate::collision_detection::hazard::HazardEntity;
use crate::entities::general::bin::N_QUALITIES;
use crate::entities::general::item::Item;
use crate::fsize;
use crate::geometry::geo_enums::GeoPosition;
use crate::geometry::geo_traits::{SeparationDistance, Shape};
use crate::geometry::primitives::aa_rectangle::AARectangle;
use crate::geometry::primitives::circle::Circle;
use crate::geometry::primitives::point::Point;

/// Represents a cell in the Hazard Proximity Grid
#[derive(Clone, Debug)]
pub struct HPGCell {
    pub bbox: AARectangle,
    pub centroid: Point,
    pub radius: fsize,
    ///Proximity of closest hazard which is universally applicable (bin or item), zero if inside
    pub uni_prox: (fsize, HazardEntity),
    ///Proximity of universal static hazards, zero if inside
    pub static_uni_prox: (fsize, HazardEntity),
    ///proximity of closest quality zone for each quality, zero if inside
    pub qz_prox: [fsize; N_QUALITIES],
}

impl HPGCell {
    pub fn new(bbox: AARectangle, static_hazards: &[Hazard]) -> Self {
        //Calculate the exact distance to the edge bin (add new method in shape trait to do this)
        //For each of the distinct quality zones in a bin, calculate the distance to the closest zone
        let centroid = bbox.centroid();
        let radius = bbox.diameter() / 2.0;

        let mut static_uni_prox = (fsize::MAX, HazardEntity::BinExterior);
        let mut qz_prox = [fsize::MAX; N_QUALITIES];

        for hazard in static_hazards {
            let (pos, distance) = hazard.shape.separation_distance(&centroid);
            let prox = match pos == hazard.entity.position() {
                true => 0.0, //cells centroid is inside the hazard
                false => distance,
            };
            match &hazard.entity {
                HazardEntity::BinExterior | HazardEntity::BinHole { .. } => {
                    if prox < static_uni_prox.0 {
                        static_uni_prox = (prox, hazard.entity);
                    }
                }
                HazardEntity::InferiorQualityZone { quality, .. } => {
                    qz_prox[*quality] = qz_prox[*quality].min(prox);
                }
                _ => panic!("Unexpected hazard entity type"),
            }
        }

        Self {
            bbox,
            centroid,
            radius,
            uni_prox: static_uni_prox,
            static_uni_prox,
            qz_prox,
        }
    }

    pub fn register_hazards<'a, I>(&mut self, to_register: I)
    where
        I: Iterator<Item = &'a Hazard>,
    {
        //For each item to register, calculate the distance from the cell to its bounding circle of the poles.
        //This serves as a lower-bound for the distance to the item itself.
        let mut bounding_pole_distances: Vec<(&Hazard, Option<fsize>)> = to_register
            .filter(|haz| haz.active)
            .map(|haz| {
                match haz.entity.position() {
                    GeoPosition::Exterior => (haz, None), //bounding poles only applicable for hazard inside the shape
                    GeoPosition::Interior => {
                        let pole_bounding_circle = &haz.shape.surrogate().poles_bounding_circle;
                        let proximity = pole_bounding_circle.separation_distance(&self.centroid);
                        match proximity {
                            (GeoPosition::Interior, _) => (haz, Some(0.0)),
                            (GeoPosition::Exterior, dist) => (haz, Some(dist.abs())),
                        }
                    }
                }
            })
            .collect();

        //Go over the items in order of the closest bounding circle
        while !bounding_pole_distances.is_empty() {
            let (index, (to_register, bounding_proximity)) = bounding_pole_distances
                .iter()
                .enumerate()
                .min_by_key(|(_, (_, d))| d.map(|d| NotNan::new(d).expect("distance was NaN")))
                .unwrap();

            let current_proximity = self.uni_prox.0;

            match bounding_proximity {
                None => {
                    self.register_hazard(to_register);
                    bounding_pole_distances.swap_remove(index);
                }
                Some(bounding_prox) => {
                    if bounding_prox <= &current_proximity {
                        //bounding circle is closer than current closest hazard, potentially affecting this cell
                        self.register_hazard(to_register);
                        bounding_pole_distances.swap_remove(index);
                    } else {
                        //bounding circle is further away than current closest.
                        //This, and all following items (which are further away) do not modify this cell
                        break;
                    }
                }
            }
        }
    }

    pub fn register_hazard(&mut self, to_register: &Hazard) -> HPGCellUpdate {
        debug_assert!(
            to_register.entity.is_universal(),
            "no support for dynamic non-universal hazards at this time"
        );
        let current_prox = self.uni_prox.0;

        //For dynamic hazards, the surrogate poles are used to calculate the distance to the hazard (overestimation, but fast)
        let haz_prox = match to_register.entity.position() {
            GeoPosition::Interior => {
                distance_to_surrogate_poles_border(self, &to_register.shape.surrogate().poles)
            }
            GeoPosition::Exterior => {
                panic!("No implementation yet for dynamic exterior hazards")
            }
        };

        match haz_prox.partial_cmp(&current_prox).unwrap() {
            Ordering::Less => {
                //new hazard is closer
                self.uni_prox = (haz_prox, to_register.entity);
                HPGCellUpdate::Affected
            }
            _ => {
                if haz_prox > current_prox + 2.0 * self.radius {
                    HPGCellUpdate::NeighborsNotAffected
                } else {
                    HPGCellUpdate::NotAffected
                }
            }
        }
    }

    pub fn register_hazard_pole(&mut self, to_register: &Hazard, pole: &Circle) -> HPGCellUpdate {
        debug_assert!(
            to_register.entity.is_universal(),
            "no support for dynamic non-universal hazards at this time"
        );
        let current_prox = self.uni_prox.0;

        //For dynamic hazards, the surrogate poles are used to calculate the distance to the hazard (overestimation, but fast)
        let new_prox = match to_register.entity.position() {
            GeoPosition::Interior => match pole.separation_distance(&self.centroid) {
                (GeoPosition::Interior, _) => 0.0,
                (GeoPosition::Exterior, dist) => dist.abs(),
            },
            GeoPosition::Exterior => {
                panic!("No implementation yet for dynamic exterior hazards")
            }
        };

        match new_prox.partial_cmp(&current_prox).unwrap() {
            Ordering::Less => {
                //new hazard is closer
                self.uni_prox = (new_prox, to_register.entity);
                HPGCellUpdate::Affected
            }
            _ => {
                //The current cell is unaffected, but its neighbors might be
                //maximum distance between neighboring cells
                let max_neighbor_distance = 2.0 * self.radius;

                let haz_prox_lower_bound = new_prox - max_neighbor_distance;
                let current_prox_upper_bound = current_prox + max_neighbor_distance;

                match haz_prox_lower_bound > current_prox_upper_bound {
                    //this cell is unaffected, but no guarantees about its neighbors
                    false => HPGCellUpdate::NotAffected,
                    //Current hazard will always be closer, we can guarantee that the neighbors will also be unaffected
                    true => HPGCellUpdate::NeighborsNotAffected,
                }
            }
        }
    }

    pub fn deregister_hazards<'a, I, J>(
        &mut self,
        mut to_deregister: J,
        remaining: I,
    ) -> HPGCellUpdate
    where
        I: Iterator<Item = &'a Hazard>,
        J: Iterator<Item = HazardEntity>,
    {
        if to_deregister.contains(&self.uni_prox.1) {
            //closest current hazard has to be deregistered
            self.uni_prox = self.static_uni_prox;

            self.register_hazards(remaining);
            HPGCellUpdate::Affected
        } else {
            HPGCellUpdate::NotAffected
        }
    }

    pub fn could_accommodate_item(&self, item: &Item) -> bool {
        let poi_d = item.shape.poi.radius;
        if self.radius > poi_d {
            //impossible to give any guarantees if the cell radius is larger than the Item's POI
            true
        } else {
            //distance of closest relevant hazard
            let haz_prox = self.hazard_proximity(item.base_quality);

            poi_d < haz_prox + self.radius
        }
    }

    pub fn hazard_proximity(&self, quality_level: Option<usize>) -> fsize {
        //calculate the minimum distance to either bin, item or qz
        let mut haz_prox = self.uni_prox.0;
        let relevant_qualities = match quality_level {
            Some(quality_level) => 0..quality_level,
            None => 0..N_QUALITIES,
        };

        for quality in relevant_qualities {
            haz_prox = haz_prox.min(self.qz_prox[quality]);
        }
        haz_prox
    }
}

pub fn distance_to_surrogate_poles_border(hp_cell: &HPGCell, poles: &[Circle]) -> fsize {
    poles
        .iter()
        .map(|p| p.separation_distance(&hp_cell.centroid))
        .map(|(pos, dist)| match pos {
            GeoPosition::Interior => 0.0,
            GeoPosition::Exterior => dist.abs(),
        })
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap()
}

///All possible results of an update on a cell in the `HazardProximityGrid`
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum HPGCellUpdate {
    ///Update affected the cell
    Affected,
    ///Update did not affect the cell, but its neighbors can be affected
    NotAffected,
    ///Update did not affect the cell and its neighbors are also guaranteed to be unaffected
    NeighborsNotAffected,
}
