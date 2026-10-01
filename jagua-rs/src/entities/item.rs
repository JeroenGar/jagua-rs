use std::sync::Arc;

use crate::geometry::fail_fast::SPSurrogateConfig;
use crate::geometry::primitives::SPolygon;
use crate::geometry::{AllowedOrientations, OriginalShape};

use crate::entities::N_QUALITIES;
use anyhow::{Result, ensure};

/// Item to be produced.
#[derive(Clone, Debug)]
pub struct Item {
    /// Dense index in the owning instance's item list.
    pub(crate) idx: usize,
    /// Original caller-supplied identifier, preserved on export.
    pub(crate) external_id: u64,
    /// Original contour of the item as defined in the input
    pub(crate) shape_orig: Arc<OriginalShape>,
    /// Contour of the item to be used for collision detection
    pub(crate) shape_cd: Arc<SPolygon>,
    /// Allowed rotations and reflections in which the item can be placed
    pub(crate) allowed_orientations: AllowedOrientations,
    /// The minimum quality the item should be produced out of, if `None` the item requires full quality
    pub(crate) min_quality: Option<usize>,
    /// Configuration for the surrogate generation
    pub(crate) surrogate_config: SPSurrogateConfig,
}

impl Item {
    /// Returns an error for invalid geometry or a minimum quality outside `0..N_QUALITIES`.
    pub fn new(
        idx: usize,
        external_id: u64,
        original_shape: OriginalShape,
        allowed_orientations: AllowedOrientations,
        min_quality: Option<usize>,
        surrogate_config: SPSurrogateConfig,
    ) -> Result<Item> {
        ensure!(
            min_quality.is_none_or(|quality| quality < N_QUALITIES),
            "Item minimum quality must be below {N_QUALITIES}"
        );
        let shape_orig = Arc::new(original_shape);
        let shape_int = {
            let mut shape_int = shape_orig.convert_to_internal()?;
            shape_int.generate_surrogate(surrogate_config)?;
            Arc::new(shape_int)
        };
        Ok(Item {
            idx,
            external_id,
            shape_orig,
            shape_cd: shape_int,
            allowed_orientations,
            min_quality,
            surrogate_config,
        })
    }

    #[must_use]
    pub fn area(&self) -> f32 {
        self.shape_orig.area()
    }

    /// Dense index in the owning instance's item list.
    #[must_use]
    pub fn idx(&self) -> usize {
        self.idx
    }

    /// Original caller-supplied identifier, preserved on export.
    #[must_use]
    pub fn external_id(&self) -> u64 {
        self.external_id
    }

    /// Original contour of the item as defined in the input
    #[must_use]
    pub fn shape_orig(&self) -> &Arc<OriginalShape> {
        &self.shape_orig
    }

    /// Contour of the item to be used for collision detection
    #[must_use]
    pub fn shape_cd(&self) -> &Arc<SPolygon> {
        &self.shape_cd
    }

    /// Allowed rotations and reflections in which the item can be placed
    #[must_use]
    pub fn allowed_orientations(&self) -> &AllowedOrientations {
        &self.allowed_orientations
    }

    /// The minimum quality the item should be produced out of, if `None` the item requires full quality
    #[must_use]
    pub fn min_quality(&self) -> Option<usize> {
        self.min_quality
    }

    /// Configuration for the surrogate generation
    #[must_use]
    pub fn surrogate_config(&self) -> SPSurrogateConfig {
        self.surrogate_config
    }
}
