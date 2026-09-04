use jagua_rs::collision_detection::CDEngine;
use jagua_rs::collision_detection::hazards::collector::{BasicHazardCollector, HazardCollector};
use jagua_rs::entities::Instance;
use jagua_rs::geometry::geo_traits::TransformableFrom;
use lbf::samplers::uniform_rect_sampler::UniformRectSampler;
use rand::SeedableRng;
use rand::prelude::{IteratorRandom, SmallRng};
use std::collections::HashSet;

#[path = "../benches/util.rs"]
mod util;

#[test]
fn collection_matches_flat_tree_on_ci_workload() {
    let config = util::create_base_config();
    let instance = util::create_instance(config.cde_config, config.poly_simpl_tolerance);
    for removed in [0, util::N_ITEMS_REMOVED] {
        let (problem, _) = util::create_lbf_problem(instance.clone(), config, removed);
        let cde = problem.layout.cde();
        let flat = CDEngine::new(
            cde.bbox(),
            cde.hazards().cloned().collect(),
            jagua_rs::collision_detection::CDEConfig {
                quadtree_depth: 0,
                ..config.cde_config
            },
        );

        for threshold in [0, config.cde_config.cd_threshold] {
            let tree = CDEngine::new(
                cde.bbox(),
                cde.hazards().cloned().collect(),
                jagua_rs::collision_detection::CDEConfig {
                    quadtree_depth: 4,
                    cd_threshold: threshold,
                    ..config.cde_config
                },
            );
            let mut rng = SmallRng::seed_from_u64(86);
            let mut expected = BasicHazardCollector::new();
            let mut actual = BasicHazardCollector::new();
            for sample in 0..1000 {
                let (_, placed) = problem.layout.placed_items.iter().choose(&mut rng).unwrap();
                let item = instance.item(placed.item_id);
                let sampler = UniformRectSampler::new(cde.bbox(), item);
                let mut shape = item.shape_cd.as_ref().clone();
                shape.transform_from(&item.shape_cd, &sampler.sample(&mut rng).compose());
                flat.collect_poly_collisions(&shape, &mut expected);
                tree.collect_surrogate_collisions(&shape, &mut actual);
                tree.collect_poly_collisions(&shape, &mut actual);
                assert_eq!(
                    actual.entities().copied().collect::<HashSet<_>>(),
                    expected.entities().copied().collect::<HashSet<_>>(),
                    "sample {sample}, threshold {threshold}",
                );
                // Already-collected hazards must not invoke the callback again.
                assert!(
                    !tree.collect_poly_collisions_until(&shape, &mut actual, |_| {
                        panic!("duplicate collision callback")
                    })
                );
                expected.clear();
                actual.clear();
            }
        }
    }
}
