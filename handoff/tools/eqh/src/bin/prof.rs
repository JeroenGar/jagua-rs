//! Profiling driver: replays the ci_bench collect and detect loops for a fixed number of queries.
//! Usage: prof <instance> <collect|detect> <depth> <n_queries>
use jagua_rs::collision_detection::hazards::collector::BasicHazardCollector;
use jagua_rs::collision_detection::hazards::filter::NoFilter;
use jagua_rs::geometry::geo_traits::TransformableFrom;
use jagua_rs::io::import::Importer;
use jagua_rs::probs::spp;
use lbf::config::LBFConfig;
use lbf::opt::lbf_spp::LBFOptimizerSP;
use lbf::samplers::uniform_rect_sampler::UniformRectSampler;
use rand::SeedableRng;
use rand::prelude::{IteratorRandom, SmallRng};
use std::hint::black_box;
use std::path::Path;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let (path, mode, depth, n) = (
        &args[1],
        args[2].as_str(),
        args[3].parse::<u8>().unwrap(),
        args[4].parse::<usize>().unwrap(),
    );
    let mut config = LBFConfig::default();
    config.cde_config.quadtree_depth = depth;
    let ext = lbf::io::read_spp_instance(Path::new(path)).unwrap();
    let importer = Importer::new(config.cde_config, config.poly_simpl_tolerance, None);
    let instance = spp::io::import_instance(&importer, &ext).unwrap();
    let mut opt = LBFOptimizerSP::new(instance, config, SmallRng::seed_from_u64(0)).unwrap();
    opt.solve().unwrap();
    let mut problem = opt.problem;
    let mut rng = SmallRng::seed_from_u64(0);
    if mode == "detect" {
        for _ in 0..5 {
            let pkey = problem.layout().placed_items().keys().choose(&mut rng).unwrap();
            problem.remove_item(pkey);
        }
    }
    let cde = problem.layout().cde();
    let mut count = 0usize;
    if mode == "feasible" {
        // Detect each placed item at its own (collision-free) position, ignoring its own hazard
        let items: Vec<_> = problem.layout().placed_items().iter().map(|(pk, pi)| (cde.haz_key(&(pk, pi).into()).unwrap(), pi.shape().clone(), pi.item().shape_cd().surrogate().clone(), pi.d_transf().compose())).collect();
        let t0 = std::time::Instant::now();
        for q in 0..n {
            let (hk, shape, sur, t) = &items[q % items.len()];
            if !cde.detect_surrogate_collision(sur, t, hk) && !cde.detect_poly_collision(shape, hk) { count += 1; }
        }
        let dt = t0.elapsed();
        println!("{} items={} us_per_query={:.3}", black_box(count), items.len(), dt.as_secs_f64() * 1e6 / n as f64);
        return;
    }
    let t0 = std::time::Instant::now();
    for _ in 0..n / 1000 {
        let (_, pi) = problem.layout().placed_items().iter().choose(&mut rng).unwrap();
        let item = pi.item();
        let mut shape = item.shape_cd().as_ref().clone();
        let mut collector = BasicHazardCollector::with_capacity(cde.hazards_map().len());
        let sampler = UniformRectSampler::new(cde.bbox(), item);
        for _ in 0..1000 {
            let transf = sampler.sample(&mut rng).compose();
            if mode == "collect" {
                shape.transform_from(item.shape_cd(), &transf);
                cde.collect_surrogate_collisions(&shape, &NoFilter, &mut collector);
                cde.collect_poly_collisions(&shape, &NoFilter, &mut collector);
                count += collector.len();
                collector.clear();
            } else if !cde.detect_surrogate_collision(item.shape_cd().surrogate(), &transf, &NoFilter) {
                shape.transform_from(item.shape_cd(), &transf);
                if !cde.detect_poly_collision(&shape, &NoFilter) {
                    count += 1;
                }
            }
        }
    }
    let dt = t0.elapsed();
    println!("{} items={} us_per_query={:.3}", black_box(count), problem.layout().placed_items().len(), dt.as_secs_f64() * 1e6 / n as f64);
}
