use crate::io::svg_util::SvgDrawOptions;
use crate::io::{svg_export, svg_util};
use jagua_rs::entities::instances::instance::Instance;
use jagua_rs::entities::instances::instance_generic::InstanceGeneric;
use jagua_rs::entities::layout::Layout;
use jagua_rs::entities::layout::LayoutSnapshot;
use jagua_rs::fsize;
use jagua_rs::geometry::geo_traits::Shape;
use jagua_rs::geometry::transformation::Transformation;
use jagua_rs::io::parser;
use svg::node::element::{Definitions, Group, Title, Use};
use svg::Document;

pub fn s_layout_to_svg_experimental(
    s_layout: &LayoutSnapshot,
    instance: &Instance,
    options: SvgDrawOptions,
) -> Document {
    let layout = Layout::from_snapshot(s_layout);
    layout_to_svg_experimental(&layout, instance, options)
}

pub fn layout_to_svg_experimental(
    layout: &Layout,
    instance: &Instance,
    options: SvgDrawOptions,
) -> Document {
    let internal_bin = layout.bin();
    let bin = parser::pretransform_bin(internal_bin, &internal_bin.pretransform.clone().inverse());

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
    let items_group = {
        let mut items_group = Group::new();
        //define all the items in the group
        let mut item_definitions = Definitions::new();
        for (internal_item, qty) in instance.items() {
            let item = parser::pretransform_item(
                internal_item,
                &internal_item.pretransform.clone().inverse(),
            );
            let shape = item.shape.as_ref();
            let color = match item.base_quality {
                None => theme.item_fill.to_owned(),
                Some(q) => svg_util::blend_colors(theme.item_fill, theme.qz_fill[q]),
            };
            item_definitions =
                item_definitions.add(Group::new().set("id", format!("item_{}", item.id)).add(
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
                ))
        }
        items_group = items_group.add(item_definitions);

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
        }
        items_group
    };

    doc.add(bin_group).add(items_group).add(qz_group)
}

fn to_svg_transform_matrix(t: &Transformation) -> String {
    //https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/transform#matrix
    let [[a, c, e], [b, d, f], [_, _, _]] = t.matrix();
    format!("matrix({a} {b} {c} {d} {e} {f})")
}
