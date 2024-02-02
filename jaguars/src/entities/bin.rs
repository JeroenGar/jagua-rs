use std::sync::Arc;

use itertools::Itertools;

use crate::collision_detection::cd_engine::CDEngine;
use crate::collision_detection::hazard::Hazard;
use crate::collision_detection::hazard::HazardEntity;
use crate::entities::quality_zone::QualityZone;
use crate::geometry::geo_traits::Shape;
use crate::geometry::primitives::aa_rectangle::AARectangle;
use crate::geometry::primitives::simple_polygon::SimplePolygon;
use crate::geometry::transformation::Transformation;
use crate::N_QUALITIES;
use crate::util::config::CDEConfig;

//TODO: Add base quality to bins

#[derive(Clone, Debug)]
pub struct Bin {
    id: usize,
    outer: Arc<SimplePolygon>,
    value: u64,
    centering_transform: Transformation,
    holes: Vec<Arc<SimplePolygon>>,
    quality_zones: [Option<QualityZone>; N_QUALITIES],
    base_cde: Arc<CDEngine>,
    area: f64,
}

impl Bin {
    pub fn new(id: usize, outer: SimplePolygon, value: u64, centering_transform: Transformation, holes: Vec<SimplePolygon>, quality_zones: Vec<QualityZone>, cde_config: CDEConfig) -> Self {
        let outer = Arc::new(outer);
        let holes = holes.into_iter().map(|z| Arc::new(z)).collect_vec();
        assert_eq!(quality_zones.len(), quality_zones.iter().map(|qz| qz.quality()).unique().count(), "Quality zones must have unique qualities");
        assert!(quality_zones.iter().map(|qz| qz.quality()).all(|q| q < N_QUALITIES), "All quality zones must be below N_QUALITIES");
        let quality_zones = {
            let mut qz = <[_; N_QUALITIES]>::default();
            for q in quality_zones {
                let quality = q.quality();
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
        //TODO: move this out of here
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
                    q_zone.shapes().iter().enumerate().map(|(id, shape)| {
                        let haz_entity = HazardEntity::QualityZoneInferior {
                            quality: q_zone.quality(),
                            id,
                        };
                        Hazard::new(haz_entity, shape.clone())
                    })
                }
            ).flatten()
        );

        hazards
    }

    pub fn id(&self) -> usize {
        self.id
    }

    pub fn outer(&self) -> &Arc<SimplePolygon> {
        &self.outer
    }

    pub fn holes(&self) -> &Vec<Arc<SimplePolygon>> {
        &self.holes
    }

    pub fn bbox(&self) -> AARectangle {
        self.outer.bbox()
    }

    pub fn value(&self) -> u64 {
        self.value
    }

    pub fn centering_transform(&self) -> &Transformation {
        &self.centering_transform
    }

    pub fn quality_zones(&self) -> &[Option<QualityZone>; N_QUALITIES] {
        &self.quality_zones
    }

    pub fn base_cde(&self) -> &CDEngine {
        &self.base_cde
    }

    pub fn area(&self) -> f64 {
        self.area
    }
}