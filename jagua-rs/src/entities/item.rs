use std::sync::Arc;

use crate::geometry::OriginalShape;
use crate::geometry::fail_fast::SPSurrogateConfig;
use crate::geometry::geo_enums::AllowedRotation;
use crate::geometry::primitives::SPolygon;

use anyhow::Result;

/// Item to be produced.
#[derive(Clone, Debug)]
pub struct Item {
    pub id: usize,
    /// Original contour of the item as defined in the input
    pub shape_orig: Arc<OriginalShape>,
    /// Contour of the item to be used for collision detection
    pub shape_cd: Arc<SPolygon>,
    /// Defines how the item can be rotated during placement
    pub allowed_rotations: Vec<AllowedRotation>,
    /// The minimum quality the item should be produced out of, if `None` the item requires full quality
    pub min_quality: Option<usize>,
    /// Configuration for the surrogate generation
    pub surrogate_config: SPSurrogateConfig,
}

impl Item {
    pub fn new(
        id: usize,
        original_shape: OriginalShape,
        allowed_rotations: Vec<AllowedRotation>,
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
            id,
            shape_orig,
            shape_cd: shape_int,
            allowed_rotations,
            min_quality,
            surrogate_config,
        })
    }

    pub fn area(&self) -> f32 {
        self.shape_orig.area()
    }
}
