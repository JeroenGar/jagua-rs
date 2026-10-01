use std::sync::Arc;

use crate::geometry::fail_fast::SPSurrogateConfig;
use crate::geometry::primitives::SPolygon;
use crate::geometry::{AllowedOrientations, OriginalShape};

use anyhow::Result;

/// Item to be produced.
#[derive(Clone, Debug)]
pub struct Item {
    /// Dense index in the owning instance's item list.
    pub idx: usize,
    /// Original caller-supplied identifier, preserved on export.
    pub external_id: u64,
    /// Original contour of the item as defined in the input
    pub shape_orig: Arc<OriginalShape>,
    /// Contour of the item to be used for collision detection
    pub shape_cd: Arc<SPolygon>,
    /// Allowed rotations and reflections in which the item can be placed
    pub allowed_orientations: AllowedOrientations,
    /// The minimum quality the item should be produced out of, if `None` the item requires full quality
    pub min_quality: Option<usize>,
    /// Configuration for the surrogate generation
    pub surrogate_config: SPSurrogateConfig,
}

impl Item {
    pub fn new(
        idx: usize,
        external_id: u64,
        original_shape: OriginalShape,
        allowed_orientations: AllowedOrientations,
        min_quality: Option<usize>,
        surrogate_config: SPSurrogateConfig,
    ) -> Result<Item> {
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
}
