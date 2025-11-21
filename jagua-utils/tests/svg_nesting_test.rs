#[cfg(test)]
mod tests {
    use anyhow::Result;
    use env_logger;
    use jagua_utils::svg_nesting::nest_svg_parts;
    use log::{debug, info};
    use std::cell::Cell;
    use std::time::Instant;

    #[test]
    fn test_svg_nesting() -> Result<()> {
        let _ = env_logger::Builder::from_default_env()
            .filter_level(log::LevelFilter::Debug)
            .try_init();

        let test_start_time = Instant::now();
        const MAX_TIME_SECONDS: u64 = 15;

        let svg_document = r#"<?xml version="1.0" standalone="no"?>
<!DOCTYPE svg PUBLIC "-//W3C//DTD SVG 1.1//EN" "http://www.w3.org/Graphics/SVG/1.1/DTD/svg11.dtd">
<svg width="90mm" height="90mm" viewBox="-45 -45 90 90" xmlns="http://www.w3.org/2000/svg" version="1.1">
<title>OpenSCAD Model</title>
<path d="
M 13.9062,42.7979 L 22.5,38.9707 L 30.1113,33.4414 L 36.4062,26.4502 L 41.1094,18.3027 L 44.0166,9.35645
 L 45,-0 L 44.0166,-9.35645 L 41.1094,-18.3027 L 36.4062,-26.4502 L 30.1113,-33.4414 L 22.5,-38.9707
 L 13.9062,-42.7979 L 4.7041,-44.7539 L -4.7041,-44.7539 L -13.9062,-42.7979 L -22.5,-38.9707 L -30.1113,-33.4414
 L -36.4062,-26.4502 L -41.1094,-18.3027 L -44.0166,-9.35645 L -45,-0 L -44.0166,9.35645 L -41.1094,18.3027
 L -36.4062,26.4502 L -30.1113,33.4414 L -22.5,38.9707 L -13.9062,42.7979 L -4.7041,44.7539 L 4.7041,44.7539
 z
M -1.41797,-24.2598 L -2.99414,-25.3477 L -3.88379,-27.043 L -3.88379,-28.957 L -2.99414,-30.6523 L -1.41797,-31.7402
 L 0.482422,-31.9707 L 2.27246,-31.292 L 3.54199,-29.8594 L 4,-28 L 3.54199,-26.1406 L 2.27246,-24.708
 L 0.482422,-24.0293 z
M -0.955078,5.41602 L -2.75,4.7627 L -4.21289,3.53516 L -5.16797,1.88086 L -5.5,-0 L -5.16797,-1.88086
 L -4.21289,-3.53516 L -2.75,-4.7627 L -0.955078,-5.41602 L 0.955078,-5.41602 L 2.75,-4.7627 L 4.21289,-3.53516
 L 5.16797,-1.88086 L 5.5,-0 L 5.16797,1.88086 L 4.21289,3.53516 L 2.75,4.7627 L 0.955078,5.41602
 z
M -29.418,3.74023 L -30.9941,2.65234 L -31.8838,0.957031 L -31.8838,-0.957031 L -30.9941,-2.65234 L -29.418,-3.74023
 L -27.5176,-3.9707 L -25.7275,-3.29199 L -24.458,-1.85938 L -24,-0 L -24.458,1.85938 L -25.7275,3.29199
 L -27.5176,3.9707 z
M 26.582,3.74023 L 25.0059,2.65234 L 24.1162,0.957031 L 24.1162,-0.957031 L 25.0059,-2.65234 L 26.582,-3.74023
 L 28.4824,-3.9707 L 30.2725,-3.29199 L 31.542,-1.85938 L 32,-0 L 31.542,1.85938 L 30.2725,3.29199
 L 28.4824,3.9707 z
M -1.41797,31.7402 L -2.99414,30.6523 L -3.88379,28.957 L -3.88379,27.043 L -2.99414,25.3477 L -1.41797,24.2598
 L 0.482422,24.0293 L 2.27246,24.708 L 3.54199,26.1406 L 4,28 L 3.54199,29.8594 L 2.27246,31.292
 L 0.482422,31.9707 z
" stroke="black" fill="lightgray" stroke-width="0.5"/>
</svg>"#;
        let svg_bytes = svg_document.as_bytes();

        // Verify that the SVG has 5 holes
        let svg_str = std::str::from_utf8(svg_bytes)?;
        let hole_count = svg_str.matches(" z").count() - 1; // Subtract 1 for the outer boundary
        debug!("Input SVG has {} holes (expected 5)", hole_count);

        let result = nest_svg_parts(350.0, 350.0, 50.0, svg_bytes, 4, 8, 10, 100)?;

        let max_parts_placed = result.parts_placed;

        assert_eq!(
            max_parts_placed, 4,
            "Expected 4 items to be placed, but got {} items",
            max_parts_placed
        );

        let svg_string = String::from_utf8(result.combined_svg.clone())?;
        debug!(
            "Best placement SVG ({} parts placed):\n{}",
            max_parts_placed, svg_string
        );

        let elapsed = test_start_time.elapsed();
        assert!(
            elapsed.as_secs() <= MAX_TIME_SECONDS,
            "Test took {} seconds, which exceeds the 15 second limit",
            elapsed.as_secs()
        );

        Ok(())
    }

    #[test]
    fn test_svg_nesting_25_parts() -> Result<()> {
        let _ = env_logger::Builder::from_default_env()
            .filter_level(log::LevelFilter::Debug)
            .try_init();

        let test_start_time = Instant::now();
        const MAX_TIME_SECONDS: u64 = 100; // 10 minutes (handled by adaptive config)

        let svg_document = r#"<?xml version="1.0" standalone="no"?>
<!DOCTYPE svg PUBLIC "-//W3C//DTD SVG 1.1//EN" "http://www.w3.org/Graphics/SVG/1.1/DTD/svg11.dtd">
<svg width="485mm" height="400mm" viewBox="-35 0 485 400" xmlns="http://www.w3.org/2000/svg" version="1.1">
    <title>OpenSCAD Model</title>
    <path d="
M 450,372.414 L 450,293.347 L 40,293.347 L 40,349.514 L 38.5,349.514 L 38.5,319.067
 L 18,-0 L -34.5332,-0 L -34.5332,400 L 40,400 z
M -16.4473,39.6191 L -18.0693,38.5352 L -19.1523,36.9131 L -19.5332,35 L -19.1523,33.0869 L -18.0693,31.4648
 L -16.4473,30.3809 L -14.5332,30 L -12.6201,30.3809 L -10.998,31.4648 L -9.91406,33.0869 L -9.5332,35
 L -9.91406,36.9131 L -10.998,38.5352 L -12.6201,39.6191 L -14.5332,40 z
M -16.4473,189.619 L -18.0693,188.535 L -19.1523,186.913 L -19.5332,185 L -19.1523,183.087 L -18.0693,181.465
 L -16.4473,180.381 L -14.5332,180 L -12.6201,180.381 L -10.998,181.465 L -9.91406,183.087 L -9.5332,185
 L -9.91406,186.913 L -10.998,188.535 L -12.6201,189.619 L -14.5332,190 z
M 104.5,333.381 L 102.619,333.049 L 100.965,332.094 L 99.7373,330.631 L 99.084,328.836 L 99.084,326.926
 L 99.7373,325.131 L 100.965,323.667 L 102.619,322.712 L 104.5,322.381 L 173.5,322.381 L 175.381,322.712
 L 177.035,323.667 L 178.263,325.131 L 178.916,326.926 L 178.916,328.836 L 178.263,330.631 L 177.035,332.094
 L 175.381,333.049 L 173.5,333.381 z
M 364.5,333.381 L 362.619,333.049 L 360.965,332.094 L 359.737,330.631 L 359.084,328.836 L 359.084,326.926
 L 359.737,325.131 L 360.965,323.667 L 362.619,322.712 L 364.5,322.381 L 433.5,322.381 L 435.381,322.712
 L 437.035,323.667 L 438.263,325.131 L 438.916,326.926 L 438.916,328.836 L 438.263,330.631 L 437.035,332.094
 L 435.381,333.049 L 433.5,333.381 z
" stroke="black" fill="lightgray" stroke-width="0.5"/>
</svg>"#;
        let svg_bytes = svg_document.as_bytes();

        // Verify that the SVG has holes
        let svg_str = std::str::from_utf8(svg_bytes)?;
        let hole_count = svg_str.matches(" z").count() - 1; // Subtract 1 for the outer boundary
        debug!("Input SVG has {} holes (expected 4)", hole_count);

        use jagua_utils::svg_nesting::{AdaptiveConfig, nest_svg_parts_adaptive};

        let config = AdaptiveConfig {
            min_samples: 50,
            max_samples: 1000,
            min_loops: 1,
            max_loops: 50,
            min_placements: 10,
            max_placements: 500,
            min_ls_frac: 0.1,
            max_ls_frac: 0.5,
            increase_after_no_improvement: 5,
            consecutive_no_improvement_limit: 100,
            max_time_seconds: 100, // 100 seconds
        };
        let intermediate = Cell::new(0);
        let intermediate_handler = |svg_bytes: &[u8], parts_placed: usize| {
            if parts_placed > intermediate.get() {
                intermediate.set(parts_placed);
                if let Ok(svg_string) = String::from_utf8(svg_bytes.to_vec()) {
                    info!(
                        "Intermediate SVG ({} parts placed):\n{}",
                        parts_placed, svg_string
                    );
                }
            }
            false
        };
        let result = nest_svg_parts_adaptive(
            2000.0, // bin_width
            2000.0, // bin_height
            50.0,   // spacing
            svg_bytes,
            25, // amount_of_parts
            8,  // amount_of_rotations (8 rotations = every 45 degrees)
            config,
            Some(intermediate_handler),
        )?;

        debug!("Placed {} parts out of 25 requested", result.parts_placed);

        let svg_string = String::from_utf8(result.combined_svg.clone())?;
        debug!(
            "Best placement SVG ({} parts placed):\n{}",
            result.parts_placed, svg_string
        );

        let elapsed = test_start_time.elapsed();
        assert!(
            elapsed.as_secs() <= MAX_TIME_SECONDS,
            "Test took {} seconds, which exceeds the {} second limit",
            elapsed.as_secs(),
            MAX_TIME_SECONDS
        );

        Ok(())
    }
}
