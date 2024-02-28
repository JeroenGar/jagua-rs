use std::sync::Arc;
use std::time::Instant;

use itertools::Itertools;
use log::{log, Level};

use crate::entities::bin::Bin;
use crate::entities::instances::bin_packing::BPInstance;
use crate::entities::instances::instance::Instance;
use crate::entities::instances::instance_generic::InstanceGeneric;
use crate::entities::instances::strip_packing::SPInstance;
use crate::entities::item::Item;
use crate::entities::placing_option::PlacingOption;
use crate::entities::problems::bin_packing::BPProblem;
use crate::entities::problems::problem::Problem;
use crate::entities::problems::problem_generic::{LayoutIndex, ProblemGeneric};
use crate::entities::problems::strip_packing::SPProblem;
use crate::entities::quality_zone::QualityZone;
use crate::entities::quality_zone::N_QUALITIES;
use crate::entities::solution::Solution;
use crate::geometry::d_transformation::DTransformation;
use crate::geometry::geo_enums::AllowedRotation;
use crate::geometry::geo_traits::{Shape, Transformable};
use crate::geometry::primitives::aa_rectangle::AARectangle;
use crate::geometry::primitives::point::Point;
use crate::geometry::primitives::simple_polygon::SimplePolygon;
use crate::geometry::transformation::Transformation;
use crate::io::json_instance::{JsonInstance, JsonShape, JsonSimplePoly};
use crate::io::json_solution::{
    JsonContainer, JsonLayout, JsonLayoutStats, JsonPlacedItem, JsonSolution, JsonTransformation,
};
use crate::util::config::CDEConfig;
use crate::util::polygon_simplification;
use crate::util::polygon_simplification::{PolySimplConfig, PolySimplMode};

/// Parses a `JsonInstance` into an `Instance`.
pub struct Parser {
    poly_simpl_config: PolySimplConfig,
    cde_config: CDEConfig,
    center_polygons: bool,
}

impl Parser {
    pub fn new(
        poly_simpl_config: PolySimplConfig,
        cde_config: CDEConfig,
        center_polygons: bool,
    ) -> Parser {
        Parser {
            poly_simpl_config,
            cde_config,
            center_polygons,
        }
    }

    /// Parses a `JsonInstance` into an `Instance`.
    pub fn parse(&self, json_instance: &JsonInstance) -> Instance {
        let mut items: Vec<(Item, usize)> = vec![];
        let mut instance = None;

        crossbeam::thread::scope(|s| {
            let mut item_join_handles = vec![];
            for (item_id, json_item) in json_instance.items.iter().enumerate() {
                let handle = s.spawn(move |_| {
                    let (shape, centering_transf) = match &json_item.shape {
                        JsonShape::Rectangle{ width, height } => {
                            let shape = SimplePolygon::from(AARectangle::new(0.0, 0.0, *width, *height));
                            (shape, Transformation::empty())
                        }
                        JsonShape::SimplePolygon(sp) => convert_json_simple_poly(
                            sp,
                            self.center_polygons,
                            self.poly_simpl_config,
                            PolySimplMode::Inflate,
                        ),
                        JsonShape::Polygon(_) => {
                            panic!("No support for polygon to simplepolygon conversion yet")
                        }
                        JsonShape::MultiPolygon(_) => {
                            panic!("No support for multipolygon shapes yet")
                        }
                    };

                    let item_value = json_item.value.unwrap_or(0);
                    let base_quality = json_item.base_quality;

                    let allowed_orientations = match json_item.allowed_orientations.as_ref() {
                        Some(a_o) => {
                            if a_o.is_empty() || (a_o.len() == 1 && a_o[0] == 0.0) {
                                AllowedRotation::None
                            } else {
                                AllowedRotation::Discrete(
                                    a_o.iter().map(|angle| angle.to_radians()).collect(),
                                )
                            }
                        }
                        None => AllowedRotation::Continuous,
                    };

                    (
                        Item::new(
                            item_id,
                            shape,
                            item_value,
                            allowed_orientations,
                            centering_transf,
                            base_quality,
                            self.cde_config.item_surrogate_config.clone(),
                        ),
                        json_item.demand as usize,
                    )
                });
                item_join_handles.push(handle);
            }
            for join_handle in item_join_handles {
                items.push(join_handle.join().unwrap());
            }

            instance = match (json_instance.bins.as_ref(), json_instance.strip.as_ref()) {
                (Some(json_bins), None) => {
                    let mut bins: Vec<(Bin, usize)> = vec![];
                    let mut bin_join_handles = vec![];
                    for (bin_id, json_bin) in json_bins.iter().enumerate() {
                        let handle = s.spawn(move |_| {
                            let (bin_outer, centering_transf) = match &json_bin.shape {
                                JsonShape::Rectangle {width, height} => {
                                    let shape = SimplePolygon::from(AARectangle::new(0.0, 0.0, *width, *height));
                                    (shape, Transformation::empty())
                                }
                                JsonShape::SimplePolygon(jsp) => convert_json_simple_poly(
                                    jsp,
                                    self.center_polygons,
                                    self.poly_simpl_config,
                                    PolySimplMode::Deflate,
                                ),
                                JsonShape::Polygon(jp) => convert_json_simple_poly(
                                    &jp.outer,
                                    self.center_polygons,
                                    self.poly_simpl_config,
                                    PolySimplMode::Deflate,
                                ),
                                JsonShape::MultiPolygon(_) => {
                                    panic!("No support for multipolygon shapes yet")
                                }
                            };

                            let bin_holes = match &json_bin.shape {
                                JsonShape::SimplePolygon(_) | JsonShape::Rectangle {..} => vec![],
                                JsonShape::Polygon(jp) => jp
                                    .inner
                                    .iter()
                                    .map(|jsp| {
                                        let (hole, _) = convert_json_simple_poly(
                                            jsp,
                                            false,
                                            self.poly_simpl_config,
                                            PolySimplMode::Inflate,
                                        );
                                        hole.transform_clone(&centering_transf)
                                    })
                                    .collect_vec(),
                                JsonShape::MultiPolygon(_) => {
                                    panic!("No support for multipolygon shapes yet")
                                }
                            };

                            let material_value = (bin_outer.area()
                                - bin_holes.iter().map(|hole| hole.area()).sum::<f64>())
                                as u64;

                            assert!(
                                json_bin.zones.iter().all(|zone| zone.quality < N_QUALITIES),
                                "Quality must be less than N_QUALITIES"
                            );

                            let quality_zones = (0..N_QUALITIES)
                                .map(|quality| {
                                    let zones = json_bin
                                        .zones
                                        .iter()
                                        .filter(|zone| zone.quality == quality)
                                        .map(|zone| {
                                            let (zone_shape, _) = match &zone.shape {
                                                JsonShape::Rectangle { width, height } => {
                                                    let shape = SimplePolygon::from(AARectangle::new(0.0, 0.0, *width, *height));
                                                    (shape, Transformation::empty())
                                                }
                                                JsonShape::SimplePolygon(jsp) => convert_json_simple_poly(
                                                    jsp,
                                                    false,
                                                    self.poly_simpl_config,
                                                    PolySimplMode::Inflate,
                                                ),
                                                JsonShape::Polygon(_) => {
                                                    panic!("No support for polygon to simplepolygon conversion yet")
                                                }
                                                JsonShape::MultiPolygon(_) => {
                                                    panic!("No support for multipolygon shapes yet")
                                                }
                                            };
                                            zone_shape.transform_clone(&centering_transf)
                                        })
                                        .collect_vec();

                                    QualityZone::new(quality, zones)
                                })
                                .collect_vec();

                            let bin = Bin::new(
                                bin_id,
                                bin_outer,
                                material_value,
                                centering_transf,
                                bin_holes,
                                quality_zones,
                                self.cde_config,
                            );
                            let stock = json_bin.stock.unwrap_or(u64::MAX) as usize;

                            (bin, stock)
                        });
                        bin_join_handles.push(handle);
                    }
                    for join_handle in bin_join_handles {
                        bins.push(join_handle.join().unwrap());
                    }
                    Some(Instance::BP(BPInstance::new(items, bins)))
                }
                (None, Some(json_strip)) => {
                    Some(Instance::SP(SPInstance::new(items, json_strip.height)))
                }
                (Some(_), Some(_)) => panic!("Both bins and strip packing specified"),
                (None, None) => panic!("No bins or strip packing specified"),
            };
        })
        .unwrap();

        let instance = instance.expect("Instance not parsed");

        match &instance {
            Instance::SP(spi) => {
                log!(
                    Level::Info,
                    "[PARSE] strip packing instance \"{}\": {} items ({} unique), {} strip height",
                    json_instance.name,
                    spi.total_item_qty(),
                    spi.items.len(),
                    spi.strip_height
                );
            }
            Instance::BP(bpi) => {
                log!(
                    Level::Info,
                    "[PARSE] bin packing instance \"{}\": {} items ({} unique), {} bins ({} unique)",
                    json_instance.name,
                    bpi.total_item_qty(),
                    bpi.items.len(),
                    bpi.bins.iter().map(|(_, qty)| *qty).sum::<usize>(),
                    bpi.bins.len()
                );
            }
        }

        instance
    }

    /// Parses a `JsonInstance` and accompanying `JsonLayout`s into an `Instance` and `Solution`.
    pub fn parse_and_build_solution(
        &self,
        json_instance: &JsonInstance,
        json_layouts: &Vec<JsonLayout>,
    ) -> (Instance, Solution) {
        let instance = Arc::new(self.parse(json_instance));
        let solution = build_solution_from_json(&json_layouts, instance.clone(), self.cde_config);
        let instance =
            Arc::try_unwrap(instance).expect("Cannot unwrap instance, strong references present");
        (instance, solution)
    }
}

/// Builds a `Solution` from a set of `JsonLayout`s and an `Instance`.
fn build_solution_from_json(
    json_layouts: &[JsonLayout],
    instance: Arc<Instance>,
    cde_config: CDEConfig,
) -> Solution {
    let mut problem: Problem = match instance.as_ref() {
        Instance::BP(bp_i) => Problem::BP(BPProblem::new(bp_i.clone())),
        Instance::SP(sp_i) => {
            assert_eq!(json_layouts.len(), 1);
            match json_layouts[0].container {
                JsonContainer::Bin { .. } => panic!("Strip packing solution should not contain layouts with references to an Object"),
                JsonContainer::Strip { width, height: _ } => {
                    Problem::SP(SPProblem::new(sp_i.clone(), width, cde_config))
                }
            }
        }
    };

    for json_layout in json_layouts {
        let bin = match (instance.as_ref(), &json_layout.container) {
            (Instance::BP(bpi), JsonContainer::Bin { index }) => Some(&bpi.bins[*index].0),
            (Instance::SP(_spi), JsonContainer::Strip { .. }) => None,
            _ => panic!("Layout object type does not match packing type"),
        };
        //Create the layout by inserting the first item

        //Find the template layout matching the bin id in the JSON solution, 0 if strip packing instance.
        let (template_layout_index, _) = problem
            .template_layouts()
            .iter()
            .enumerate()
            .find(|(_, layout)| layout.bin().id == bin.map_or(0, |b| b.id))
            .unwrap();

        let bin_centering = bin
            .map_or(DTransformation::empty(), |b| {
                DTransformation::from(&b.centering_transform)
            })
            .translation();

        let json_first_item = json_layout.placed_items.get(0).unwrap();
        let first_item = instance.item(json_first_item.index);

        //all items have a centering transformation applied during parsing.
        //However, the transformation described in the JSON solution is relative to the item's original position, not the one after the centering transformation
        let first_item_centering_correction = first_item
            .centering_transform
            .clone()
            .inverse()
            .decompose()
            .translation();

        let transf = Transformation::empty()
            .translate(first_item_centering_correction) //undo the item centering transformation
            .rotate(json_first_item.transformation.rotation) //apply the rotation from the JSON solution
            .translate(json_first_item.transformation.translation) //apply the translation from the JSON solution
            .translate(bin_centering); //correct for the bin centering transformation

        let d_transf = transf.decompose();

        let initial_insert_opt = PlacingOption {
            layout_index: LayoutIndex::Template(template_layout_index),
            item_id: first_item.id,
            transform: transf,
            d_transform: d_transf,
        };
        problem.place_item(&initial_insert_opt);
        problem.flush_changes();

        //TODO: assuming layouts are always added to the back of the vector is not very robust
        let layout_index = problem.layouts().len() - 1;

        //Insert the rest of the items
        for json_item in json_layout.placed_items.iter().skip(1) {
            let item = instance.item(json_item.index);
            let item_centering_correction = item
                .centering_transform
                .clone()
                .inverse()
                .decompose()
                .translation();
            let transf = Transformation::empty()
                .translate(item_centering_correction)
                .rotate(json_item.transformation.rotation)
                .translate(json_item.transformation.translation)
                .translate(bin_centering);

            let d_transf = transf.decompose();

            let insert_opt = PlacingOption {
                layout_index: LayoutIndex::Real(layout_index),
                item_id: item.id,
                transform: transf,
                d_transform: d_transf,
            };
            problem.place_item(&insert_opt);
            problem.flush_changes();
        }
    }

    problem.create_solution(&None)
}

/// Composes a `JsonSolution` from a `Solution` and an `Instance`.
pub fn compose_json_solution(
    solution: &Solution,
    instance: &Instance,
    epoch: Instant,
) -> JsonSolution {
    let layouts = solution
        .layout_snapshots
        .iter()
        .map(|sl| {
            let container = match &instance {
                Instance::BP(_bpi) => JsonContainer::Bin { index: sl.bin.id },
                Instance::SP(spi) => JsonContainer::Strip {
                    width: sl.bin.bbox().width(),
                    height: spi.strip_height,
                },
            };
            //JSON solution should have their bins back in their original position, so we need to correct for the centering transformation
            let bin_centering_correction = match &instance {
                Instance::BP(bpi) => {
                    let bin = &bpi.bins[sl.bin.id].0;
                    bin.centering_transform
                        .clone()
                        .inverse()
                        .decompose()
                        .translation()
                }
                Instance::SP(_) => (0.0, 0.0), //no bin, no correction
            };

            let placed_items = sl
                .placed_items
                .iter()
                .map(|pl| {
                    let item_index = pl.item_id();
                    let item = instance.item(item_index);
                    let item_centering = item.centering_transform.decompose().translation();

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
                        index: item_index,
                        transformation: json_transform,
                    }
                })
                .collect::<Vec<JsonPlacedItem>>();
            let statistics = JsonLayoutStats { usage: sl.usage };
            JsonLayout {
                container,
                placed_items,
                statistics,
            }
        })
        .collect::<Vec<JsonLayout>>();

    JsonSolution {
        layouts,
        usage: solution.usage,
        run_time_sec: solution.time_stamp.duration_since(epoch).as_secs(),
    }
}

fn convert_json_simple_poly(
    s_json_shape: &JsonSimplePoly,
    center_polygon: bool,
    simpl_config: PolySimplConfig,
    simpl_mode: PolySimplMode,
) -> (SimplePolygon, Transformation) {
    let shape = SimplePolygon::new(json_simple_poly_to_points(s_json_shape));

    let shape = match simpl_config {
        PolySimplConfig::Enabled { tolerance } => {
            polygon_simplification::simplify_shape(&shape, simpl_mode, tolerance)
        }
        PolySimplConfig::Disabled => shape,
    };

    let (shape, centering_transform) = match center_polygon {
        true => shape.center_around_centroid(),
        false => (shape, Transformation::empty()),
    };

    (shape, centering_transform)
}

fn json_simple_poly_to_points(jsp: &JsonSimplePoly) -> Vec<Point> {
    //Strip the last vertex if it is the same as the first one
    let n_vertices = match jsp.0[0] == jsp.0[jsp.0.len() - 1] {
        true => jsp.0.len() - 1,
        false => jsp.0.len(),
    };

    (0..n_vertices).map(|i| Point::from(jsp.0[i])).collect_vec()
}
