use jagua_rs::collision_detection::hazards::collector::{BasicHazardCollector, HazardCollector};
use jagua_rs::collision_detection::hazards::filter::NoFilter;
use jagua_rs::collision_detection::hazards::{Hazard, HazardEntity};
use jagua_rs::collision_detection::{CDEConfig, CDEngine};
use jagua_rs::entities::{Instance, Item, Layout};
use jagua_rs::geometry::DTransformation;
use jagua_rs::geometry::geo_traits::TransformableFrom;
use jagua_rs::io::import::Importer;
use jagua_rs::probs::spp;
use lbf::io::output::SPOutput;
use lbf::samplers::uniform_rect_sampler::UniformRectSampler;
use rand::SeedableRng;
use rand::prelude::SmallRng;
use serde::Serialize;
use std::env;
use std::fs::File;
use std::path::Path;

const DEPTHS: [u8; 4] = [0, 3, 4, 5];
const N_RANDOM_QUERIES_PER_ITEM: usize = 32;

#[test]
fn collision_trace_is_conservative_across_quadtree_depths() {
    let (instance, layout, config) = frozen_scene();
    let engines = DEPTHS.map(|depth| rebuild_cde(&layout, config, depth));
    let queries = query_corpus(&instance, engines[0].bbox());

    let trace = queries
        .iter()
        .enumerate()
        .map(|(query_id, query)| {
            let item = instance.item(query.item_id);
            let results = engines
                .iter()
                .zip(DEPTHS)
                .map(|(engine, depth)| run_query(engine, item, query.d_transf, depth))
                .collect::<Vec<_>>();

            let depth_zero = &results[0];
            for result in &results[1..] {
                assert!(
                    !depth_zero.collision || result.collision,
                    "depth {} missed depth-zero collision for query {query_id}: {query:?}",
                    result.depth
                );

                let missing_hazards = depth_zero
                    .hazards
                    .iter()
                    .filter(|hazard| !result.hazards.contains(hazard))
                    .collect::<Vec<_>>();
                assert!(
                    missing_hazards.is_empty(),
                    "depth {} missed depth-zero hazards for query {query_id}: {query:?}, missing: {missing_hazards:?}",
                    result.depth
                );
            }

            QueryTrace {
                query_id,
                item_id: query.item_id,
                rotation: query.d_transf.rotation(),
                translation: query.d_transf.translation(),
                results,
            }
        })
        .collect::<Vec<_>>();

    if let Some(path) = env::var_os("JAGUA_COLLISION_TRACE") {
        let path = Path::new(&path);
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).unwrap();
        }
        serde_json::to_writer_pretty(File::create(path).unwrap(), &trace).unwrap();
    }
}

#[test]
fn incremental_updates_match_a_fresh_cde() {
    let (instance, mut layout, config) = frozen_scene();
    let probes = query_corpus(&instance, layout.cde().bbox())
        .into_iter()
        .take(24)
        .collect::<Vec<_>>();
    let keys = layout.placed_items.keys().take(5).collect::<Vec<_>>();
    let mut removed = Vec::with_capacity(keys.len());

    for key in keys {
        removed.push(layout.remove_item(key));
        assert_matches_fresh_cde(&layout, config, &instance, &probes);
    }

    for placed_item in removed {
        let item = instance.item(placed_item.item_id);
        layout.place_item(item, placed_item.d_transf);
        assert_matches_fresh_cde(&layout, config, &instance, &probes);
    }
}

fn frozen_scene() -> (spp::entities::SPInstance, Layout, CDEConfig) {
    let output: SPOutput =
        serde_json::from_str(include_str!("fixtures/swim_oracle_scene.json")).unwrap();
    let config = output.config.cde_config;
    let importer = Importer::new(
        config,
        output.config.poly_simpl_tolerance,
        output.config.min_item_separation,
        output.config.narrow_concavity_cutoff,
    );
    let instance = spp::io::import_instance(&importer, &output.instance).unwrap();
    let solution = spp::io::import_solution(&instance, &output.solution);
    let layout = Layout::from_snapshot(&solution.layout_snapshot);
    (instance, layout, config)
}

fn rebuild_cde(layout: &Layout, mut config: CDEConfig, depth: u8) -> CDEngine {
    config.quadtree_depth = depth;
    CDEngine::new(
        layout.cde().bbox(),
        layout.cde().hazards().cloned().collect(),
        config,
    )
}

fn fresh_cde(layout: &Layout) -> CDEngine {
    let mut cde = layout.container.base_cde.as_ref().clone();
    for (key, placed_item) in &layout.placed_items {
        cde.register_hazard(Hazard::new(
            (key, placed_item).into(),
            placed_item.shape.clone(),
            true,
        ));
    }
    cde
}

fn query_corpus(
    instance: &impl Instance,
    bbox: jagua_rs::geometry::primitives::Rect,
) -> Vec<Query> {
    let mut rng = SmallRng::seed_from_u64(0);
    let mut queries = Vec::new();

    for item in instance.items() {
        let sampler = UniformRectSampler::new(bbox, item);
        queries.extend((0..N_RANDOM_QUERIES_PER_ITEM).map(|_| Query {
            item_id: item.id,
            d_transf: sampler.sample(&mut rng),
        }));
        queries.extend(boundary_queries(item, bbox));
    }

    queries
}

fn boundary_queries(
    item: &Item,
    bbox: jagua_rs::geometry::primitives::Rect,
) -> impl Iterator<Item = Query> {
    let shape_bbox = item.shape_cd.bbox;
    let x_centered =
        f32::midpoint(bbox.x_min, bbox.x_max) - f32::midpoint(shape_bbox.x_min, shape_bbox.x_max);
    let y_centered =
        f32::midpoint(bbox.y_min, bbox.y_max) - f32::midpoint(shape_bbox.y_min, shape_bbox.y_max);
    let boundary_translations = [
        (bbox.x_min - shape_bbox.x_min, y_centered),
        (bbox.x_max - shape_bbox.x_max, y_centered),
        (x_centered, bbox.y_min - shape_bbox.y_min),
        (x_centered, bbox.y_max - shape_bbox.y_max),
    ];

    boundary_translations.into_iter().flat_map(|(x, y)| {
        [(x.next_down(), y), (x, y), (x.next_up(), y)]
            .into_iter()
            .chain([(x, y.next_down()), (x, y.next_up())])
            .map(|translation| Query {
                item_id: item.id,
                d_transf: DTransformation::new(0.0, translation),
            })
    })
}

fn assert_matches_fresh_cde(
    layout: &Layout,
    config: CDEConfig,
    instance: &impl Instance,
    queries: &[Query],
) {
    let fresh = fresh_cde(layout);
    for query in queries {
        let item = instance.item(query.item_id);
        let incremental = run_query(layout.cde(), item, query.d_transf, config.quadtree_depth);
        let rebuilt = run_query(&fresh, item, query.d_transf, config.quadtree_depth);
        assert_eq!(
            incremental, rebuilt,
            "query differs after CDE rebuild: {query:?}"
        );
    }
}

fn run_query(cde: &CDEngine, item: &Item, d_transf: DTransformation, depth: u8) -> QueryResult {
    let transform = d_transf.compose();
    let mut shape = item.shape_cd.as_ref().clone();
    shape.transform_from(&item.shape_cd, &transform);

    let collision =
        cde.detect_surrogate_collision(item.shape_cd.surrogate(), &transform, &NoFilter)
            || cde.detect_poly_collision(&shape, &NoFilter);

    let mut collector = BasicHazardCollector::with_capacity(cde.hazards_map.len());
    cde.collect_surrogate_collisions(item.shape_cd.surrogate(), &transform, &mut collector);
    cde.collect_poly_collisions(&shape, &mut collector);
    let mut hazards = collector
        .entities()
        .map(EntityTrace::from)
        .collect::<Vec<_>>();
    hazards.sort_unstable();

    assert_eq!(collision, !hazards.is_empty());
    QueryResult {
        depth,
        collision,
        hazards,
    }
}

#[derive(Clone, Copy, Debug)]
struct Query {
    item_id: usize,
    d_transf: DTransformation,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
struct QueryResult {
    depth: u8,
    collision: bool,
    hazards: Vec<EntityTrace>,
}

#[derive(Serialize)]
struct QueryTrace {
    query_id: usize,
    item_id: usize,
    rotation: f32,
    translation: (f32, f32),
    results: Vec<QueryResult>,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize)]
enum EntityTrace {
    PlacedItem {
        id: usize,
        rotation_bits: u32,
        translation_bits: (u32, u32),
    },
    Exterior,
    Hole {
        idx: usize,
    },
    InferiorQualityZone {
        quality: usize,
        idx: usize,
    },
}

impl From<&HazardEntity> for EntityTrace {
    fn from(entity: &HazardEntity) -> Self {
        match *entity {
            HazardEntity::PlacedItem { id, dt, pk: _ } => {
                let (x, y) = dt.translation();
                Self::PlacedItem {
                    id,
                    rotation_bits: dt.rotation().to_bits(),
                    translation_bits: (x.to_bits(), y.to_bits()),
                }
            }
            HazardEntity::Exterior => Self::Exterior,
            HazardEntity::Hole { idx } => Self::Hole { idx },
            HazardEntity::InferiorQualityZone { quality, idx } => {
                Self::InferiorQualityZone { quality, idx }
            }
        }
    }
}
