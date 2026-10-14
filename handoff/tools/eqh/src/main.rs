//! Equivalence harness: prints the result of many seeded CDE queries, one line each.
//! Run against two builds of jagua-rs and diff the outputs.
use jagua_rs::collision_detection::hazards::collector::BasicHazardCollector;
use jagua_rs::collision_detection::hazards::filter::{HazKeyFilter, HazardFilter, NoFilter};
use jagua_rs::collision_detection::hazards::{BasicHazardEntity, HazKey};
use jagua_rs::geometry::geo_traits::TransformableFrom;
use jagua_rs::io::import::Importer;
use jagua_rs::probs::spp;
use jagua_rs::probs::spp::entities::{SPPlacement, SPProblem};
use lbf::config::LBFConfig;
use lbf::opt::lbf_spp::LBFOptimizerSP;
use lbf::samplers::uniform_rect_sampler::UniformRectSampler;
use rand::prelude::{IteratorRandom, SmallRng};
use rand::{RngExt, SeedableRng};
use std::io::{BufWriter, Write};
use std::path::Path;

const QUERIES_PER_RUN: usize = 17_000;

fn main() {
    let path = std::env::args().nth(1).expect("instance path");
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    for depth in [3u8, 4, 5] {
        for n_removed in [0usize, 5] {
            run(&path, depth, n_removed, &mut out);
        }
    }
}

fn run(path: &str, depth: u8, n_removed: usize, out: &mut impl Write) {
    let mut config = LBFConfig::default();
    config.cde_config.quadtree_depth = depth;
    let ext = lbf::io::read_spp_instance(Path::new(path)).unwrap();
    let importer = Importer::new(config.cde_config, config.poly_simpl_tolerance, None);
    let instance = spp::io::import_instance(&importer, &ext).unwrap();
    let mut opt = LBFOptimizerSP::new(instance, config, SmallRng::seed_from_u64(0)).unwrap();
    opt.solve().unwrap();
    let mut problem = opt.problem;
    let mut rng = SmallRng::seed_from_u64(depth as u64 * 1000 + n_removed as u64);
    for _ in 0..n_removed {
        let pkey = problem.layout().placed_items().keys().choose(&mut rng).unwrap();
        problem.remove_item(pkey);
    }
    writeln!(out, "run depth={depth} removed={n_removed} placed={}", problem.n_placed_items()).unwrap();
    let mut saved = problem.save();

    for q in 0..QUERIES_PER_RUN {
        // Mutate the layout every so often: move an item to a random pose
        if q % 25 == 24 {
            let pkey = problem.layout().placed_items().keys().choose(&mut rng).unwrap();
            let pl = problem.remove_item(pkey);
            let item = problem.instance().item(pl.item_idx).clone();
            let sampler = UniformRectSampler::new(problem.layout().cde().bbox(), &item);
            let d_transf = sampler.sample(&mut rng);
            problem.place_item(SPPlacement { item_idx: pl.item_idx, d_transf });
        }
        if q % 3000 == 1500 {
            saved = problem.save();
        }
        if q % 3000 == 2999 {
            problem.restore(&saved);
        }
        if q % 5000 == 4999 {
            let w = problem.strip_width() * rng.random_range(0.9..1.1f32);
            let ok = problem.change_strip_width(w).is_ok();
            writeln!(out, "width {w} {ok}").unwrap();
        }

        let cde = problem.layout().cde();
        let (_, pi) = problem.layout().placed_items().iter().choose(&mut rng).unwrap();
        let item = pi.item().clone();
        let sampler = UniformRectSampler::new(cde.bbox(), &item);
        let transf = sampler.sample(&mut rng).compose();
        let mut shape = item.shape_cd().as_ref().clone();
        shape.transform_from(item.shape_cd(), &transf);

        let line = if rng.random_bool(0.5) {
            query(&problem, &item, &shape, &transf, &NoFilter)
        } else {
            let keys: Vec<HazKey> = cde.hazards_map().keys().choose_multiple(&mut rng, 3);
            query(&problem, &item, &shape, &transf, &HazKeyFilter::from_keys(keys))
        };
        writeln!(out, "{q} {line}").unwrap();
    }
}

fn query(
    problem: &SPProblem,
    item: &jagua_rs::entities::Item,
    shape: &jagua_rs::geometry::primitives::SPolygon,
    transf: &jagua_rs::geometry::Transformation,
    filter: &impl HazardFilter<BasicHazardEntity>,
) -> String {
    let cde = problem.layout().cde();
    let mut col = BasicHazardCollector::with_capacity(cde.hazards_map().len());
    cde.collect_surrogate_collisions(shape, filter, &mut col);
    let mut sur: Vec<_> = col.iter().map(|(k, _)| k).collect();
    sur.sort();
    cde.collect_poly_collisions(shape, filter, &mut col);
    let mut all: Vec<_> = col.iter().map(|(k, _)| k).collect();
    all.sort();
    let ds = cde.detect_surrogate_collision(item.shape_cd().surrogate(), transf, filter);
    let dp = cde.detect_poly_collision(shape, filter);
    let mut line = format!("s={sur:?} a={all:?} ds={ds} dp={dp}");
    if std::env::var("ORACLE").is_ok() {
        for k in &all {
            let h = cde.hazard(*k).unwrap();
            let exterior = matches!(h.entity, BasicHazardEntity::Exterior);
            line += &format!(" | {k:?} {}", oracle(shape.vertices(), h.shape.vertices(), exterior));
        }
    }
    line
}

type P = (f64, f64);
fn pts(v: &[jagua_rs::geometry::primitives::Point]) -> Vec<P> {
    v.iter().map(|p| (p.0 as f64, p.1 as f64)).collect()
}
fn orient(a: P, b: P, c: P) -> f64 { (b.0 - a.0) * (c.1 - a.1) - (b.1 - a.1) * (c.0 - a.0) }
fn seg_intersect(a: P, b: P, c: P, d: P) -> bool {
    let (d1, d2, d3, d4) = (orient(c, d, a), orient(c, d, b), orient(a, b, c), orient(a, b, d));
    if ((d1 > 0.0 && d2 < 0.0) || (d1 < 0.0 && d2 > 0.0)) && ((d3 > 0.0 && d4 < 0.0) || (d3 < 0.0 && d4 > 0.0)) { return true; }
    let on = |p: P, q: P, r: P| orient(p, q, r) == 0.0 && r.0 >= p.0.min(q.0) && r.0 <= p.0.max(q.0) && r.1 >= p.1.min(q.1) && r.1 <= p.1.max(q.1);
    on(c, d, a) || on(c, d, b) || on(a, b, c) || on(a, b, d)
}
fn seg_point_dist(a: P, b: P, p: P) -> f64 {
    let (dx, dy) = (b.0 - a.0, b.1 - a.1);
    let t = (((p.0 - a.0) * dx + (p.1 - a.1) * dy) / (dx * dx + dy * dy)).clamp(0.0, 1.0);
    ((a.0 + t * dx - p.0).powi(2) + (a.1 + t * dy - p.1).powi(2)).sqrt()
}
fn inside(poly: &[P], p: P) -> bool {
    let mut c = false;
    let n = poly.len();
    for i in 0..n {
        let (a, b) = (poly[i], poly[(i + 1) % n]);
        if (a.1 > p.1) != (b.1 > p.1) && p.0 < (b.0 - a.0) * (p.1 - a.1) / (b.1 - a.1) + a.0 { c = !c; }
    }
    c
}
/// "hit" if the f64 geometry collides; otherwise "miss <distance>" (closest approach, relative to the shape's size).
fn oracle(shape: &[jagua_rs::geometry::primitives::Point], haz: &[jagua_rs::geometry::primitives::Point], exterior: bool) -> String {
    let (s, h) = (pts(shape), pts(haz));
    let edges = |v: &Vec<P>| (0..v.len()).map(|i| (v[i], v[(i + 1) % v.len()])).collect::<Vec<_>>();
    let (se, he) = (edges(&s), edges(&h));
    if se.iter().any(|&(a, b)| he.iter().any(|&(c, d)| seg_intersect(a, b, c, d))) { return "hit".into(); }
    let contained = if exterior { !inside(&h, s[0]) } else { inside(&h, s[0]) || inside(&s, h[0]) };
    if contained { return "hit".into(); }
    let mut d = f64::INFINITY;
    for &(a, b) in &se { for &p in &h { d = d.min(seg_point_dist(a, b, p)); } }
    for &(a, b) in &he { for &p in &s { d = d.min(seg_point_dist(a, b, p)); } }
    let (xs, ys): (Vec<f64>, Vec<f64>) = s.iter().cloned().unzip();
    let size = (xs.iter().cloned().fold(f64::MIN, f64::max) - xs.iter().cloned().fold(f64::MAX, f64::min))
        .max(ys.iter().cloned().fold(f64::MIN, f64::max) - ys.iter().cloned().fold(f64::MAX, f64::min));
    format!("miss {:.3e}", d / size)
}
