use jagua_rs::geometry::primitives::Point as JPoint;
type P = (f64, f64);
fn pts(v: &[JPoint]) -> Vec<P> {
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
fn oracle(shape: &[JPoint], haz: &[JPoint], exterior: bool) -> String {
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
