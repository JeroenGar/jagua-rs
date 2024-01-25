use crate::entities::layout::Layout;

//INTENDED FOR DEBUGGING PURPOSES
pub fn print_layout(layout: &Layout) {
    println!("let mut layout = Layout::new(0, instance.bin({}).clone());", layout.bin().id());
    println!();

    for pi in layout.placed_items() {
        let transformation_str = {
            let t_decomp = pi.uid().d_transformation();
            let (tr, (tx, ty)) = (t_decomp.rotation(), t_decomp.translation());
            format!("&DTransformation::new({:.6},({:.6},{:.6}))", tr, tx, ty)
        };

        println!("layout.place_item(instance.item({}), {});", pi.item_id(), transformation_str);
    }
}