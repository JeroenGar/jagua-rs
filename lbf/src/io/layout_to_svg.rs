use crate::io::svg_util::SvgDrawOptions;
use crate::io::{svg_export, svg_util};
use jagua_rs::entities::instances::instance::Instance;
use jagua_rs::entities::instances::instance_generic::InstanceGeneric;
use jagua_rs::entities::layout::Layout;
use jagua_rs::entities::layout::LayoutSnapshot;
use jagua_rs::fsize;
use jagua_rs::geometry::primitives::circle::Circle;
use jagua_rs::geometry::transformation::Transformation;
use jagua_rs::io::parser;
use svg::node::element::{Definitions, Group, Title, Use};
use svg::Document;

pub fn s_layout_to_svg(
    s_layout: &LayoutSnapshot,
    instance: &Instance,
    options: SvgDrawOptions,
) -> Document {
    let layout = Layout::from_snapshot(s_layout);
    layout_to_svg(&layout, instance, options)
}

pub fn layout_to_svg(layout: &Layout, instance: &Instance, options: SvgDrawOptions) -> Document {
    let internal_bin = layout.bin();
    let inverse_bin_pretransform = internal_bin.pretransform.clone().inverse();
    let bin = parser::pretransform_bin(internal_bin, &inverse_bin_pretransform);

    let vbox = bin.bbox().clone().scale(1.05);

    let theme = &options.theme;

    let doc = Document::new()
        .set(
            "viewBox",
            (vbox.x_min, vbox.y_min, vbox.width(), vbox.height()),
        )
        .set("xmlns:xlink", "http://www.w3.org/1999/xlink");

    let stroke_width =
        fsize::min(vbox.width(), vbox.height()) * 0.001 * theme.stroke_width_multiplier;

    //draw bin
    let bin_group = {
        let mut group = Group::new();
        let bbox = bin.bbox();
        let title = Title::new(format!(
            "bin, id: {}, bbox: [x_min: {:.3}, y_min: {:.3}, x_max: {:.3}, y_max: {:.3}]",
            bin.id, bbox.x_min, bbox.y_min, bbox.x_max, bbox.y_max
        ));

        //outer
        group = group
            .add(svg_export::data_to_path(
                svg_export::simple_polygon_data(&bin.outer),
                &[
                    ("fill", &*format!("{}", theme.bin_fill)),
                    ("stroke", "black"),
                    ("stroke-width", &*format!("{}", 2.0 * stroke_width)),
                ],
            ))
            .add(title);

        //holes
        for (hole_idx, hole) in bin.holes.iter().enumerate() {
            group = group.add(
                svg_export::data_to_path(
                    svg_export::simple_polygon_data(hole),
                    &[
                        ("fill", &*format!("{}", theme.hole_fill)),
                        ("stroke", "black"),
                        ("stroke-width", &*format!("{}", 1.0 * stroke_width)),
                    ],
                )
                .add(Title::new(format!("hole #{}", hole_idx))),
            );
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
                            ("stroke-linejoin", "round"),
                        ],
                    )
                    .add(Title::new(format!("quality zone, q: {}", qz.quality))),
                );
            }
        }
        group
    };

    //draw items
    let (items_group, surrogate_group) = {
        //define all the items and their surrogates (if enabled)
        let mut item_defs = Definitions::new();
        let mut surrogate_defs = Definitions::new();
        for (internal_item, _) in instance.items() {
            let item = parser::pretransform_item(
                internal_item,
                &internal_item.pretransform.clone().inverse(),
            );
            let shape = item.shape.as_ref();
            let color = match item.base_quality {
                None => theme.item_fill.to_owned(),
                Some(q) => svg_util::blend_colors(theme.item_fill, theme.qz_fill[q]),
            };
            item_defs = item_defs.add(Group::new().set("id", format!("item_{}", item.id)).add(
                svg_export::data_to_path(
                    svg_export::simple_polygon_data(shape),
                    &[
                        ("fill", &*format!("{}", color)),
                        ("stroke-width", &*format!("{}", stroke_width)),
                        ("fill-rule", "nonzero"),
                        ("stroke", "black"),
                        ("opacity", "0.9"),
                    ],
                ),
            ));

            if options.surrogate {
                let mut surrogate_group = Group::new().set("id", format!("surrogate_{}", item.id));
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
                    ("stroke-linejoin", "round"),
                ];

                let surrogate = item.shape.surrogate();
                let poi = &surrogate.poles[0];
                let ff_poles = surrogate.ff_poles();

                for pole in surrogate.poles.iter() {
                    if pole == poi {
                        surrogate_group = surrogate_group.add(svg_export::circle(pole, &poi_style));
                    }
                    if ff_poles.contains(pole) {
                        surrogate_group = surrogate_group.add(svg_export::circle(pole, &ff_style));
                    } else {
                        surrogate_group =
                            surrogate_group.add(svg_export::circle(pole, &no_ff_style));
                    }
                }
                for pier in &surrogate.piers {
                    surrogate_group = surrogate_group.add(svg_export::data_to_path(
                        svg_export::edge_data(pier),
                        &ff_style,
                    ));
                }
                surrogate_defs = surrogate_defs.add(surrogate_group)
            }
        }
        let mut items_group = Group::new().add(item_defs);
        let mut surrogate_group = Group::new().add(surrogate_defs);

        for pi in layout.placed_items().values() {
            let abs_transf = parser::internal_to_absolute_transform(
                &pi.d_transf,
                &instance.item(pi.item_id).pretransform,
                &internal_bin.pretransform,
            );
            let title = Title::new(format!(
                "item, id: {}, transf: [{}]",
                pi.item_id,
                abs_transf.decompose()
            ));
            let pi_ref = Use::new()
                .set("transform", to_svg_transform_matrix(&abs_transf))
                .set("xlink:href", format!("#item_{}", pi.item_id))
                .add(title);

            items_group = items_group.add(pi_ref);

            if options.surrogate {
                let pi_surr_ref = Use::new()
                    .set("transform", to_svg_transform_matrix(&abs_transf))
                    .set("xlink:href", format!("#surrogate_{}", pi.item_id));

                surrogate_group = surrogate_group.add(pi_surr_ref);
            }
        }

        (items_group, surrogate_group)
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
        group.set(
            "transform",
            to_svg_transform_matrix(&inverse_bin_pretransform),
        )
    };

    let haz_prox_grid_group = {
        let mut group = Group::new();
        if options.haz_prox_grid {
            let hpg = layout.cde().haz_prox_grid().unwrap();
            for hp_cell in hpg.grid.cells.iter().flatten() {
                let center = hp_cell.centroid;
                let prox = hp_cell.hazard_proximity(None);

                let color = if prox == 0.0 { "red" } else { "blue" };

                group = group.add(svg_export::point(center, Some(color), Some(stroke_width)));

                group = group.add(svg_export::circle(
                    &Circle::new(center, prox),
                    &[
                        ("fill", "none"),
                        ("stroke", color),
                        ("stroke-width", &*format!("{}", stroke_width / 2.0)),
                    ],
                ));
            }
        }
        group.set(
            "transform",
            to_svg_transform_matrix(&inverse_bin_pretransform),
        )
    };

    doc.add(bin_group)
        .add(items_group)
        .add(surrogate_group)
        .add(qz_group)
        .add(quadtree_group)
        .add(haz_prox_grid_group)
}

fn to_svg_transform_matrix(t: &Transformation) -> String {
    //https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/transform#matrix
    let [[a, c, e], [b, d, f], [_, _, _]] = t.matrix();
    format!("matrix({a} {b} {c} {d} {e} {f})")
}
