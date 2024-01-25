use std::{env};
use std::process::Command;

use svg::Document;
use svg::node::element::{Circle, Path, Pattern, Rectangle};
use svg::node::element::path::Data;
use jaguars::collision_detection::hazards::hazard_entity::HazardEntity;

use jaguars::collision_detection::quadtree::qt_hazard_type::QTHazType;
use jaguars::collision_detection::quadtree::qt_node::QTNode;
use jaguars::entities::instance::Instance;
use jaguars::entities::layout::Layout;
use jaguars::entities::placed_item_uid::PlacedItemUID;
use jaguars::geometry;
use jaguars::geometry::primitives::edge::Edge;
use jaguars::geometry::geo_traits::Transformable;
use jaguars::geometry::primitives::point::Point;
use jaguars::geometry::primitives::simple_polygon::SimplePolygon;
use crate::io::svg_util::SvgDrawOptions;

pub fn simple_polygon_data(s_poly: &SimplePolygon) -> Data {
    let mut data = Data::new().move_to::<(f64, f64)>(s_poly.get_point(0).into());
    for i in 1..s_poly.number_of_points() {
        data = data.line_to::<(f64, f64)>(s_poly.get_point(i).into());
    }
    data.close()
}

pub fn quad_tree_data(qt_root: &QTNode, ignored_entities: Option<&Vec<&HazardEntity>>) -> (Data, Data, Data) {
    qt_node_data(qt_root, Data::new(), Data::new(), Data::new(), ignored_entities)
}

fn qt_node_data(
    qt_node: &QTNode,
    mut data_eh: Data, //entire inclusion data
    mut data_ph: Data, //partial inclusion data
    mut data_nh: Data, //no inclusion data
    ignored_entities: Option<&Vec<&HazardEntity>>,
) -> (Data, Data, Data) {
    //Only draw qt_nodes that do not have a child

    match (qt_node.has_children(), qt_node.hazards().strongest(ignored_entities)) {
        (true, Some(_)) => {
            //not a leaf node, go to children
            for child in qt_node.children().as_ref().unwrap().iter() {
                let data = qt_node_data(child, data_eh, data_ph, data_nh, ignored_entities);
                data_eh = data.0;
                data_ph = data.1;
                data_nh = data.2;
            }
        }
        (true, None) | (false, _) => {
            //leaf node, draw it
            let rect = qt_node.bbox();
            let draw = |data: Data| -> Data {
                data
                    .move_to((rect.x_min(), rect.y_min()))
                    .line_to((rect.x_max(), rect.y_min()))
                    .line_to((rect.x_max(), rect.y_max()))
                    .line_to((rect.x_min(), rect.y_max()))
                    .close()
            };

            match qt_node.hazards().strongest(ignored_entities) {
                Some(ch) => {
                    match ch.haz_type() {
                        QTHazType::Entire => data_eh = draw(data_eh),
                        QTHazType::Partial(_) => data_ph = draw(data_ph)
                    }
                }
                None => data_nh = draw(data_nh),
            }
        }
    }

    (data_eh, data_ph, data_nh)
}

pub fn data_to_path(data: Data, params: &[(&str, &str)]) -> Path {
    let mut path = Path::new();
    for param in params {
        path = path.set(param.0, param.1)
    }
    path.set("d", data)
}

pub fn point(Point(x, y): Point, fill: Option<&str>, rad: Option<f64>) -> Circle {
    Circle::new()
        .set("cx", x)
        .set("cy", y)
        .set("r", rad.unwrap_or(0.5))
        .set("fill", fill.unwrap_or("black"))
}

pub fn circle(circle: &geometry::primitives::circle::Circle, params: &[(&str, &str)]) -> Circle {
    let mut circle = Circle::new()
        .set("cx", circle.center().0)
        .set("cy", circle.center().1)
        .set("r", circle.radius());
    for param in params {
        circle = circle.set(param.0, param.1)
    }
    circle
}

pub fn rect_data(rect: &geometry::primitives::aa_rectangle::AARectangle) -> Data {
    Data::new()
        .move_to((rect.x_min(), rect.y_min()))
        .line_to((rect.x_max(), rect.y_min()))
        .line_to((rect.x_max(), rect.y_max()))
        .line_to((rect.x_min(), rect.y_max()))
        .close()
}

pub fn edge_data(edge: &Edge) -> Data {
    Data::new()
        .move_to((edge.start().0, edge.start().1))
        .line_to((edge.end().0, edge.end().1))
}

pub fn rgb_to_hex(r: i32, g: i32, b: i32) -> String {
    format!(
        "#{:02X}{:02X}{:02X}",
        r as f32 as u8, g as f32 as u8, b as f32 as u8
    )
}

pub fn diagonal_hatch_pattern(id: &str, background_color: &str, hatch_color: &str, distance: f64, width: f64) -> Pattern {
    Pattern::new()
        .set("id", id)
        .set("width", distance)
        .set("height", distance)
        .set("patternTransform", "rotate(45 0 0)")
        .set("patternUnits", "userSpaceOnUse")
        .add(
            Rectangle::new()
                .set("width", distance)
                .set("height", distance)
                .set("fill", background_color),
        )
        .add(
            svg::node::element::Line::new()
                .set("x1", 0)
                .set("y1", 0)
                .set("x2", 0)
                .set("y2", distance)
                .set("style", format!("stroke:{}; stroke-width:{}", hatch_color, width)),
        )
}
