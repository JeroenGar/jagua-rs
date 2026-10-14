//! Equivalence harness for bin packing (irregular bins): collect and detect queries on LBF layouts.
use jagua_rs::collision_detection::hazards::collector::BasicHazardCollector;
use jagua_rs::collision_detection::hazards::filter::NoFilter;
use jagua_rs::collision_detection::hazards::BasicHazardEntity;
use jagua_rs::geometry::geo_traits::TransformableFrom;
use jagua_rs::io::import::Importer;
use jagua_rs::probs::bpp;
use lbf::config::LBFConfig;
use lbf::opt::lbf_bpp::LBFOptimizerBP;
use lbf::samplers::uniform_rect_sampler::UniformRectSampler;
use rand::prelude::{IteratorRandom, SmallRng};
use rand::SeedableRng;
use std::io::{BufWriter, Write};
use std::path::Path;

include!("../oracle_fns.rs");

fn main() {
    let path = std::env::args().nth(1).unwrap();
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    for depth in [3u8, 4, 5] {
        let mut config = LBFConfig::default();
        config.cde_config.quadtree_depth = depth;
        let ext = lbf::io::read_bpp_instance(Path::new(&path)).unwrap();
        let importer = Importer::new(config.cde_config, config.poly_simpl_tolerance, None);
        let instance = bpp::io::import_instance(&importer, &ext).unwrap();
        let mut opt = LBFOptimizerBP::new(instance.clone(), config, SmallRng::seed_from_u64(0));
        opt.solve();
        let problem = opt.problem;
        let mut rng = SmallRng::seed_from_u64(depth as u64);
        writeln!(out, "run depth={depth} layouts={}", problem.layouts().len()).unwrap();
        for q in 0..20_000 {
            let (_, layout) = problem.layouts().iter().choose(&mut rng).unwrap();
            let cde = layout.cde();
            let (item, _) = instance.items.iter().choose(&mut rng).unwrap();
            let sampler = UniformRectSampler::new(cde.bbox(), item);
            let transf = sampler.sample(&mut rng).compose();
            let mut shape = item.shape_cd().as_ref().clone();
            shape.transform_from(item.shape_cd(), &transf);
            let mut col = BasicHazardCollector::with_capacity(cde.hazards_map().len());
            cde.collect_surrogate_collisions(&shape, &NoFilter, &mut col);
            cde.collect_poly_collisions(&shape, &NoFilter, &mut col);
            let mut all: Vec<_> = col.iter().map(|(k, _)| k).collect();
            all.sort();
            let dp = cde.detect_poly_collision(&shape, &NoFilter);
            let mut line = format!("{q} s=[] a={all:?} ds=false dp={dp}");
            for k in &all {
                let h = cde.hazard(*k).unwrap();
                let exterior = matches!(h.entity, BasicHazardEntity::Exterior);
                line += &format!(" | {k:?} {}", oracle(shape.vertices(), h.shape.vertices(), exterior));
            }
            // Hazards the oracle sees colliding but the collector missed
            let missed = cde.hazards_map().iter().filter(|(k, h)| !all.contains(k) && {
                let exterior = matches!(h.entity, BasicHazardEntity::Exterior);
                oracle(shape.vertices(), h.shape.vertices(), exterior) == "hit"
            }).count();
            line += &format!(" || missed_by_collect={missed}");
            writeln!(out, "{line}").unwrap();
        }
    }
}
