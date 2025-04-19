use std::sync::Arc;

use itertools::Itertools;

use crate::collision_detection::hazards::Hazard;
use crate::collision_detection::hazards::HazardEntity;
use crate::collision_detection::{CDEConfig, CDEngine};
use crate::geometry::DTransformation;
use crate::geometry::geo_traits::Shape;
use crate::geometry::primitives::Rect;
use crate::geometry::primitives::SPolygon;

#[cfg(doc)]
use crate::entities::general::Item;
use crate::entities::general::original_shape::OriginalShape;
use crate::geometry::shape_modification::{ShapeModifyConfig, ShapeModifyMode};

/// A container in which [`Item`]'s can be placed.
#[derive(Clone, Debug)]
pub struct Bin {
    pub id: usize,
    /// Contour of the bin as defined in the input file
    pub outer_orig: Arc<OriginalShape>,
    /// Contour of the bin to be used for collision detection
    pub outer_cd: Arc<SPolygon>,
    /// The cost of using the bin
    pub cost: u64,
    /// Zones of different qualities in the bin, stored per quality.
    pub quality_zones: [Option<InferiorQualityZone>; N_QUALITIES],
    /// The starting state of the `CDEngine` for this bin.
    pub base_cde: Arc<CDEngine>,
}

impl Bin {
    pub fn new(
        id: usize,
        original_outer: OriginalShape,
        cost: u64,
        quality_zones: Vec<InferiorQualityZone>,
        cde_config: CDEConfig,
    ) -> Self {
        let outer_int = Arc::new(original_outer.convert_to_internal());
        let outer_orig = Arc::new(original_outer);
        assert_eq!(
            quality_zones.len(),
            quality_zones.iter().map(|qz| qz.quality).unique().count(),
            "Quality zones must have unique qualities"
        );
        assert!(
            quality_zones
                .iter()
                .map(|qz| qz.quality)
                .all(|q| q < N_QUALITIES),
            "All quality zones must be below N_QUALITIES: {}",
            N_QUALITIES
        );
        let quality_zones = {
            let mut qz = <[_; N_QUALITIES]>::default();
            for q in quality_zones {
                let quality = q.quality;
                qz[quality] = Some(q);
            }
            qz
        };

        let base_cde = {
            let mut hazards = vec![Hazard::new(HazardEntity::BinExterior, outer_int.clone())];
            let qz_hazards = quality_zones
                .iter()
                .flatten()
                .map(|qz| qz.to_hazards())
                .flatten();
            hazards.extend(qz_hazards);
            let base_cde = CDEngine::new(outer_int.bbox().inflate_to_square(), hazards, cde_config);
            Arc::new(base_cde)
        };

        Self {
            id,
            outer_cd: outer_int,
            outer_orig,
            cost,
            quality_zones,
            base_cde,
        }
    }

    /// Create a new `Bin` for a strip-packing problem. Instead of a shape, the bin is always rectangular.
    pub fn from_strip(
        id: usize,
        rect: Rect,
        cde_config: CDEConfig,
        shape_modify_config: ShapeModifyConfig,
    ) -> Self {
        assert_eq!(rect.x_min, 0.0, "Strip x_min must be 0.0");
        assert_eq!(rect.y_min, 0.0, "Strip y_min must be 0.0");

        let value = rect.area() as u64;
        let original = OriginalShape {
            shape: SPolygon::from(rect),
            pre_transform: DTransformation::empty(),
            modify_mode: ShapeModifyMode::Deflate,
            modify_config: shape_modify_config,
        };

        Bin::new(id, original, value, vec![], cde_config)
    }

    /// The area of the contour of the bin, excluding holes
    pub fn area(&self) -> f32 {
        self.outer_orig.area() - self.quality_zones[0].as_ref().map_or(0.0, |qz| qz.area())
    }
}

/// Maximum number of qualities that can be used for quality zones in a bin.
pub const N_QUALITIES: usize = 10;

/// Represents a zone of inferior quality in the [`Bin`]
#[derive(Clone, Debug)]
pub struct InferiorQualityZone {
    /// Quality of this zone. Higher qualities are superior. A zone with quality 0 is treated as a hole.
    pub quality: usize,
    /// Contours of this quality-zone as defined in the input file
    pub shapes_orig: Vec<Arc<OriginalShape>>,
    /// Contours of this quality-zone to be used for collision detection
    pub shapes_cd: Vec<Arc<SPolygon>>,
}

impl InferiorQualityZone {
    pub fn new(quality: usize, original_shapes: Vec<OriginalShape>) -> Self {
        assert!(
            quality < N_QUALITIES,
            "Quality must be in range of N_QUALITIES"
        );
        let shapes = original_shapes
            .iter()
            .map(|orig| orig.convert_to_internal())
            .map(|shape| Arc::new(shape))
            .collect_vec();

        let original_shapes = original_shapes.into_iter().map(Arc::new).collect_vec();

        Self {
            quality,
            shapes_cd: shapes,
            shapes_orig: original_shapes,
        }
    }

    /// Returns the set of hazards induced by this zone.
    pub fn to_hazards(&self) -> impl Iterator<Item = Hazard> {
        self.shapes_cd.iter().enumerate().map(|(id, shape)| {
            let entity = match self.quality {
                0 => HazardEntity::BinHole { id },
                _ => HazardEntity::InferiorQualityZone {
                    quality: self.quality,
                    id,
                },
            };
            Hazard::new(entity, shape.clone())
        })
    }

    pub fn area(&self) -> f32 {
        self.shapes_orig.iter().map(|shape| shape.area()).sum()
    }
}
