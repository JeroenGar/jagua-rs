use crate::collision_detection::CDEConfig;
use crate::entities::Container;
use crate::geometry::primitives::{Rect, SPolygon};
use crate::geometry::shape_modification::{ShapeModifyConfig, ShapeModifyMode};
use crate::geometry::{DTransformation, OriginalShape};
use anyhow::{Result, ensure};

#[derive(Clone, Debug, Copy, PartialEq)]
/// Represents a rectangular container with fixed width and a variable height between \]0,max_height\].
/// Can be converted into a [`Container`] for use in layouts.
pub struct Strip {
    pub max_height: f32,
    pub fixed_width: f32,
    pub cde_config: CDEConfig,
    pub shape_modify_config: ShapeModifyConfig,
    pub height: f32,
}

impl Strip {
    pub fn new(
        max_height: f32,
        fixed_width: f32,
        cde_config: CDEConfig,
        shape_modify_config: ShapeModifyConfig,
        height: f32,
    ) -> Result<Self> {
        ensure!(fixed_width > 0.0, "strip width must be positive");
        ensure!(max_height > 0.0, "strip maximum height must be positive");
        ensure!(height > 0.0, "strip height must be positive");
        Ok(Strip {
            max_height,
            fixed_width,
            cde_config,
            shape_modify_config,
            height,
        })
    }

    pub fn set_height(&mut self, height: f32) {
        assert!(height <= self.max_height, "strip height exceeds maximum height");
        assert!(height > 0.0, "strip height must be positive");
        self.height = height;
    }
}

impl From<Strip> for Container {
    fn from(s: Strip) -> Container {
        let id = s.height.to_bits() as usize;
        Container::new(
            id,
            OriginalShape {
                shape: SPolygon::from(Rect::try_new(0.0, 0.0, s.fixed_width, s.height).unwrap()),
                pre_transform: DTransformation::empty(),
                modify_mode: ShapeModifyMode::Deflate,
                modify_config: s.shape_modify_config,
            },
            vec![],
            s.cde_config,
        )
        .unwrap()
    }
}
