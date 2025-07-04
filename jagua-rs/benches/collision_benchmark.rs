use iai_callgrind::{library_benchmark, library_benchmark_group, main};
use jagua_rs::geometry::geo_traits::CollidesWith;
use jagua_rs::geometry::primitives::{Edge, Point, Rect};

// Test data generation
fn generate_test_cases() -> Vec<(Rect, Edge)> {
    let mut cases = Vec::new();

    // Rectangle at origin
    let rect = Rect::try_new(0.0, 0.0, 10.0, 10.0).unwrap();

    // Various edge configurations
    let edges = vec![
        // Edge completely inside
        Edge {
            start: Point(2.0, 2.0),
            end: Point(8.0, 8.0),
        },
        // Edge crossing rectangle
        Edge {
            start: Point(-5.0, 5.0),
            end: Point(15.0, 5.0),
        },
        // Edge touching corner
        Edge {
            start: Point(10.0, 10.0),
            end: Point(15.0, 15.0),
        },
        // Edge outside
        Edge {
            start: Point(-5.0, -5.0),
            end: Point(-2.0, -2.0),
        },
        // Edge parallel to sides
        Edge {
            start: Point(0.0, 15.0),
            end: Point(10.0, 15.0),
        },
    ];

    for edge in edges {
        cases.push((rect, edge));
    }

    cases
}

#[library_benchmark]
fn bench_simd_collision() {
    let cases = generate_test_cases();
    for (rect, edge) in cases {
        let _result = rect.collides_with(&edge);
    }
}

library_benchmark_group!(
    name = collision_benchmarks;
    benchmarks = bench_simd_collision
);

main!(library_benchmark_groups = collision_benchmarks);
