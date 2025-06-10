use crate::collision_detection::CDEConfig;
use crate::entities::Item;
use crate::entities::{Container, InferiorQualityZone, N_QUALITIES};
use crate::geometry::OriginalShape;
use crate::geometry::geo_enums::RotationRange;
use crate::geometry::primitives::Point;
use crate::geometry::primitives::Rect;
use crate::geometry::primitives::SPolygon;
use crate::geometry::shape_modification::{ShapeModifyConfig, ShapeModifyMode};
use crate::geometry::{DTransformation, Transformation};
use crate::io::ext_repr::{ExtContainer, ExtItem, ExtSPolygon, ExtShape};
use anyhow::{Result, bail};
use itertools::Itertools;

/// Converts external representations of items and containers into internal ones.
#[derive(Clone, Debug, Copy)]
pub struct Importer {
    pub shape_modify_config: ShapeModifyConfig,
    pub cde_config: CDEConfig,
}

impl Importer {
    /// Creates a new instance with the given configuration.
    ///
    /// * `cde_config` - Configuration for the CDE (Collision Detection Engine).
    /// * `poly_simpl_tolerance` - See [`ShapeModifyConfig::simplify_tolerance`].
    /// * `min_item_separation` - Optional minimum separation distance between items and any other hazard. If enabled, every hazard is inflated/deflated by half this value. See [`ShapeModifyConfig::offset`].
    pub fn new(
        cde_config: CDEConfig,
        poly_simpl_tolerance: Option<f32>,
        min_item_separation: Option<f32>,
    ) -> Importer {
        Importer {
            shape_modify_config: ShapeModifyConfig {
                offset: min_item_separation.map(|f| f / 2.0),
                simplify_tolerance: poly_simpl_tolerance,
            },
            cde_config,
        }
    }

    pub fn import_item(&self, ext_item: &ExtItem) -> Result<Item> {
        let original_shape = {
            let shape = match &ext_item.shape {
                ExtShape::Rectangle {
                    x_min,
                    y_min,
                    width,
                    height,
                } => {
                    let rect = Rect::try_new(*x_min, *y_min, x_min + width, y_min + height)?;
                    SPolygon::from(rect)
                }
                ExtShape::SimplePolygon(esp) => import_simple_polygon(esp)?,
                ExtShape::Polygon(_) => {
                    bail!("No support for polygons with holes yet")
                }
                ExtShape::MultiPolygon(_) => {
                    bail!("No support for multipolygons yet")
                }
            };
            OriginalShape {
                pre_transform: centering_transformation(&shape),
                shape,
                modify_mode: ShapeModifyMode::Inflate,
                modify_config: self.shape_modify_config,
            }
        };

        let base_quality = ext_item.min_quality;

        let allowed_orientations = match ext_item.allowed_orientations.as_ref() {
            Some(a_o) => {
                if a_o.is_empty() || (a_o.len() == 1 && a_o[0] == 0.0) {
                    RotationRange::None
                } else {
                    RotationRange::Discrete(a_o.iter().map(|angle| angle.to_radians()).collect())
                }
            }
            None => RotationRange::Continuous,
        };

        Item::new(
            ext_item.id as usize,
            original_shape,
            allowed_orientations,
            base_quality,
            self.cde_config.item_surrogate_config,
        )
    }

    pub fn import_container(&self, ext_cont: &ExtContainer) -> Result<Container> {
        assert!(
            ext_cont.zones.iter().all(|zone| zone.quality < N_QUALITIES),
            "All quality zones must have lower quality than N_QUALITIES, set N_QUALITIES to a higher value if required"
        );

        let original_outer = {
            let outer = match &ext_cont.shape {
                ExtShape::Rectangle {
                    x_min,
                    y_min,
                    width,
                    height,
                } => Rect::try_new(*x_min, *y_min, x_min + width, y_min + height)?.into(),
                ExtShape::SimplePolygon(esp) => import_simple_polygon(esp)?,
                ExtShape::Polygon(ep) => import_simple_polygon(&ep.outer)?,
                ExtShape::MultiPolygon(_) => {
                    bail!("No support for multipolygon shapes yet")
                }
            };
            OriginalShape {
                shape: outer,
                pre_transform: DTransformation::empty(),
                modify_mode: ShapeModifyMode::Deflate,
                modify_config: self.shape_modify_config,
            }
        };

        let holes = match &ext_cont.shape {
            ExtShape::SimplePolygon(_) | ExtShape::Rectangle { .. } => vec![],
            ExtShape::Polygon(jp) => {
                let json_holes = &jp.inner;
                json_holes
                    .iter()
                    .map(import_simple_polygon)
                    .collect::<Result<Vec<SPolygon>>>()?
            }
            ExtShape::MultiPolygon(_) => {
                unimplemented!("No support for multipolygon shapes yet")
            }
        };

        let mut shapes_inferior_qzones = (0..N_QUALITIES)
            .map(|q| {
                ext_cont
                    .zones
                    .iter()
                    .filter(|zone| zone.quality == q)
                    .map(|zone| match &zone.shape {
                        ExtShape::Rectangle {
                            x_min,
                            y_min,
                            width,
                            height,
                        } => Rect::try_new(*x_min, *y_min, x_min + width, y_min + height)
                            .map(|r| r.into()),
                        ExtShape::SimplePolygon(esp) => import_simple_polygon(esp),
                        ExtShape::Polygon(_) => {
                            unimplemented!("No support for polygon to simplepolygon conversion yet")
                        }
                        ExtShape::MultiPolygon(_) => {
                            unimplemented!("No support for multipolygon shapes yet")
                        }
                    })
                    .collect::<Result<Vec<SPolygon>>>()
            })
            .collect::<Result<Vec<Vec<SPolygon>>>>()?;

        //merge the container holes with quality == 0
        shapes_inferior_qzones[0].extend(holes);

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
            .collect::<Result<Vec<InferiorQualityZone>>>()?;

        Container::new(
            ext_cont.id as usize,
            original_outer,
            quality_zones,
            self.cde_config,
        )
    }
}

pub fn import_simple_polygon(sp: &ExtSPolygon) -> Result<SPolygon> {
    let mut points = sp.0.iter().map(|(x, y)| Point(*x, *y)).collect_vec();
    //Strip the last vertex if it is the same as the first one
    if points.len() > 1 && points[0] == points[points.len() - 1] {
        points.pop();
    }
    //Remove duplicates that are consecutive (e.g. [1, 2, 2, 3] -> [1, 2, 3])
    points.dedup();
    //Bail if there are any non-consecutive duplicates.
    if points.len() != points.iter().unique().count() {
        bail!("Simple polygon has non-consecutive duplicate vertices");
    }
    SPolygon::new(points)
}

/// Returns a transformation that translates the shape's centroid to the origin.
pub fn centering_transformation(shape: &SPolygon) -> DTransformation {
    let Point(cx, cy) = shape.centroid();
    DTransformation::new(0.0, (-cx, -cy))
}

/// Converts an external transformation (applicable to the original shapes) to an internal transformation (used within `jagua-rs`).
///
/// * `ext_transf` - The external transformation.
/// * `pre_transf` - The transformation that was applied to the original shape to derive the internal representation.
pub fn ext_to_int_transformation(
    ext_transf: &DTransformation,
    pre_transf: &DTransformation,
) -> DTransformation {
    //1. undo pre-transform
    //2. do the absolute transformation

    Transformation::empty()
        .transform(&pre_transf.compose().inverse())
        .transform_from_decomposed(ext_transf)
        .decompose()
}
