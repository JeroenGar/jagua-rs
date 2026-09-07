use super::FPA;
use std::cmp::Ordering::{Equal, Greater, Less};

#[test]
fn tolerant_comparisons_match_partial_order() {
    let values: Vec<_> = [
        f32::NAN,
        f32::NEG_INFINITY,
        -f32::MAX,
        -56.1875,
        -1.0,
        -f32::MIN_POSITIVE,
        -0.0,
        0.0,
        f32::MIN_POSITIVE,
        f32::EPSILON,
        1.0,
        56.1875,
        f32::MAX,
        f32::INFINITY,
    ]
    .into_iter()
    .flat_map(|value| {
        // Include both sides of the default four-ULP tolerance.
        let mut neighbors = vec![value];
        let (mut below, mut above) = (value, value);
        for _ in 0..5 {
            below = below.next_down();
            above = above.next_up();
            neighbors.extend([below, above]);
        }
        neighbors
    })
    .map(FPA)
    .collect();
    for a in &values {
        for b in &values {
            assert_eq!(a <= b, matches!(a.partial_cmp(b), Some(Less | Equal)));
            assert_eq!(a >= b, matches!(a.partial_cmp(b), Some(Greater | Equal)));
        }
    }
}
