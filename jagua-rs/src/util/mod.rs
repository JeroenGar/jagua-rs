use crate::entities::layout::Layout;

pub mod assertions;
pub mod config;
pub mod f64a;
pub mod polygon_simplification;

///Intended for debugging purposes
pub fn print_layout(layout: &Layout) {
    println!(
        "let mut layout = Layout::new(0, instance.bin({}).clone());",
        layout.bin().id
    );
    println!();

    for pi in layout.placed_items() {
        let transformation_str = {
            let t_decomp = &pi.uid.d_transf;
            let (tr, (tx, ty)) = (t_decomp.rotation(), t_decomp.translation());
            format!("&DTransformation::new({:.6},({:.6},{:.6}))", tr, tx, ty)
        };

        println!(
            "layout.place_item(instance.item({}), {});",
            pi.item_id(),
            transformation_str
        );
    }
}
