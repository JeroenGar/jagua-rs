use svg::Document;
use svg::node::element::Group;
use jaguars::entities::instance::Instance;
use jaguars::entities::layout::Layout;
use jaguars::entities::stored_layout::StoredLayout;
use jaguars::geometry::geo_enums::GeoPosition;
use jaguars::geometry::geo_traits::Transformable;
use jaguars::geometry::primitives::circle::Circle;
use crate::io::{svg_data_export, svg_util};
use crate::io::svg_util::{SvgDrawOptions};

pub fn layout_to_svg(s_layout: &StoredLayout, instance: &Instance, options: SvgDrawOptions) -> Document {
    let layout = Layout::new_from_stored(s_layout.id(), s_layout, &instance);
    let bin = layout.bin();

    let vbox = bin.bbox().clone().scale(1.05);

    let theme = &options.theme;

    let mut doc = Document::new()
        .set("viewBox", (vbox.x_min(), vbox.y_min(), vbox.width(), vbox.height()));

    let stroke_width = f64::min(vbox.width(), vbox.height()) * 0.001 * theme.stroke_width_multiplier;

    //draw bin
    let bin_group = {
        let mut group = Group::new();

        //outer
        group = group.add(
            svg_data_export::data_to_path(
                svg_data_export::simple_polygon_data(bin.outer()),
                &[
                    ("fill", &*format!("{}", theme.bin_fill)),
                    ("stroke", "black"),
                    ("stroke-width", &*format!("{}", 2.0 * stroke_width)),
                ]
            )
        );

        //holes
        for hole in bin.holes() {
            group = group.add(
                svg_data_export::data_to_path(
                    svg_data_export::simple_polygon_data(hole),
                    &[
                        ("fill", &*format!("{}", theme.hole_fill)),
                        ("stroke", "black"),
                        ("stroke-width", &*format!("{}", 1.0 * stroke_width)),
                    ]
                )
            )
        }
        group
    };

    let qz_group = {
        let mut group = Group::new();

        //quality zones
        for qz in bin.quality_zones().iter().rev().flatten() {
            let color = theme.qz_fill[qz.quality()];
            let stroke_color = svg_util::change_brightness(color, 0.5);
            for qz_shape in qz.shapes().iter() {
                group = group.add(
                    svg_data_export::data_to_path(
                        svg_data_export::simple_polygon_data(qz_shape),
                        &[
                            ("fill", &*format!("{}", color)),
                            ("fill-opacity", "0.50"),
                            ("stroke", &*format!("{}", stroke_color)),
                            ("stroke-width", &*format!("{}", 2.0 * stroke_width)),
                            ("stroke-opacity", &*format!("{}", theme.qz_stroke_opac)),
                            ("stroke-dasharray", &*format!("{}", 5.0 * stroke_width)),
                            ("stroke-linecap", "round"),
                            ("stroke-linejoin", "round")
                        ]
                    )
                )
            }
        }
        group
    };

    //draw items
    let item_group = {
        let mut group = Group::new();
        for pi in layout.placed_items() {
            let item = instance.item(pi.item_id());
            let shape = pi.shape();
            let color = match item.base_quality() {
                None => theme.item_fill.to_owned(),
                Some(q) => {
                    svg_util::blend_colors(
                        theme.item_fill,
                        theme.qz_fill[q],
                    )
                }
            };
            group = group.add(svg_data_export::data_to_path(
                svg_data_export::simple_polygon_data(&shape),
                &[
                    ("fill", &*format!("{}", color)),
                    ("stroke-width", &*format!("{}", stroke_width)),
                    ("fill-rule", "nonzero"),
                    ("stroke", "black"),
                    ("opacity", "0.9")
                ],
            ));

            if options.ff_surrogate {
                let poi_style = [
                    ("fill", "black"),
                    ("fill-opacity", "0.1"),
                    ("stroke", "black"),
                    ("stroke-width", &*format!("{}", stroke_width)),
                    ("stroke-opacity", "0.8"),
                ];
                let ff_style = [
                    ("fill", "none"),
                    ("stroke", "black"),
                    ("stroke-width", &*format!("{}", stroke_width)),
                    ("stroke-opacity", "0.8"),
                ];
                let no_ff_style = [
                    ("fill", "none"),
                    ("stroke", "black"),
                    ("stroke-width", &*format!("{}", stroke_width)),
                    ("stroke-opacity", "0.5"),
                    ("stroke-dasharray", &*format!("{}", 5.0 * stroke_width)),
                    ("stroke-linecap", "round"),
                    ("stroke-linejoin", "round")
                ];
                let transformed_surrogate = item.shape().surrogate().transform_clone(&pi.d_transformation().compose());
                let ff_range_poles = transformed_surrogate.config().ff_range_poles();
                let ff_range_clips = transformed_surrogate.config().ff_range_clips();


                for i in 0..transformed_surrogate.poles().len() {
                    let pole = &transformed_surrogate.poles()[i];
                    match i {
                        0 => group = group.add(svg_data_export::circle(pole, &poi_style)),
                        i if ff_range_poles.contains(&i) =>
                            group = group.add(svg_data_export::circle(pole, &ff_style)),
                        _ => group = group.add(svg_data_export::circle(pole, &no_ff_style)),
                    };
                }
                for i in 0..transformed_surrogate.clips().len() {
                    let clip = &transformed_surrogate.clips()[i];

                    match ff_range_clips.contains(&i) {
                        true => group = group.add(svg_data_export::data_to_path(svg_data_export::edge_data(clip), &ff_style)),
                        false => group = group.add(svg_data_export::data_to_path(svg_data_export::edge_data(clip), &no_ff_style)),
                    };
                }
            }
        }
        group
    };

    let quadtree_group = {
        let mut group = Group::new();
        if options.quadtree {
            let qt_data = svg_data_export::quad_tree_data(layout.cde().quadtree(), None);
            group = group.add(svg_data_export::data_to_path(
                qt_data.0,
                &[
                    ("fill", "red"),
                    ("stroke-width", &*format!("{}", stroke_width * 0.25)),
                    ("fill-rule", "nonzero"),
                    ("fill-opacity", "0.6"),
                    ("stroke", "black"),
                ],
            ));
            group = group.add(svg_data_export::data_to_path(
                qt_data.1,
                &[
                    ("fill", "none"),
                    ("stroke-width", &*format!("{}", stroke_width * 0.25)),
                    ("fill-rule", "nonzero"),
                    ("fill-opacity", "0.3"),
                    ("stroke", "black"),
                ],
            ));
            group = group.add(svg_data_export::data_to_path(
                qt_data.2,
                &[
                    ("fill", "green"),
                    ("fill-opacity", "0.6"),
                    ("stroke-width", &*format!("{}", stroke_width * 0.25)),
                    ("stroke", "black"),
                ],
            ));
        }
        group
    };

    let haz_prox_grid_group = {
        let mut group = Group::new();
        if options.haz_prox_grid {
            for hp_cell in layout.cde().haz_prox_grid().unwrap().cells().unwrap().iter().flatten() {
                let center = hp_cell.centroid();
                let prox = hp_cell.hazard_proximity(None);

                let (radius, color) = match prox.position {
                    GeoPosition::Interior => (0.0, "red"),
                    GeoPosition::Exterior => (prox.distance_from_border.into_inner(), "blue")
                };

                group = group.add(svg_data_export::point(center, Some(color), Some(stroke_width)));

                group = group.add(svg_data_export::circle(&Circle::new(center, radius), &[
                    ("fill", "none"),
                    ("stroke", color),
                    ("stroke-width", &*format!("{}", stroke_width)),
                ]));
            }
        }
        group
    };

    doc.add(bin_group)
        .add(item_group)
        .add(qz_group)
        .add(quadtree_group)
        .add(haz_prox_grid_group)
}
