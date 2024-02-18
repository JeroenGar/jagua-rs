use svg::Document;
use svg::node::element::Group;

use jaguars::entities::instance::Instance;
use jaguars::entities::instance::InstanceGeneric;
use jaguars::entities::layout::Layout;
use jaguars::entities::layout::LayoutSnapshot;
use jaguars::geometry::geo_enums::GeoPosition;
use jaguars::geometry::geo_traits::Transformable;
use jaguars::geometry::primitives::circle::Circle;

use crate::io::{svg_export, svg_util};
use crate::io::svg_util::SvgDrawOptions;

pub fn s_layout_to_svg(s_layout: &LayoutSnapshot, instance: &Instance, options: SvgDrawOptions) -> Document {
    let layout = Layout::new_from_stored(s_layout.id, s_layout);
    layout_to_svg(&layout, instance, options)
}

pub fn layout_to_svg(layout: &Layout, instance: &Instance, options: SvgDrawOptions) -> Document {
    let bin = layout.bin();

    let vbox = bin.bbox().clone().scale(1.05);

    let theme = &options.theme;

    let doc = Document::new()
        .set("viewBox", (vbox.x_min, vbox.y_min, vbox.width(), vbox.height()));

    let stroke_width = f64::min(vbox.width(), vbox.height()) * 0.001 * theme.stroke_width_multiplier;

    //draw bin
    let bin_group = {
        let mut group = Group::new();

        //outer
        group = group.add(
            svg_export::data_to_path(
                svg_export::simple_polygon_data(&bin.outer),
                &[
                    ("fill", &*format!("{}", theme.bin_fill)),
                    ("stroke", "black"),
                    ("stroke-width", &*format!("{}", 2.0 * stroke_width)),
                ]
            )
        );

        //holes
        for hole in &bin.holes {
            group = group.add(
                svg_export::data_to_path(
                    svg_export::simple_polygon_data(hole),
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
        for qz in bin.quality_zones.iter().rev().flatten() {
            let color = theme.qz_fill[qz.quality];
            let stroke_color = svg_util::change_brightness(color, 0.5);
            for qz_shape in qz.zones.iter() {
                group = group.add(
                    svg_export::data_to_path(
                        svg_export::simple_polygon_data(qz_shape),
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
    let items_group = {
        let mut items_group = Group::new();
        for pi in layout.placed_items() {
            let mut group = Group::new();
            let item = instance.item(pi.item_id());
            let shape = &pi.shape;
            let color = match item.base_quality {
                None => theme.item_fill.to_owned(),
                Some(q) => {
                    svg_util::blend_colors(
                        theme.item_fill,
                        theme.qz_fill[q],
                    )
                }
            };
            group = group.add(svg_export::data_to_path(
                svg_export::simple_polygon_data(&shape),
                &[
                    ("fill", &*format!("{}", color)),
                    ("stroke-width", &*format!("{}", stroke_width)),
                    ("fill-rule", "nonzero"),
                    ("stroke", "black"),
                    ("opacity", "0.9")
                ],
            ));

            if options.surrogate {
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

                let transformed_surrogate = item.shape.surrogate().transform_clone(&pi.d_transformation().compose());
                let poi = &transformed_surrogate.poles[0];
                let ff_poles = transformed_surrogate.ff_poles();

                for i in 0..transformed_surrogate.poles.len() {
                    let pole = &transformed_surrogate.poles[i];
                    if pole == poi {
                        group = group.add(svg_export::circle(pole, &poi_style));
                    }
                    if ff_poles.contains(&pole) {
                        group = group.add(svg_export::circle(pole, &ff_style));
                    } else {
                        group = group.add(svg_export::circle(pole, &no_ff_style));
                    }
                }
                for pier in &transformed_surrogate.piers {
                    group = group.add(svg_export::data_to_path(svg_export::edge_data(pier), &ff_style));
                }
            }
            items_group = items_group.add(group);
        }
        items_group
    };

    let quadtree_group = {
        let mut group = Group::new();
        if options.quadtree {
            let qt_data = svg_export::quad_tree_data(layout.cde().quadtree(), &[]);
            group = group.add(svg_export::data_to_path(
                qt_data.0,
                &[
                    ("fill", "red"),
                    ("stroke-width", &*format!("{}", stroke_width * 0.25)),
                    ("fill-rule", "nonzero"),
                    ("fill-opacity", "0.6"),
                    ("stroke", "black"),
                ],
            ));
            group = group.add(svg_export::data_to_path(
                qt_data.1,
                &[
                    ("fill", "none"),
                    ("stroke-width", &*format!("{}", stroke_width * 0.25)),
                    ("fill-rule", "nonzero"),
                    ("fill-opacity", "0.3"),
                    ("stroke", "black"),
                ],
            ));
            group = group.add(svg_export::data_to_path(
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
            let hpg = layout.cde().haz_prox_grid().unwrap();
            for hp_cell in hpg.grid.cells.iter().flatten() {
                let center = hp_cell.centroid();
                let prox = hp_cell.hazard_proximity(None);

                let (radius, color) = match prox.position {
                    GeoPosition::Interior => (0.0, "red"),
                    GeoPosition::Exterior => (prox.distance_from_border.into_inner(), "blue")
                };

                group = group.add(svg_export::point(center, Some(color), Some(stroke_width)));

                group = group.add(svg_export::circle(&Circle::new(center, radius), &[
                    ("fill", "none"),
                    ("stroke", color),
                    ("stroke-width", &*format!("{}", stroke_width)),
                ]));

                group = group.add(svg_export::data_to_path(svg_export::aa_rect_data(hp_cell.bbox()), &[
                    ("fill", "none"),
                    ("stroke", color),
                    ("stroke-width", &*format!("{}", stroke_width)),
                ]));
            }
        }
        group
    };

    doc.add(bin_group)
        .add(items_group)
        .add(qz_group)
        .add(quadtree_group)
        .add(haz_prox_grid_group)
}
