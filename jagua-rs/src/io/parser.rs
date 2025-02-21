use std::sync::Arc;
use std::time::Instant;

use crate::entities::bin::Bin;
use crate::entities::instances::bin_packing::BPInstance;
use crate::entities::instances::instance::Instance;
use crate::entities::instances::instance_generic::InstanceGeneric;
use crate::entities::instances::strip_packing::SPInstance;
use crate::entities::item::Item;
use crate::entities::placing_option::PlacingOption;
use crate::entities::problems::bin_packing::BPProblem;
use crate::entities::problems::problem_generic::{LayoutIndex, ProblemGeneric, STRIP_LAYOUT_IDX};
use crate::entities::problems::strip_packing::SPProblem;
use crate::entities::quality_zone::InferiorQualityZone;
use crate::entities::quality_zone::N_QUALITIES;
use crate::entities::solution::Solution;
use crate::fsize;
use crate::geometry::d_transformation::DTransformation;
use crate::geometry::geo_enums::AllowedRotation;
use crate::geometry::geo_traits::{Shape, Transformable};
use crate::geometry::primitives::aa_rectangle::AARectangle;
use crate::geometry::primitives::point::Point;
use crate::geometry::primitives::simple_polygon::SimplePolygon;
use crate::geometry::transformation::Transformation;
use crate::io::json_instance::{JsonBin, JsonInstance, JsonItem, JsonShape, JsonSimplePoly};
use crate::io::json_solution::{
    JsonContainer, JsonLayout, JsonLayoutStats, JsonPlacedItem, JsonSolution, JsonTransformation,
};
use crate::util::config::CDEConfig;
use crate::util::polygon_simplification;
use crate::util::polygon_simplification::{PolySimplConfig, PolySimplMode};
use itertools::Itertools;
use log::{Level, log};
use rayon::iter::IndexedParallelIterator;
use rayon::iter::ParallelIterator;
use rayon::prelude::IntoParallelRefIterator;

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
        let items = json_instance
            .items
            .par_iter()
            .enumerate()
            .map(|(item_id, json_item)| self.parse_item(json_item, item_id))
            .collect();

        let instance: Instance = match (json_instance.bins.as_ref(), json_instance.strip.as_ref()) {
            (Some(json_bins), None) => {
                let bins: Vec<(Bin, usize)> = json_bins
                    .par_iter()
                    .enumerate()
                    .map(|(bin_id, json_bin)| self.parse_bin(json_bin, bin_id))
                    .collect();
                BPInstance::new(items, bins).into()
            }
            (None, Some(json_strip)) => SPInstance::new(items, json_strip.height).into(),
            (Some(_), Some(_)) => {
                panic!("Both bins and strip packing specified, has to be one or the other")
            }
            (None, None) => panic!("Neither bins or strips specified"),
        };

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
        json_layouts: &[JsonLayout],
    ) -> (Instance, Solution) {
        let instance = Arc::new(self.parse(json_instance));
        let solution = build_solution_from_json(instance.as_ref(), json_layouts, self.cde_config);
        let instance =
            Arc::try_unwrap(instance).expect("Cannot unwrap instance, strong references present");
        (instance, solution)
    }

    fn parse_item(&self, json_item: &JsonItem, item_id: usize) -> (Item, usize) {
        let shape = match &json_item.shape {
            JsonShape::Rectangle { width, height } => {
                SimplePolygon::from(AARectangle::new(0.0, 0.0, *width, *height))
            }
            JsonShape::SimplePolygon(sp) => {
                convert_json_simple_poly(sp, self.poly_simpl_config, PolySimplMode::Inflate)
            }
            JsonShape::Polygon(_) => {
                unimplemented!("No support for polygon shapes yet")
            }
            JsonShape::MultiPolygon(_) => {
                unimplemented!("No support for multipolygon shapes yet")
            }
        };

        let item_value = json_item.value.unwrap_or(0);
        let base_quality = json_item.base_quality;

        let allowed_orientations = match json_item.allowed_orientations.as_ref() {
            Some(a_o) => {
                if a_o.is_empty() || (a_o.len() == 1 && a_o[0] == 0.0) {
                    AllowedRotation::None
                } else {
                    AllowedRotation::Discrete(a_o.iter().map(|angle| angle.to_radians()).collect())
                }
            }
            None => AllowedRotation::Continuous,
        };

        let base_item = Item::new(
            item_id,
            shape,
            allowed_orientations,
            base_quality,
            item_value,
            Transformation::empty(),
            self.cde_config.item_surrogate_config,
        );

        let item = match self.center_polygons {
            false => base_item,
            true => {
                let centering_transform = centering_transformation(&base_item.shape);
                pretransform_item(&base_item, &centering_transform.compose())
            }
        };

        (item, json_item.demand as usize)
    }

    fn parse_bin(&self, json_bin: &JsonBin, bin_id: usize) -> (Bin, usize) {
        let bin_outer = match &json_bin.shape {
            JsonShape::Rectangle { width, height } => {
                SimplePolygon::from(AARectangle::new(0.0, 0.0, *width, *height))
            }
            JsonShape::SimplePolygon(jsp) => {
                convert_json_simple_poly(jsp, self.poly_simpl_config, PolySimplMode::Deflate)
            }
            JsonShape::Polygon(jp) => {
                convert_json_simple_poly(&jp.outer, self.poly_simpl_config, PolySimplMode::Deflate)
            }
            JsonShape::MultiPolygon(_) => {
                unimplemented!("No support for multipolygon shapes yet")
            }
        };

        let bin_holes = match &json_bin.shape {
            JsonShape::SimplePolygon(_) | JsonShape::Rectangle { .. } => vec![],
            JsonShape::Polygon(jp) => jp
                .inner
                .iter()
                .map(|jsp| {
                    convert_json_simple_poly(jsp, self.poly_simpl_config, PolySimplMode::Inflate)
                })
                .collect_vec(),
            JsonShape::MultiPolygon(_) => {
                unimplemented!("No support for multipolygon shapes yet")
            }
        };

        let material_value =
            (bin_outer.area() - bin_holes.iter().map(|hole| hole.area()).sum::<fsize>()) as u64;

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
                    .map(|zone| match &zone.shape {
                        JsonShape::Rectangle { width, height } => {
                            SimplePolygon::from(AARectangle::new(0.0, 0.0, *width, *height))
                        }
                        JsonShape::SimplePolygon(jsp) => convert_json_simple_poly(
                            jsp,
                            self.poly_simpl_config,
                            PolySimplMode::Inflate,
                        ),
                        JsonShape::Polygon(_) => {
                            unimplemented!("No support for polygon to simplepolygon conversion yet")
                        }
                        JsonShape::MultiPolygon(_) => {
                            unimplemented!("No support for multipolygon shapes yet")
                        }
                    })
                    .collect_vec();
                InferiorQualityZone::new(quality, zones)
            })
            .collect_vec();

        let base_bin = Bin::new(
            bin_id,
            bin_outer,
            material_value,
            Transformation::empty(),
            bin_holes,
            quality_zones,
            self.cde_config,
        );

        let bin = match self.center_polygons {
            false => base_bin,
            true => {
                let centering_transform = centering_transformation(&base_bin.outer);
                pretransform_bin(&base_bin, &centering_transform.compose())
            }
        };

        let stock = json_bin.stock.unwrap_or(u64::MAX) as usize;

        (bin, stock)
    }
}

/// Builds a `Solution` from a set of `JsonLayout`s and an `Instance`.
pub fn build_solution_from_json(
    instance: &Instance,
    json_layouts: &[JsonLayout],
    cde_config: CDEConfig,
) -> Solution {
    match instance {
        Instance::BP(bp_i) => build_bin_packing_solution(bp_i, json_layouts),
        Instance::SP(sp_i) => {
            assert_eq!(json_layouts.len(), 1);
            build_strip_packing_solution(sp_i, &json_layouts[0], cde_config)
        }
    }
}

pub fn build_strip_packing_solution(
    instance: &SPInstance,
    json_layout: &JsonLayout,
    cde_config: CDEConfig,
) -> Solution {
    let mut problem = match json_layout.container {
        JsonContainer::Bin { .. } => {
            panic!("Strip packing solution should not contain layouts with references to an Object")
        }
        JsonContainer::Strip { width, height: _ } => {
            SPProblem::new(instance.clone(), width, cde_config)
        }
    };

    for json_item in json_layout.placed_items.iter() {
        let item = instance.item(json_item.index);
        let json_rotation = json_item.transformation.rotation;
        let json_translation = json_item.transformation.translation;

        let abs_transform = DTransformation::new(json_rotation, json_translation);
        let transform = absolute_to_internal_transform(
            &abs_transform,
            &item.pretransform,
            &problem.layout.bin.pretransform,
        );

        let d_transf = transform.decompose();

        let placing_opt = PlacingOption {
            layout_idx: STRIP_LAYOUT_IDX,
            item_id: item.id,
            d_transf,
        };

        problem.place_item(placing_opt);
        problem.flush_changes();
    }

    problem.create_solution(None)
}

pub fn build_bin_packing_solution(instance: &BPInstance, json_layouts: &[JsonLayout]) -> Solution {
    let mut problem = BPProblem::new(instance.clone());

    for json_layout in json_layouts {
        let bin = match json_layout.container {
            JsonContainer::Bin { index } => &instance.bins[index].0,
            JsonContainer::Strip { .. } => {
                panic!("Bin packing solution should not contain layouts with references to a Strip")
            }
        };
        //Create the layout by inserting the first item

        //Find the template layout matching the bin id in the JSON solution
        let template_index = problem
            .template_layouts()
            .iter()
            .position(|tl| tl.bin.id == bin.id)
            .expect("no template layout found for bin");

        let json_first_item = json_layout
            .placed_items
            .first()
            .expect("no items in layout");
        let first_item = instance.item(json_first_item.index);
        let abs_transform = DTransformation::new(
            json_first_item.transformation.rotation,
            json_first_item.transformation.translation,
        );

        let transform = absolute_to_internal_transform(
            &abs_transform,
            &first_item.pretransform,
            &bin.pretransform,
        );
        let d_transf = transform.decompose();

        let initial_insert_opt = PlacingOption {
            layout_idx: LayoutIndex::Template(template_index),
            item_id: first_item.id,
            d_transf,
        };
        let (layout_idx, _) = problem.place_item(initial_insert_opt);
        problem.flush_changes();

        //Insert the rest of the items
        for json_item in json_layout.placed_items.iter().skip(1) {
            let item = instance.item(json_item.index);
            let json_rotation = json_item.transformation.rotation;
            let json_translation = json_item.transformation.translation;

            let abs_transform = DTransformation::new(json_rotation, json_translation);
            let transform = absolute_to_internal_transform(
                &abs_transform,
                &item.pretransform,
                &bin.pretransform,
            );

            let d_transf = transform.decompose();

            let insert_opt = PlacingOption {
                layout_idx,
                item_id: item.id,
                d_transf,
            };
            problem.place_item(insert_opt);
            problem.flush_changes();
        }
    }

    problem.create_solution(None)
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

            let placed_items = sl
                .placed_items
                .values()
                .map(|placed_item| {
                    let item_index = placed_item.item_id;
                    let item = instance.item(item_index);

                    let abs_transf = internal_to_absolute_transform(
                        &placed_item.d_transf,
                        &item.pretransform,
                        &sl.bin.pretransform,
                    )
                    .decompose();

                    JsonPlacedItem {
                        index: item_index,
                        transformation: JsonTransformation {
                            rotation: abs_transf.rotation(),
                            translation: abs_transf.translation(),
                        },
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
    simpl_config: PolySimplConfig,
    simpl_mode: PolySimplMode,
) -> SimplePolygon {
    let shape = SimplePolygon::new(json_simple_poly_to_points(s_json_shape));

    let shape = match simpl_config {
        PolySimplConfig::Enabled { tolerance } => {
            polygon_simplification::simplify_shape(&shape, simpl_mode, tolerance)
        }
        PolySimplConfig::Disabled => shape,
    };

    shape
}

fn json_simple_poly_to_points(jsp: &JsonSimplePoly) -> Vec<Point> {
    //Strip the last vertex if it is the same as the first one
    let n_vertices = match jsp.0[0] == jsp.0[jsp.0.len() - 1] {
        true => jsp.0.len() - 1,
        false => jsp.0.len(),
    };

    (0..n_vertices).map(|i| Point::from(jsp.0[i])).collect_vec()
}

pub fn internal_to_absolute_transform(
    placed_item_transf: &DTransformation,
    item_pretransf: &Transformation,
    bin_pretransf: &Transformation,
) -> Transformation {
    //1. apply the item pretransform
    //2. apply the placement transformation
    //3. undo the bin pretransformation

    Transformation::empty()
        .transform(item_pretransf)
        .transform_from_decomposed(placed_item_transf)
        .transform(&bin_pretransf.clone().inverse())
}

pub fn absolute_to_internal_transform(
    abs_transf: &DTransformation,
    item_pretransf: &Transformation,
    bin_pretransf: &Transformation,
) -> Transformation {
    //1. undo the item pretransform
    //2. do the absolute transformation
    //3. apply the bin pretransform

    Transformation::empty()
        .transform(&item_pretransf.clone().inverse())
        .transform_from_decomposed(abs_transf)
        .transform(bin_pretransf)
}

pub fn pretransform_bin(bin: &Bin, extra_pretransf: &Transformation) -> Bin {
    let Bin {
        id,
        outer,
        value,
        pretransform,
        holes,
        quality_zones,
        ..
    } = bin;

    Bin::new(
        *id,
        outer.transform_clone(&extra_pretransf),
        *value,
        pretransform.clone().transform(&extra_pretransf),
        holes
            .iter()
            .map(|h| h.transform_clone(&extra_pretransf))
            .collect(),
        quality_zones
            .iter()
            .flatten()
            .map(|qz| {
                InferiorQualityZone::new(
                    qz.quality,
                    qz.zones
                        .iter()
                        .map(|z| z.transform_clone(&extra_pretransf))
                        .collect(),
                )
            })
            .collect(),
        bin.base_cde.config(),
    )
}

pub fn pretransform_item(item: &Item, extra_pretransf: &Transformation) -> Item {
    let Item {
        id,
        shape,
        allowed_rotation,
        base_quality,
        value,
        pretransform,
        surrogate_config,
        ..
    } = item;

    Item::new(
        *id,
        shape.transform_clone(extra_pretransf),
        allowed_rotation.clone(),
        *base_quality,
        *value,
        pretransform.clone().transform(extra_pretransf),
        *surrogate_config,
    )
}

pub fn centering_transformation(shape: &SimplePolygon) -> DTransformation {
    let Point(cx, cy) = shape.centroid();
    DTransformation::new(0.0, (-cx, -cy))
}
