use std::time::Instant;

use crate::entities::bin_packing::BPInstance;
use crate::entities::bin_packing::BPSolution;
use crate::entities::general::Item;
use crate::entities::general::{Bin, InferiorQualityZone, N_QUALITIES};
use crate::entities::general::{Instance, OriginalShape};
use crate::entities::strip_packing::SPInstance;
use crate::entities::strip_packing::SPSolution;
use crate::fsize;
use crate::geometry::DTransformation;
use crate::geometry::Transformation;
use crate::geometry::geo_enums::AllowedRotation;
use crate::geometry::geo_traits::Shape;
use crate::geometry::primitives::AARectangle;
use crate::geometry::primitives::Point;
use crate::geometry::primitives::SimplePolygon;
use crate::io::json_instance::{JsonBin, JsonInstance, JsonItem, JsonShape, JsonSimplePoly};
use crate::io::json_solution::{
    JsonContainer, JsonLayout, JsonLayoutStats, JsonPlacedItem, JsonSolution, JsonTransformation,
};
use crate::util::ShapeModifyMode;
use crate::util::{CDEConfig, ShapeModifyConfig};
use itertools::Itertools;
use log::{Level, log};
use rayon::iter::IndexedParallelIterator;
use rayon::iter::ParallelIterator;
use rayon::prelude::IntoParallelRefIterator;

/// Parses a `JsonInstance` into an `Instance`.
pub struct Parser {
    shape_modify_config: ShapeModifyConfig,
    cde_config: CDEConfig,
}

impl Parser {
    /// Creates a new `Parser` with the given configuration.
    ///
    /// * `cde_config` - Configuration for the CDE (Collision Detection Engine).
    /// * `poly_simpl_tolerance` - See [`ShapeModifyConfig::simplify_tolerance`].
    /// * `min_item_separation` - Optional minimum separation distance between items and any other hazard.
    /// If enabled, every hazard is inflated/deflated by half this value. See [`ShapeModifyConfig::offset`].
    pub fn new(
        cde_config: CDEConfig,
        poly_simpl_tolerance: Option<fsize>,
        min_item_separation: Option<fsize>,
    ) -> Parser {
        let shape_modify_config = ShapeModifyConfig {
            offset: min_item_separation.map(|f| f / 2.0),
            simplify_tolerance: poly_simpl_tolerance,
        };
        Parser {
            shape_modify_config,
            cde_config,
        }
    }

    /// Parses a `JsonInstance` into an `Instance`.
    pub fn parse(&self, json_instance: &JsonInstance) -> Box<dyn Instance> {
        let items = json_instance
            .items
            .par_iter()
            .enumerate()
            .map(|(item_id, json_item)| self.parse_item(json_item, item_id))
            .collect();

        match (json_instance.bins.as_ref(), json_instance.strip.as_ref()) {
            (Some(json_bins), None) => {
                //bin packing instance
                let bins: Vec<(Bin, usize)> = json_bins
                    .par_iter()
                    .enumerate()
                    .map(|(bin_id, json_bin)| self.parse_bin(json_bin, bin_id))
                    .collect();
                let bpi = BPInstance::new(items, bins);
                log!(
                    Level::Info,
                    "[PARSE] bin packing instance \"{}\": {} items ({} unique), {} bins ({} unique)",
                    json_instance.name,
                    bpi.total_item_qty(),
                    bpi.items.len(),
                    bpi.bins.iter().map(|(_, qty)| *qty).sum::<usize>(),
                    bpi.bins.len()
                );
                Box::new(bpi)
            }
            (None, Some(json_strip)) => {
                let strip_modify_config = ShapeModifyConfig {
                    offset: self.shape_modify_config.offset,
                    simplify_tolerance: None,
                };
                let spi = SPInstance::new(items, json_strip.height, strip_modify_config);
                log!(
                    Level::Info,
                    "[PARSE] strip packing instance \"{}\": {} items ({} unique), {} strip height",
                    json_instance.name,
                    spi.total_item_qty(),
                    spi.items.len(),
                    spi.strip_height
                );
                Box::new(spi)
            }
            (Some(_), Some(_)) => {
                panic!("Both bins and strip packing specified, has to be one or the other")
            }
            (None, None) => panic!("Neither bins or strips specified"),
        }
    }

    fn parse_item(&self, json_item: &JsonItem, item_id: usize) -> (Item, usize) {
        let original_shape = {
            let shape = match &json_item.shape {
                JsonShape::Rectangle {
                    x_min,
                    y_min,
                    width,
                    height,
                } => AARectangle::new(*x_min, *y_min, x_min + width, y_min + height).into(),
                JsonShape::SimplePolygon(jsp) => {
                    SimplePolygon::new(json_simple_poly_to_points(jsp))
                }
                JsonShape::Polygon(_) => {
                    unimplemented!("No support for polygon shapes yet")
                }
                JsonShape::MultiPolygon(_) => {
                    unimplemented!("No support for multipolygon shapes yet")
                }
            };
            OriginalShape {
                pre_transform: centering_transformation(&shape),
                shape: shape,
                modify_mode: ShapeModifyMode::Inflate,
                modify_config: self.shape_modify_config,
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

        let item = Item::new(
            item_id,
            original_shape,
            allowed_orientations,
            base_quality,
            self.cde_config.item_surrogate_config,
            item_value,
        );

        (item, json_item.demand as usize)
    }

    fn parse_bin(&self, json_bin: &JsonBin, bin_id: usize) -> (Bin, usize) {
        assert!(
            json_bin.zones.iter().all(|zone| zone.quality < N_QUALITIES),
            "All quality zones must have lower quality than N_QUALITIES, configure N_QUALITIES to a higher value"
        );

        let original_outer = {
            let outer = match &json_bin.shape {
                JsonShape::Rectangle {
                    x_min,
                    y_min,
                    width,
                    height,
                } => AARectangle::new(*x_min, *y_min, x_min + width, y_min + height).into(),
                JsonShape::SimplePolygon(jsp) => {
                    SimplePolygon::new(json_simple_poly_to_points(jsp))
                }
                JsonShape::Polygon(jp) => SimplePolygon::new(json_simple_poly_to_points(&jp.outer)),
                JsonShape::MultiPolygon(_) => {
                    unimplemented!("No support for multipolygon shapes yet")
                }
            };
            OriginalShape {
                shape: outer,
                pre_transform: DTransformation::empty(),
                modify_mode: ShapeModifyMode::Deflate,
                modify_config: self.shape_modify_config,
            }
        };

        let bin_holes = match &json_bin.shape {
            JsonShape::SimplePolygon(_) | JsonShape::Rectangle { .. } => vec![],
            JsonShape::Polygon(jp) => {
                let json_holes = &jp.inner;
                json_holes
                    .iter()
                    .map(|jsp| SimplePolygon::new(json_simple_poly_to_points(jsp)))
                    .collect_vec()
            }
            JsonShape::MultiPolygon(_) => {
                unimplemented!("No support for multipolygon shapes yet")
            }
        };

        let mut shapes_inferior_qzones = (0..N_QUALITIES)
            .map(|q| {
                json_bin
                    .zones
                    .iter()
                    .filter(|zone| zone.quality == q)
                    .map(|zone| match &zone.shape {
                        JsonShape::Rectangle {
                            x_min,
                            y_min,
                            width,
                            height,
                        } => AARectangle::new(*x_min, *y_min, x_min + width, y_min + height).into(),
                        JsonShape::SimplePolygon(jsp) => {
                            SimplePolygon::new(json_simple_poly_to_points(jsp))
                        }
                        JsonShape::Polygon(_) => {
                            unimplemented!("No support for polygon to simplepolygon conversion yet")
                        }
                        JsonShape::MultiPolygon(_) => {
                            unimplemented!("No support for multipolygon shapes yet")
                        }
                    })
                    .collect_vec()
            })
            .collect_vec();

        //merge the bin holes with quality == 0
        shapes_inferior_qzones[0].extend(bin_holes);

        //convert the shapes to inferior quality zones
        let quality_zones = shapes_inferior_qzones
            .into_iter()
            .enumerate()
            .map(|(q, zone_shapes)| {
                let original_shapes = zone_shapes
                    .into_iter()
                    .map(|s| OriginalShape {
                        shape: s,
                        pre_transform: DTransformation::empty(),
                        modify_mode: ShapeModifyMode::Inflate,
                        modify_config: self.shape_modify_config,
                    })
                    .collect_vec();
                InferiorQualityZone::new(q, original_shapes)
            })
            .collect_vec();

        let bin = Bin::new(
            bin_id,
            original_outer,
            json_bin.cost,
            quality_zones,
            self.cde_config,
        );

        let stock = json_bin.stock.unwrap_or(u64::MAX) as usize;

        (bin, stock)
    }
}

/// Composes a `JsonSolution` from a `SPSolution` and an `SPInstance`.
pub fn compose_json_solution_spp(
    solution: &SPSolution,
    instance: &SPInstance,
    epoch: Instant,
) -> JsonSolution {
    let container = JsonContainer::Strip {
        width: solution.strip_width,
        height: instance.strip_height,
    };

    let placed_items = solution
        .layout_snapshot
        .placed_items
        .values()
        .map(|placed_item| {
            let item_index = placed_item.item_id;
            let item = instance.item(item_index);

            let abs_transf = internal_to_absolute_transform(
                &placed_item.d_transf,
                &item.shape_orig.pre_transform,
            );

            JsonPlacedItem {
                index: item_index,
                transformation: JsonTransformation {
                    rotation: abs_transf.rotation(),
                    translation: abs_transf.translation(),
                },
            }
        })
        .collect::<Vec<JsonPlacedItem>>();
    let statistics = JsonLayoutStats {
        density: solution.density(instance),
    };
    JsonSolution {
        layouts: vec![JsonLayout {
            container,
            placed_items,
            statistics,
        }],
        density: solution.density(instance),
        run_time_sec: solution.time_stamp.duration_since(epoch).as_secs(),
    }
}

/// Composes a `JsonSolution` from a `BPSolution` and an `BPInstance`.
pub fn compose_json_solution_bpp(
    solution: &BPSolution,
    instance: &BPInstance,
    epoch: Instant,
) -> JsonSolution {
    let layouts = solution
        .layout_snapshots
        .iter()
        .map(|(_, sl)| {
            let container = JsonContainer::Bin { index: sl.bin.id };
            let placed_items = sl
                .placed_items
                .values()
                .map(|placed_item| {
                    let item_index = placed_item.item_id;
                    let item = instance.item(item_index);

                    let abs_transf = internal_to_absolute_transform(
                        &placed_item.d_transf,
                        &item.shape_orig.pre_transform,
                    );

                    JsonPlacedItem {
                        index: item_index,
                        transformation: JsonTransformation {
                            rotation: abs_transf.rotation(),
                            translation: abs_transf.translation(),
                        },
                    }
                })
                .collect::<Vec<JsonPlacedItem>>();
            let statistics = JsonLayoutStats {
                density: sl.density(instance),
            };
            JsonLayout {
                container,
                placed_items,
                statistics,
            }
        })
        .collect::<Vec<JsonLayout>>();

    JsonSolution {
        layouts,
        density: solution.density(instance),
        run_time_sec: solution.time_stamp.duration_since(epoch).as_secs(),
    }
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
    internal_transf: &DTransformation,
    pre_transf: &DTransformation,
) -> DTransformation {
    //1. apply the pre-transform
    //2. apply the internal transformation

    Transformation::empty()
        .transform_from_decomposed(pre_transf)
        .transform_from_decomposed(internal_transf)
        .decompose()
}

pub fn absolute_to_internal_transform(
    absolute_transf: &DTransformation,
    pre_transf: &DTransformation,
) -> DTransformation {
    //1. undo pre-transform
    //2. do the absolute transformation

    Transformation::empty()
        .transform(&pre_transf.compose().inverse())
        .transform_from_decomposed(absolute_transf)
        .decompose()
}

pub fn centering_transformation(shape: &SimplePolygon) -> DTransformation {
    let Point(cx, cy) = shape.centroid();
    DTransformation::new(0.0, (-cx, -cy))
}
