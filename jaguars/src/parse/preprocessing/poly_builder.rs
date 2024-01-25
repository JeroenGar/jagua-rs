use std::collections::HashMap;

use crate::geometry::geo_traits::CollidesWith;
use crate::geometry::primitives::simple_polygon::SimplePolygon;
use crate::simplification::polygon_converter;

pub fn build_polygons(simple_polygons: &Vec<SimplePolygon>) -> Vec<SimplePolygon> {
    //detect inner and outer polygons
    let mut results = Vec::new();
    let mut inner_outer_map: HashMap<usize, usize> = HashMap::new();

    for i1 in 0..simple_polygons.len() {
        //search for outer poly
        let poly_1 = simple_polygons.get(i1).unwrap();
        let point = poly_1.get_point(0);
        for i2 in 0..simple_polygons.len() {
            if i1 != i2 {
                let poly_2 = simple_polygons.get(i2).unwrap();
                if poly_2.collides_with(&point) {
                    //poly 1 is inside poly 2
                    inner_outer_map.insert(i1, i2);
                }
            }
        }
    }

    let mut outer_inner_map: HashMap<usize, Vec<usize>> = HashMap::new();
    for (i_inner, i_outer) in &inner_outer_map {
        let entry = outer_inner_map.entry(i_outer.clone()).or_insert(Vec::new());
        entry.push(i_inner.clone());
    }

    for i in 0..simple_polygons.len() {
        if !outer_inner_map.contains_key(&i) && !inner_outer_map.contains_key(&i) {
            //this is a outer poly without any holes
            outer_inner_map.insert(i, Vec::new());
        }
    }

    for (i_outer, i_inner_vec) in outer_inner_map {
        let outer = simple_polygons.get(i_outer).unwrap().clone();
        let mut inner_vec = Vec::new();
        for i_inner in i_inner_vec {
            inner_vec.push(
                simple_polygons
                    .get(i_inner)
                    .unwrap().clone()
            );
        }
        let shape = polygon_converter::convert_to_simple_polygon(&outer, &inner_vec);
        results.push(shape);
    }
    results
}