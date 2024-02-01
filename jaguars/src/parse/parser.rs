use std::sync::Arc;
use std::time::Instant;

use itertools::Itertools;
use log::{Level, log, warn};

use crate::entities::bin::Bin;
use crate::entities::insertion_option::InsertionOption;
use crate::entities::instance::{Instance, PackingType};
use crate::entities::item::Item;
use crate::entities::problems::bp_problem::BPProblem;
use crate::entities::problems::problem::{LayoutIndex, Problem, ProblemEnum};
use crate::entities::problems::sp_problem::SPProblem;
use crate::entities::quality_zone::QualityZone;
use crate::entities::solution::Solution;
use crate::geometry::d_transformation::DTransformation;
use crate::geometry::geo_traits::{Shape, Transformable};
use crate::geometry::primitives::point::Point;
use crate::geometry::primitives::simple_polygon::SimplePolygon;
use crate::geometry::rotation::Rotation;
use crate::geometry::transformation::Transformation;
use crate::N_QUALITIES;
use crate::parse::json::json_instance::{JsonInstance, JsonPoly, JsonSimplePoly};
use crate::parse::json::json_solution::{JsonLayout, JsonLayoutStats, JsonObjectType, JsonPlacedItem, JsonSolution, JsonTransformation};
use crate::simplification::{poly_simplification, polygon_converter};
use crate::simplification::simplification_config::{PolySimplConfig, PolySimplMode};
use crate::util::config::CDEConfig;

pub struct Parser {
    poly_simpl_config: PolySimplConfig,
    cde_config: CDEConfig,
    center_polygons: bool,
}

impl Parser {
    pub fn new(poly_simpl_config: PolySimplConfig, cde_config: CDEConfig, center_polygons: bool) -> Parser {
        Parser {poly_simpl_config, cde_config, center_polygons }
    }

    pub fn parse(&self, json_instance: &JsonInstance) -> Instance {
        let mut items: Vec<(Item, usize)> = vec![];
        let mut packing_type = None;

        crossbeam::thread::scope(|s| {
            let mut item_join_handles = vec![];
            for (item_id, json_item) in json_instance.items.iter().enumerate() {
                let handle = s.spawn(move |_| {
                    let (shape, centering_transf) = json_shape_to_simple_polygon(
                        &json_item.shape,
                        self.center_polygons,
                        self.poly_simpl_config,
                        PolySimplMode::Inflate,
                    );
                    let item_value = json_item.value.unwrap_or(0);
                    let base_quality = json_item.base_quality;

                    match json_item.zones.as_ref() {
                        Some(json_zones) => {
                            if !json_zones.is_empty() {
                                warn!("Quality zones for items are not supported yet, ignoring them");
                            }
                            /*let different_qualities = json_zones.values().map(|zone| zone.quality).unique().collect::<Vec<usize>>();

                            for quality in different_qualities {
                                let zones = json_zones.values()
                                    .filter(|zone| zone.quality == quality)
                                    .map(|zone| {
                                        let (zone_shape, _) = json_shape_to_polygon(
                                            &zone.shape,
                                            false,
                                            self.config.poly_simplification_config,
                                            SimplificationMode::Deflate,
                                        );
                                        zone_shape.transform_clone(&centering_transf)
                                    })
                                    .collect::<Vec<Polygon>>();
                                let qz = QualityZone::new(quality, zones);
                                assert!((quality as usize) < N_QUALITIES, "Quality {} is out of range, (configure N_QUALITIES const higher)", quality);
                                quality_zones[quality as usize] = Some(qz);
                            }*/
                        }
                        None => {}
                    };
                    let allowed_orientations = match json_item.allowed_orientations.as_ref() {
                        Some(a_o) => {
                            if a_o.is_empty() || (a_o.len() == 1 && a_o[0] == 0.0) {
                                Rotation::None
                            }
                            else{
                                Rotation::Discrete(a_o.iter().map(|angle| angle.to_radians()).collect())
                            }
                        },
                        None => Rotation::Continuous,
                    };

                    (Item::new(item_id, shape, item_value, allowed_orientations, centering_transf, base_quality, self.cde_config.item_surrogate_config.clone()), json_item.demand as usize)
                });
                item_join_handles.push(handle);
            }
            for join_handle in item_join_handles {
                items.push(join_handle.join().unwrap());
            }

            match (json_instance.bins.as_ref(), json_instance.strip.as_ref()) {
                (Some(json_bins), None) => {
                    let mut bins: Vec<(Bin, usize)> = vec![];
                    let mut bin_join_handles = vec![];
                    for (bin_id, json_bin) in json_bins.iter().enumerate() {
                        let handle = s.spawn(move |_| {
                            let (bin_outer, centering_transf) = simple_json_shape_to_simple_polygon(
                                &json_bin.shape.outer,
                                self.center_polygons,
                                self.poly_simpl_config,
                                PolySimplMode::Deflate,
                            );

                            let bin_holes = match json_bin.shape.inner.as_ref() {
                                Some(json_holes) => {
                                    json_holes.iter().map(|jsp| {
                                        let (hole, _) = simple_json_shape_to_simple_polygon(
                                            jsp,
                                            false,
                                            self.poly_simpl_config,
                                            PolySimplMode::Inflate,
                                        );
                                        hole.transform_clone(&centering_transf)
                                    }).collect_vec()
                                }
                                None => vec![]
                            };

                            let material_value = (bin_outer.area() - bin_holes.iter().map(|hole| hole.area()).sum::<f64>()) as u64;

                            let mut quality_zones = vec![];

                            match json_bin.zones.as_ref() {
                                Some(json_zones) => {
                                    let different_qualities = json_zones.values().map(|zone| zone.quality).unique().collect::<Vec<usize>>();


                                    for quality in different_qualities {
                                        let zones = json_zones.values()
                                            .filter(|zone| zone.quality == quality)
                                            .map(|zone| {
                                                let (zone_shape, _) = json_shape_to_simple_polygon(
                                                    &zone.shape,
                                                    false,
                                                    self.poly_simpl_config,
                                                    PolySimplMode::Inflate,
                                                );
                                                zone_shape.transform_clone(&centering_transf)
                                            })
                                            .collect_vec();
                                        let qz = QualityZone::new(quality, zones);
                                        assert!((quality as usize) < N_QUALITIES, "Quality {} is out of range, (set N_QUALITIES const higher)", quality);
                                        quality_zones.push(qz);
                                    }
                                }
                                None => {}
                            };
                            let bin = Bin::new(
                                bin_id,
                                bin_outer,
                                material_value,
                                centering_transf,
                                bin_holes,
                                quality_zones,
                                self.cde_config,
                            );

                            (bin, json_bin.stock as usize)
                        });
                        bin_join_handles.push(handle);
                    }
                    for join_handle in bin_join_handles {
                        bins.push(join_handle.join().unwrap());
                    }
                    packing_type = Some(PackingType::BinPacking(bins));
                }
                (None, Some(json_strip)) => {
                    packing_type = Some(PackingType::StripPacking { height: json_strip.height });
                }
                (Some(_), Some(_)) => panic!("Both bins and strip packing specified"),
                (None, None) => panic!("No bins or strip packing specified")
            }
        }).unwrap();

        let instance = Instance::new(items, packing_type.unwrap());

        log!(Level::Info, "instance parsed: {} items ({} unique)",
        instance.total_item_qty(),
        instance.items().len()
    );

        match instance.packing_type() {
            PackingType::StripPacking { height } => {
                log!(Level::Info, "strip packing instance parsed: {} items ({} unique), {} strip height",
                instance.total_item_qty(),
                instance.items().len(),
                height
            );
            }
            PackingType::BinPacking(bins) => {
                log!(Level::Info, "bin packing, {} items ({} unique), {} bins ({} unique)",
                instance.total_item_qty(),
                instance.items().len(),
                bins.iter().map(|(_,qty)| *qty).sum::<usize>(),
                bins.len());
            }
        };

        log!(Level::Info, "total area of items: {}", instance.items().iter().map(|(item,qty)| item.shape().area() * *qty as f64).sum::<f64>());

        instance
    }

    pub fn parse_and_build_solution(&self, json_instance: &JsonInstance, json_layouts: &Vec<JsonLayout>) -> (Instance, Solution) {
        let instance = Arc::new(self.parse(json_instance));
        let solution = build_solution_from_json(&json_layouts, instance.clone(), self.cde_config);
        let instance = Arc::try_unwrap(instance).expect("Cannot unwrap instance, strong references present");
        (instance, solution)
    }
}

fn build_solution_from_json(json_layouts: &[JsonLayout], instance: Arc<Instance>, cde_config: CDEConfig) -> Solution {
    let mut problem: ProblemEnum = match instance.packing_type() {
        PackingType::BinPacking(_) => ProblemEnum::BPProblem(BPProblem::new(instance.clone())),
        PackingType::StripPacking { .. } => {
            assert_eq!(json_layouts.len(), 1);
            match json_layouts[0].object_type {
                JsonObjectType::Object { .. } => panic!("Strip packing solution should not contain layouts with references to an Object"),
                JsonObjectType::Strip { width, height: _ } => {
                    ProblemEnum::SPProblem(SPProblem::new(instance.clone(), width, cde_config))
                }
            }
        }
    };

    for json_layout in json_layouts {
        let bin = match (instance.packing_type(), &json_layout.object_type) {
            (PackingType::BinPacking(bins), JsonObjectType::Object { id }) => Some(&bins[*id].0),
            (PackingType::StripPacking { .. }, JsonObjectType::Strip { .. }) => None,
            _ => panic!("Layout object type does not match packing type")
        };
        //Create the layout by inserting the first item
        let (empty_layout_index, _empty_layout) = problem.empty_layouts().iter().enumerate()
            .find(|(_, layout)| layout.bin().id() == bin.map_or(0, |b| b.id())).unwrap();

        let bin_centering = bin.map_or(DTransformation::empty(), |b| DTransformation::from(b.centering_transform())).translation();

        let json_first_item = json_layout.placed_items.get(0).unwrap();
        let first_item = instance.item(json_first_item.item_index);

        let first_item_centering_correction = first_item.centering_transform().clone().inverse().decompose().translation();

        let transformation = Transformation::empty()
            .translate(first_item_centering_correction)
            .rotate(json_first_item.transformation.rotation)
            .translate(json_first_item.transformation.translation)
            .translate(bin_centering);

        let d_transform = transformation.decompose();

        let initial_insert_opt = InsertionOption::new(
            LayoutIndex::Empty(empty_layout_index),
            first_item.id(),
            transformation,
            d_transform,
        );
        problem.insert_item(&initial_insert_opt);
        problem.flush_changes();

        //TODO: assuming layouts are always added to the back of the vector is not very robust
        let layout_index = problem.layouts().len() - 1;

        for json_item in json_layout.placed_items.iter().skip(1) {
            let item = instance.item(json_item.item_index);
            let item_centering_correction = item.centering_transform().clone().inverse().decompose().translation();
            let transformation = Transformation::empty()
                .translate(item_centering_correction)
                .rotate(json_item.transformation.rotation)
                .translate(json_item.transformation.translation)
                .translate(bin_centering);

            let d_transform = transformation.decompose();

            let insert_opt = InsertionOption::new(
                LayoutIndex::Existing(layout_index),
                item.id(),
                transformation,
                d_transform,
            );
            problem.insert_item(&insert_opt);
            problem.flush_changes();
        }
    }

    problem.create_solution(&None)
}

pub fn compose_json_solution(solution: &Solution, instance: &Instance, epoch: Instant) -> JsonSolution {
    let layouts = solution.stored_layouts().iter()
        .map(|sl| {
            let object_type = match instance.packing_type() {
                PackingType::BinPacking(..) => JsonObjectType::Object { id: sl.bin().id() },
                PackingType::StripPacking { height } => JsonObjectType::Strip { width: sl.bin().bbox().width(), height: *height },
            };
            let bin_centering_correction = match instance.packing_type() {
                PackingType::StripPacking { .. } => (0.0, 0.0),
                PackingType::BinPacking(bins) => bins[sl.bin().id()].0.centering_transform().clone().inverse().decompose().translation(),
            };

            let placed_items = sl.placed_items().iter()
                .map(|pl| {
                    let item_index = pl.item_id();
                    let item = instance.item(item_index);
                    let item_centering = item.centering_transform().decompose().translation();

                    let pl_decomp_transf = pl.d_transformation();

                    //Both bins and items have centering transformations, however in the output file, we need to restore them to the original positions

                    let transformation = Transformation::empty()
                        .translate(item_centering)
                        .rotate(pl_decomp_transf.rotation())
                        .translate(pl_decomp_transf.translation())
                        .translate(bin_centering_correction);

                    let decomp_transf = transformation.decompose();
                    let json_transform = JsonTransformation {
                        rotation: decomp_transf.rotation(),
                        translation: decomp_transf.translation(),
                    };
                    JsonPlacedItem {
                        item_index,
                        transformation: json_transform,
                    }
                }).collect::<Vec<JsonPlacedItem>>();
            let statistics = JsonLayoutStats {
                usage: sl.usage(),
            };
            JsonLayout {
                object_type,
                placed_items,
                statistics,
            }
        }).collect::<Vec<JsonLayout>>();

    JsonSolution {
        layouts,
        usage: solution.usage(),
        run_time_sec: solution.time_stamp().duration_since(epoch).as_secs(),
    }
}

fn json_shape_to_simple_polygon(json_shape: &JsonPoly, center_polygon: bool, simpl_config: PolySimplConfig, simpl_mode: PolySimplMode) -> (SimplePolygon, Transformation) {
    let outer = SimplePolygon::new(json_simple_poly_to_points(&json_shape.outer));

    let mut inners = vec![];
    if let Some(json_shape_inner) = json_shape.inner.as_ref() {
        for jp_vec in json_shape_inner {
            let shape = SimplePolygon::new(json_simple_poly_to_points(jp_vec));
            inners.push(shape);
        }
    }

    let shape = match inners.is_empty() {
        true => outer,
        false => polygon_converter::convert_to_simple_polygon(&outer, &inners)
    };

    let shape = match simpl_config {
        PolySimplConfig::Enabled { tolerance } => {
            poly_simplification::simplify_simple_poly(&shape, tolerance, simpl_mode)
        }
        PolySimplConfig::Disabled => shape
    };

    let (shape, centering_transform) = match center_polygon {
        true => polygon_converter::center_around_centroid(&shape),
        false => (shape, Transformation::empty())
    };

    (shape, centering_transform)
}

pub fn simple_json_shape_to_simple_polygon(s_json_shape: &JsonSimplePoly, center_polygon: bool, simpl_config: PolySimplConfig, simpl_mode: PolySimplMode) -> (SimplePolygon, Transformation) {
    let shape = SimplePolygon::new(json_simple_poly_to_points(s_json_shape));

    let shape = match simpl_config {
        PolySimplConfig::Enabled { tolerance } => {
            poly_simplification::simplify_simple_poly(&shape, tolerance, simpl_mode)
        }
        PolySimplConfig::Disabled => shape
    };

    let (shape, centering_transform) = match center_polygon {
        true => polygon_converter::center_around_centroid(&shape),
        false => (shape, Transformation::empty())
    };

    (shape, centering_transform)
}

fn json_simple_poly_to_points(jsp: &JsonSimplePoly) -> Vec<Point> {
    //Strip the last vertex if it is the same as the first one
    let n_vertices = match jsp.0[0] == jsp.0[jsp.0.len() - 1] {
        true => jsp.0.len() - 1,
        false => jsp.0.len()
    };

    (0..n_vertices).map(|i| Point::from(jsp.0[i])).collect_vec()
}

