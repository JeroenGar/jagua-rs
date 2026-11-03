use crate::geometry::DTransformation;
use crate::geometry::geo_traits::Transformable;
use crate::geometry::primitives::{Point, Rect, SPolygon};
use crate::geometry::shape_modification::{
    ShapeModifyConfig, ShapeModifyMode, close_narrow_concavities, offset_shape,
    shape_modification_valid, simplify_shape,
};
use crate::io::ext_repr::ExtSPolygon;
use crate::io::import;
use anyhow::Result;

#[derive(Clone, Debug)]
/// A [`SPolygon`] exactly as is defined in the input file
///
/// Also contains all required operations to convert it to a shape that can be used internally.
/// Currently, these are centering and simplification operations, but could be extended in the future.
pub struct OriginalShape {
    pub shape: SPolygon,
    pub pre_transform: DTransformation,
    pub modify_mode: ShapeModifyMode,
    pub modify_config: ShapeModifyConfig,
}

impl OriginalShape {
    pub fn convert_to_internal(&self) -> Result<SPolygon> {
        // Apply the transformation.
        // Rounding to f32 can make distinct vertices coincide after the transformation (see issue #133),
        // so the transformed vertices are passed through the same cleanup and validation as imported ones.
        let transformed = self.shape.transform_clone(&self.pre_transform.compose());
        let mut internal = import::import_simple_polygon(&ExtSPolygon(
            transformed.vertices.iter().map(|p| (p.0, p.1)).collect(),
        ))?;

        if let Some(offset) = self.modify_config.offset {
            // Offset the shape
            if offset != 0.0 {
                internal = offset_shape(&internal, self.modify_mode, offset)?;
            }
        }
        if let Some(tolerance) = self.modify_config.simplify_tolerance {
            let pre_simplified = internal.clone();
            // Simplify the shape
            internal = simplify_shape(&internal, self.modify_mode, tolerance);
            if let Some(narrow_concavity_cutoff) = self.modify_config.narrow_concavity_cutoff {
                // Close narrow concavities
                internal =
                    close_narrow_concavities(&internal, self.modify_mode, narrow_concavity_cutoff);
                // Do another simplification after closing concavities
                internal = simplify_shape(&internal, self.modify_mode, tolerance / 10.0);
            }
            debug_assert!(shape_modification_valid(
                &pre_simplified,
                &internal,
                self.modify_mode
            ));
        }

        Ok(internal)
    }

    #[must_use]
    pub fn centroid(&self) -> Point {
        self.shape.centroid()
    }

    #[must_use]
    pub fn area(&self) -> f32 {
        self.shape.area
    }

    #[must_use]
    pub fn bbox(&self) -> Rect {
        self.shape.bbox
    }

    #[must_use]
    pub fn diameter(&self) -> f32 {
        self.shape.diameter
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::io::import::{centering_transformation, import_simple_polygon};

    #[test]
    fn centering_does_not_collapse_vertices() {
        // Polygon from issue #133. The vertices at index 8 and 9 are distinct in f32,
        // but become identical once the shape is translated to its centroid.
        let points = vec![
            (10.5, 6.204_517),
            (3.930_766e-16, 6.419_428),
            (-33.442_33, 5.734_94),
            (-47.662_89, 4.766_93),
            (-59.808_86, 3.290_652),
            (-69.298_87, 1.320_259),
            (-75.391_26, -0.889_303),
            (-77.600_12, -2.461_642),
            (-77.686_46, -2.777_069),
            (-77.686_46, -2.777_068),
            (-80.021_02, -11.306_129),
            (-88.827_56, -10.450_826),
            (-203.632_35, 0.699_174),
            (-220.234_46, 2.311_593),
            (-211.601_77, 16.584_175),
            (-194.721_86, 44.492_01),
            (-187.869_04, 57.095_108),
            (-183.597_82, 67.951_93),
            (-182.840_06, 71.706_25),
            (-182.800_17, 74.573_4),
            (-183.2407, 76.380_84),
            (-184.061_2, 77.732_344),
            (-185.847_34, 79.293_46),
            (-189.443_63, 81.091_91),
            (-195.307_63, 82.745_61),
            (-204.078_86, 83.978_09),
            (-226.052_1, 87.065_65),
            (-226.783_56, 90.060_72),
            (-208.706_9, 102.929_69),
            (-182.091_23, 121.877_63),
            (-180.688_03, 127.232_44),
            (-179.573_62, 135.095_55),
            (-179.523_21, 142.374_56),
            (-180.492_94, 149.230_05),
            (-182.492_9, 155.858_63),
            (-185.596_57, 162.443),
            (-189.911_15, 169.110_44),
            (-195.857_58, 176.317_67),
            (-199.083_3, 180.227_33),
            (-196.657_06, 191.631_46),
            (-190.255_38, 193.198_78),
            (-26.876_544, 233.198_78),
            (-18.457_486, 235.260_02),
            (-13.992_686, 225.543_02),
            (-13.892_675, 223.5243),
            (-13.421_246, 214.008_47),
            (-12.161_206, 205.7295),
            (-10.219_268, 198.415_95),
            (-7.773_85, 192.343_95),
            (-5.091_749, 187.800_5),
            (-2.598_881, 185.016_36),
            (-0.863_646, 183.905_51),
            (1.124_994_5e-14, 183.725_54),
            (10.5, 185.913_53),
        ];
        let shape = import_simple_polygon(&ExtSPolygon(points)).unwrap();
        let original = OriginalShape {
            pre_transform: centering_transformation(&shape),
            shape,
            modify_mode: ShapeModifyMode::Inflate,
            modify_config: ShapeModifyConfig {
                simplify_tolerance: Some(0.001),
                offset: None,
                narrow_concavity_cutoff: Some((0.01, 0.01)),
            },
        };

        let internal = original.convert_to_internal().unwrap();

        let centered = original
            .shape
            .transform_clone(&original.pre_transform.compose());
        assert!(shape_modification_valid(
            &centered,
            &internal,
            ShapeModifyMode::Inflate
        ));
    }
}
