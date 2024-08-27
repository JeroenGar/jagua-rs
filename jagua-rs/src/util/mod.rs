use crate::entities::layout::Layout;

/// Set of functions used throughout assure the correctness of the library.
pub mod assertions;

/// Configuration options for the library
pub mod config;

pub mod fpa;

/// Functions to simplify polygons in preprocessing
pub mod polygon_simplification;

///Prints code to recreate a layout. Intended for debugging purposes.
pub fn print_layout(layout: &Layout) {
    println!(
        "let mut layout = Layout::new(0, instance.bin({}).clone());",
        layout.bin().id
    );
    println!();

    for pi in layout.placed_items().values() {
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
