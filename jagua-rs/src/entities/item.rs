use crate::collision_detection::hazards::filter::QZHazardFilter;
use crate::geometry::OriginalShape;
use crate::geometry::fail_fast::SPSurrogateConfig;
use crate::geometry::geo_enums::RotationRange;
use crate::geometry::primitives::SPolygon;

use anyhow::Result;

/// Item to be produced.
#[derive(Clone, Debug)]
pub struct Item {
    pub id: usize,
    /// Original contour of the item as defined in the input
    pub shape_orig: OriginalShape,
    /// Contour of the item to be used for collision detection
    pub shape_cd: SPolygon,
    /// Allowed rotations in which the item can be placed
    pub allowed_rotation: RotationRange,
    /// The minimum quality the item should be produced out of, if `None` the item requires full quality
    pub min_quality: Option<usize>,
    /// Filter for hazards that the item is unaffected by
    pub hazard_filter: Option<QZHazardFilter>,
    /// Configuration for the surrogate generation
    pub surrogate_config: SPSurrogateConfig,
}

impl Item {
    pub fn new(
        id: usize,
        original_shape: OriginalShape,
        allowed_rotation: RotationRange,
        base_quality: Option<usize>,
        surrogate_config: SPSurrogateConfig,
    ) -> Result<Item> {
        let mut shape_cd = original_shape.convert_to_internal()?;
        shape_cd.generate_surrogate(surrogate_config)?;

        let hazard_filter = base_quality.map(QZHazardFilter);

        Ok(Item {
            id,
            shape_orig: original_shape,
            shape_cd,
            allowed_rotation,
            min_quality: base_quality,
            hazard_filter,
            surrogate_config,
        })
    }

    pub fn area(&self) -> f32 {
        self.shape_orig.area()
    }
}
