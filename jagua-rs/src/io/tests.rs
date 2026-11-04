//! Tests of the io module

mod import {
    use crate::collision_detection::CDEConfig;
    use crate::geometry::fail_fast::SPSurrogateConfig;
    use crate::io::ext_repr::{ExtContainer, ExtShape};
    use crate::io::import::Importer;

    fn importer() -> Importer {
        let cde_config = CDEConfig {
            item_surrogate_config: SPSurrogateConfig {
                n_pole_limits: [(100, 0.0), (20, 0.75), (10, 0.90)],
                ff_pole_area_ratio: 0.5,
            },
        };
        Importer::new(cde_config, None, None)
    }

    fn rectangle_container(width: f32, height: f32) -> ExtContainer {
        ExtContainer {
            id: 0,
            shape: ExtShape::Rectangle {
                x_min: 0.0,
                y_min: 0.0,
                width,
                height,
            },
            zones: vec![],
        }
    }

    #[test]
    fn too_thin_rectangle_is_an_error() {
        let importer = importer();
        assert!(
            importer
                .import_container(&rectangle_container(10.0, 5.0))
                .is_ok()
        );
        // the short sides are below the minimum edge length
        assert!(
            importer
                .import_container(&rectangle_container(10.0, 1e-6))
                .is_err()
        );
    }
}
