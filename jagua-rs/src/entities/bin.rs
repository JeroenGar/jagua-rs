use std::sync::Arc;

use itertools::Itertools;

use crate::collision_detection::cd_engine::CDEngine;
use crate::collision_detection::hazard::Hazard;
use crate::collision_detection::hazard::HazardEntity;
use crate::entities::quality_zone::N_QUALITIES;
use crate::entities::quality_zone::QualityZone;
use crate::geometry::geo_traits::Shape;
use crate::geometry::primitives::aa_rectangle::AARectangle;
use crate::geometry::primitives::simple_polygon::SimplePolygon;
use crate::geometry::transformation::Transformation;
use crate::util::config::CDEConfig;

/// A `Bin` is a container in which items can be placed.
/// Its interior is defined by an outer polygon and zero or more holes.
#[derive(Clone, Debug)]
pub struct Bin {
    pub id: usize,
    pub outer: Arc<SimplePolygon>,
    pub value: u64,
    pub centering_transform: Transformation,
    pub holes: Vec<Arc<SimplePolygon>>,
    pub quality_zones: [Option<QualityZone>; N_QUALITIES],
    pub base_cde: Arc<CDEngine>,
    pub area: f64,
}

impl Bin {
    pub fn new(id: usize, outer: SimplePolygon, value: u64, centering_transform: Transformation, holes: Vec<SimplePolygon>, quality_zones: Vec<QualityZone>, cde_config: CDEConfig) -> Self {
        let outer = Arc::new(outer);
        let holes = holes.into_iter().map(|z| Arc::new(z)).collect_vec();
        assert_eq!(quality_zones.len(), quality_zones.iter().map(|qz| qz.quality).unique().count(), "Quality zones must have unique qualities");
        assert!(quality_zones.iter().map(|qz| qz.quality).all(|q| q < N_QUALITIES), "All quality zones must be below N_QUALITIES");
        let quality_zones = {
            let mut qz = <[_; N_QUALITIES]>::default();
            for q in quality_zones {
                let quality = q.quality;
                qz[quality] = Some(q);
            }
            qz
        };

        let base_cde = CDEngine::new(
            outer.bbox().inflate_to_square(),
            Self::generate_hazards(&outer, &holes, &quality_zones),
            cde_config,
        );
        let base_cde = Arc::new(base_cde);
        let area = outer.area() - holes.iter().map(|h| h.area()).sum::<f64>();

        Self {
            id,
            outer,
            value,
            centering_transform,
            holes,
            quality_zones,
            base_cde,
            area,
        }
    }

    pub fn from_strip(id: usize, width: f64, height: f64, cde_config: CDEConfig) -> Self {
        let poly = SimplePolygon::from(AARectangle::new(0.0, 0.0, width, height));
        let value = poly.area() as u64;

        Bin::new(id, poly, value, Transformation::empty(), vec![], vec![], cde_config)
    }
    fn generate_hazards(outer: &Arc<SimplePolygon>, holes: &[Arc<SimplePolygon>], quality_zones: &[Option<QualityZone>]) -> Vec<Hazard> {
        let mut hazards = vec![
            Hazard::new(HazardEntity::BinExterior, outer.clone())
        ];
        hazards.extend(
            holes.iter().enumerate()
                .map(|(i, shape)| {
                    let haz_entity = HazardEntity::BinHole { id: i };
                    Hazard::new(haz_entity, shape.clone())
                })
        );

        hazards.extend(
            quality_zones.iter().flatten().map(
                |q_zone| {
                    q_zone.zones.iter().enumerate().map(|(id, shape)| {
                        let haz_entity = HazardEntity::QualityZoneInferior {
                            quality: q_zone.quality,
                            id,
                        };
                        Hazard::new(haz_entity, shape.clone())
                    })
                }
            ).flatten()
        );

        hazards
    }

    pub fn bbox(&self) -> AARectangle {
        self.outer.bbox()
    }
}