use std::sync::Arc;

use itertools::Itertools;

use crate::collision_detection::hazards::Hazard;
use crate::collision_detection::hazards::HazardEntity;
use crate::collision_detection::{CDEConfig, CDEngine};
use crate::geometry::OriginalShape;
use crate::geometry::fail_fast::SPSurrogateConfig;
use crate::geometry::primitives::SPolygon;

use anyhow::{Result, ensure};

/// A container in which [`Item`](crate::entities::Item)'s can be placed.
#[derive(Clone, Debug)]
pub struct Container {
    /// Original contour of the container as defined in the input
    pub(crate) outer_orig: Arc<OriginalShape>,
    /// Contour of the container to be used for collision detection
    pub(crate) outer_cd: Arc<SPolygon>,
    /// Zones of different qualities in the container, stored per quality.
    pub(crate) quality_zones: [Option<InferiorQualityZone>; N_QUALITIES],
    /// The initial state of the `CDEngine` for this container. (equivalent to an empty layout using this container)
    pub(crate) base_cde: Arc<CDEngine>,
}

impl Container {
    pub fn new(
        original_outer: OriginalShape,
        quality_zones: Vec<InferiorQualityZone>,
        cde_config: CDEConfig,
    ) -> Result<Self> {
        let outer = Arc::new(original_outer.convert_to_internal()?);
        let outer_orig = Arc::new(original_outer);
        ensure!(
            quality_zones.len() == quality_zones.iter().map(|qz| qz.quality).unique().count(),
            "Quality zones must have unique qualities"
        );
        ensure!(
            quality_zones
                .iter()
                .map(|qz| qz.quality)
                .all(|q| q < N_QUALITIES),
            "All quality zones must be below N_QUALITIES: {N_QUALITIES}"
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
            let mut hazards = vec![Hazard::new(HazardEntity::Exterior, outer.clone(), false)];
            let qz_hazards = quality_zones
                .iter()
                .flatten()
                .flat_map(InferiorQualityZone::to_hazards);
            hazards.extend(qz_hazards);
            let base_cde = CDEngine::new(outer.bbox.inflate_to_square(), hazards, cde_config);
            Arc::new(base_cde)
        };

        Ok(Self {
            outer_cd: outer,
            outer_orig,
            quality_zones,
            base_cde,
        })
    }

    /// Gross area of the original outer contour, without subtracting holes or quality zones.
    #[must_use]
    pub fn area(&self) -> f32 {
        self.outer_orig.area()
    }

    /// Original contour of the container as defined in the input
    #[must_use]
    pub fn outer_orig(&self) -> &Arc<OriginalShape> {
        &self.outer_orig
    }

    /// Contour of the container to be used for collision detection
    #[must_use]
    pub fn outer_cd(&self) -> &Arc<SPolygon> {
        &self.outer_cd
    }

    /// Zones of different qualities in the container, stored per quality.
    #[must_use]
    pub fn quality_zones(&self) -> &[Option<InferiorQualityZone>; N_QUALITIES] {
        &self.quality_zones
    }

    /// The initial state of the `CDEngine` for this container. (equivalent to an empty layout using this container)
    #[must_use]
    pub fn base_cde(&self) -> &Arc<CDEngine> {
        &self.base_cde
    }
}

/// Maximum number of qualities that can be used for quality zones in a container.
pub const N_QUALITIES: usize = 10;

/// Represents a zone of inferior quality in the [`Container`]
#[derive(Clone, Debug)]
pub struct InferiorQualityZone {
    /// Quality of this zone. Higher qualities are superior. A zone with quality 0 is treated as a hole.
    pub(crate) quality: usize,
    /// Contours of this quality-zone as defined in the input file
    pub(crate) shapes_orig: Vec<Arc<OriginalShape>>,
    /// Contours of this quality-zone to be used for collision detection
    pub(crate) shapes_cd: Vec<Arc<SPolygon>>,
}

impl InferiorQualityZone {
    /// Converts the shapes for collision detection and generates their surrogates,
    /// so they can be quantified as collision targets.
    /// Returns an error for invalid geometry or a quality outside `0..N_QUALITIES`.
    pub fn new(
        quality: usize,
        original_shapes: Vec<OriginalShape>,
        surrogate_config: SPSurrogateConfig,
    ) -> Result<Self> {
        ensure!(
            quality < N_QUALITIES,
            "Quality must be in range of N_QUALITIES"
        );
        let shapes: Result<Vec<Arc<SPolygon>>> = original_shapes
            .iter()
            .map(|orig| {
                let mut shape = orig.convert_to_internal()?;
                shape.generate_surrogate(surrogate_config)?;
                Ok(Arc::new(shape))
            })
            .collect();

        let original_shapes = original_shapes.into_iter().map(Arc::new).collect_vec();

        Ok(Self {
            quality,
            shapes_cd: shapes?,
            shapes_orig: original_shapes,
        })
    }

    /// Returns a zone with only the shapes whose collision contour satisfies `f`, in their original order.
    /// Kept shapes are shared, not converted again. Hazard indices refer to the new zone's order.
    #[must_use]
    pub fn filtered(&self, mut f: impl FnMut(&SPolygon) -> bool) -> Self {
        let (shapes_orig, shapes_cd) = self
            .shapes_orig
            .iter()
            .zip(&self.shapes_cd)
            .filter(|(_, shape)| f(shape))
            .map(|(orig, cd)| (orig.clone(), cd.clone()))
            .unzip();
        Self {
            quality: self.quality,
            shapes_orig,
            shapes_cd,
        }
    }

    /// Returns the set of hazards induced by this zone.
    pub fn to_hazards(&self) -> impl Iterator<Item = Hazard> {
        self.shapes_cd.iter().enumerate().map(|(idx, shape)| {
            let entity = match self.quality {
                0 => HazardEntity::Hole { idx },
                _ => HazardEntity::InferiorQualityZone {
                    quality: self.quality,
                    idx,
                },
            };
            Hazard::new(entity, shape.clone(), false)
        })
    }

    #[must_use]
    pub fn area(&self) -> f32 {
        self.shapes_orig.iter().map(|shape| shape.area()).sum()
    }

    /// Quality of this zone. Higher qualities are superior. A zone with quality 0 is treated as a hole.
    #[must_use]
    pub fn quality(&self) -> usize {
        self.quality
    }

    /// Contours of this quality-zone as defined in the input file
    #[must_use]
    pub fn shapes_orig(&self) -> &[Arc<OriginalShape>] {
        &self.shapes_orig
    }

    /// Contours of this quality-zone to be used for collision detection
    #[must_use]
    pub fn shapes_cd(&self) -> &[Arc<SPolygon>] {
        &self.shapes_cd
    }
}
