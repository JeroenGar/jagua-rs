use std::sync::Arc;

use itertools::Itertools;

use crate::collision_detection::CDEngine;
use crate::collision_detection::hazards::Hazard;
use crate::collision_detection::hazards::HazardEntity;
use crate::fsize;
use crate::geometry::Transformation;
use crate::geometry::geo_traits::Shape;
use crate::geometry::primitives::AARectangle;
use crate::geometry::primitives::SimplePolygon;
use crate::util::config::CDEConfig;

#[cfg(doc)]
use crate::entities::general::Item;

/// A container in which [`Item`]'s can be placed.
#[derive(Clone, Debug)]
pub struct Bin {
    pub id: usize,
    /// The contour of the bin
    pub outer: Arc<SimplePolygon>,
    /// The cost of using the bin
    pub value: u64,
    /// Transformation applied to the shape with respect to the original shape in the input file (for example to center it).
    pub pretransform: Transformation,
    /// Shapes of holes/defects in the bins, if any
    pub holes: Vec<Arc<SimplePolygon>>,
    /// Zones of different qualities in the bin, stored per quality.
    pub quality_zones: [Option<InferiorQualityZone>; N_QUALITIES],
    /// The starting state of the `CDEngine` for this bin.
    pub base_cde: Arc<CDEngine>,
    pub area: fsize,
}

impl Bin {
    pub fn new(
        id: usize,
        outer: SimplePolygon,
        value: u64,
        pretransform: Transformation,
        holes: Vec<SimplePolygon>,
        quality_zones: Vec<InferiorQualityZone>,
        cde_config: CDEConfig,
    ) -> Self {
        let outer = Arc::new(outer);
        let holes = holes.into_iter().map(Arc::new).collect_vec();
        assert_eq!(
            quality_zones.len(),
            quality_zones.iter().map(|qz| qz.quality).unique().count(),
            "Quality zones must have unique qualities"
        );
        assert!(
            quality_zones
                .iter()
                .map(|qz| qz.quality)
                .all(|q| q < N_QUALITIES),
            "All quality zones must be below N_QUALITIES"
        );
        let quality_zones = {
            let mut qz = <[_; N_QUALITIES]>::default();
            for q in quality_zones {
                let quality = q.quality;
                qz[quality] = Some(q);
            }
            qz
        };

        let bin_hazards = generate_bin_hazards(&outer, &holes, &quality_zones);

        let base_cde = CDEngine::new(outer.bbox().inflate_to_square(), bin_hazards, cde_config);
        let base_cde = Arc::new(base_cde);
        let area = outer.area() - holes.iter().map(|h| h.area()).sum::<fsize>();

        Self {
            id,
            outer,
            value,
            pretransform,
            holes,
            quality_zones,
            base_cde,
            area,
        }
    }

    /// Create a new `Bin` for a strip-packing problem. Instead of a shape, the bin is always rectangular.
    pub fn from_strip(id: usize, rect: AARectangle, cde_config: CDEConfig) -> Self {
        //The "original" x_min and y_min of the strip should always be at (0, 0)
        let pretransform = Transformation::from_translation((rect.x_min, rect.y_min));

        let poly = SimplePolygon::from(rect);
        let value = poly.area() as u64;

        Bin::new(id, poly, value, pretransform, vec![], vec![], cde_config)
    }

    pub fn bbox(&self) -> AARectangle {
        self.outer.bbox()
    }
}

fn generate_bin_hazards(
    outer: &Arc<SimplePolygon>,
    holes: &[Arc<SimplePolygon>],
    quality_zones: &[Option<InferiorQualityZone>],
) -> Vec<Hazard> {
    //Hazard induced by the outside of the bin
    let mut hazards = vec![Hazard::new(HazardEntity::BinExterior, outer.clone())];

    //Hazard induced by any holes in the bin
    hazards.extend(holes.iter().enumerate().map(|(i, shape)| {
        let haz_entity = HazardEntity::BinHole { id: i };
        Hazard::new(haz_entity, shape.clone())
    }));

    //Hazards induced by quality zones
    for q_zone in quality_zones.iter().flatten() {
        for (id, shape) in q_zone.zones.iter().enumerate() {
            let haz_entity = HazardEntity::InferiorQualityZone {
                quality: q_zone.quality,
                id,
            };
            hazards.push(Hazard::new(haz_entity, shape.clone()));
        }
    }
    hazards
}

/// Maximum number of qualities that can be used for quality zones in a bin.
pub const N_QUALITIES: usize = 10;

/// Represents a zone of inferior quality in the [`Bin`]
#[derive(Clone, Debug)]
pub struct InferiorQualityZone {
    /// Quality of this zone. Higher qualities are superior.
    pub quality: usize,
    /// The shapes of all zones of this quality
    pub zones: Vec<Arc<SimplePolygon>>,
}

impl InferiorQualityZone {
    pub fn new(quality: usize, shapes: Vec<SimplePolygon>) -> Self {
        assert!(
            quality < N_QUALITIES,
            "Quality must be in range of N_QUALITIES"
        );
        let zones = shapes.into_iter().map(Arc::new).collect();
        Self { quality, zones }
    }
}
