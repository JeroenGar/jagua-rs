use jagua_rs_base::entities::{Container, Instance};
use jagua_rs_base::geometry::shape_modification::ShapeModifyConfig;
use jagua_rs_base::io::json_instance::JsonInstance;

/// Parses a `JsonInstance` into an `Instance`.
pub fn parse(&self, json_instance: &JsonInstance) -> Box<dyn Instance> {
    let items = json_instance
        .items
        .par_iter()
        .enumerate()
        .map(|(item_id, json_item)| self.parse_item(json_item, item_id))
        .collect();

    match (json_instance.bins.as_ref(), json_instance.strip.as_ref()) {
        (Some(json_bins), None) => {
            //bin packing instance
            let bins: Vec<(Container, usize)> = json_bins
                .par_iter()
                .enumerate()
                .map(|(bin_id, json_bin)| self.parse_bin(json_bin, bin_id))
                .collect();
            let bpi = BPInstance::new(items, bins);
            log!(
                    Level::Info,
                    "[PARSE] bin packing instance \"{}\": {} items ({} unique), {} bins ({} unique)",
                    json_instance.name,
                    bpi.total_item_qty(),
                    bpi.items.len(),
                    bpi.bins.iter().map(|(_, qty)| *qty).sum::<usize>(),
                    bpi.bins.len()
                );
            Box::new(bpi)
        }
        (None, Some(json_strip)) => {
            let strip_modify_config = ShapeModifyConfig {
                offset: self.shape_modify_config.offset,
                simplify_tolerance: None,
            };
            let spi = SPInstance::new(items, json_strip.height, strip_modify_config);
            log!(
                    Level::Info,
                    "[PARSE] strip packing instance \"{}\": {} items ({} unique), {} strip height",
                    json_instance.name,
                    spi.total_item_qty(),
                    spi.items.len(),
                    spi.strip_height
                );
            Box::new(spi)
        }
        (Some(_), Some(_)) => {
            panic!("Both bins and strip packing specified, has to be one or the other")
        }
        (None, None) => panic!("Neither bins or strips specified"),
    }
}