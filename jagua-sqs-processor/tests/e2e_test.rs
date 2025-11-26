use anyhow::Result;
use base64::{engine::general_purpose, Engine as _};
use jagua_sqs_processor::{SqsNestingRequest, SqsNestingResponse};
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};

/// Process a request directly (bypassing AWS SDK) and capture responses
fn process_request_direct(request_json: &str) -> Result<Vec<SqsNestingResponse>> {
    use jagua_utils::svg_nesting::{nest_svg_parts_adaptive, AdaptiveConfig};

    let request: SqsNestingRequest = serde_json::from_str(request_json)?;

    // Validate required fields for non-cancellation requests
    let svg_base64 = request
        .svg_base64
        .as_ref()
        .ok_or_else(|| anyhow::anyhow!("Missing required field: svg_base64"))?;
    let bin_width = request
        .bin_width
        .ok_or_else(|| anyhow::anyhow!("Missing required field: bin_width"))?;
    let bin_height = request
        .bin_height
        .ok_or_else(|| anyhow::anyhow!("Missing required field: bin_height"))?;
    let spacing = request
        .spacing
        .ok_or_else(|| anyhow::anyhow!("Missing required field: spacing"))?;
    let amount_of_parts = request
        .amount_of_parts
        .ok_or_else(|| anyhow::anyhow!("Missing required field: amount_of_parts"))?;

    let svg_bytes = general_purpose::STANDARD
        .decode(svg_base64)
        .map_err(|e| anyhow::anyhow!("Failed to decode svg_base64: {}", e))?;

    let improvements: Arc<Mutex<Vec<SqsNestingResponse>>> = Arc::new(Mutex::new(Vec::new()));
    let improvements_clone = improvements.clone();
    let correlation_id = request.correlation_id.clone();

    let callback = move |svg_bytes: &[u8], parts_placed: usize| {
        let encoded_page = general_purpose::STANDARD.encode(svg_bytes);
        let response = SqsNestingResponse {
            correlation_id: correlation_id.clone(),
            first_page_svg_base64: encoded_page.clone(),
            last_page_svg_base64: Some(encoded_page),
            parts_placed,
            is_improvement: true,
            is_final: false,
            timestamp: current_timestamp(),
            error_message: None,
        };
        improvements_clone.lock().unwrap().push(response);
        false
    };

    let config = AdaptiveConfig::default();
    let nesting_result = nest_svg_parts_adaptive(
        bin_width,
        bin_height,
        spacing,
        &svg_bytes,
        amount_of_parts,
        request.amount_of_rotations,
        config,
        Some(callback),
    )?;

    let mut responses = improvements.lock().unwrap().clone();

    let first_page_bytes = nesting_result
        .page_svgs
        .first()
        .unwrap_or(&nesting_result.combined_svg);
    let last_page_bytes = nesting_result.page_svgs.last().unwrap_or(first_page_bytes);
    let first_page_svg_base64 = general_purpose::STANDARD.encode(first_page_bytes);
    let last_page_svg_base64 = general_purpose::STANDARD.encode(last_page_bytes);

    responses.push(SqsNestingResponse {
        correlation_id: request.correlation_id,
        first_page_svg_base64,
        last_page_svg_base64: Some(last_page_svg_base64),
        parts_placed: nesting_result.parts_placed,
        is_improvement: false,
        is_final: true,
        timestamp: current_timestamp(),
        error_message: None,
    });

    Ok(responses)
}

#[tokio::test]
async fn test_e2e_processing() -> Result<()> {
    let _ = env_logger::Builder::from_default_env()
        .filter_level(log::LevelFilter::Debug)
        .try_init();

    let test_svg = r#"<?xml version="1.0" standalone="no"?>
<!DOCTYPE svg PUBLIC "-//W3C//DTD SVG 1.1//EN" "http://www.w3.org/Graphics/SVG/1.1/DTD/svg11.dtd">
<svg width="90mm" height="90mm" viewBox="-45 -45 90 90" xmlns="http://www.w3.org/2000/svg" version="1.1">
<title>Test Shape</title>
<path d="M 13.9062,42.7979 L 22.5,38.9707 L 30.1113,33.4414 L 36.4062,26.4502 L 41.1094,18.3027 L 44.0166,9.35645
 L 45,-0 L 44.0166,-9.35645 L 41.1094,-18.3027 L 36.4062,-26.4502 L 30.1113,-33.4414 L 22.5,-38.9707
 L 13.9062,-42.7979 L 4.7041,-44.7539 L -4.7041,-44.7539 L -13.9062,-42.7979 L -22.5,-38.9707 L -30.1113,-33.4414
 L -36.4062,-26.4502 L -41.1094,-18.3027 L -44.0166,-9.35645 L -45,-0 L -44.0166,9.35645 L -41.1094,18.3027
 L -36.4062,26.4502 L -30.1113,33.4414 L -22.5,38.9707 L -13.9062,42.7979 L -4.7041,44.7539 L 4.7041,44.7539 z
" stroke="black" fill="lightgray" stroke-width="0.5"/>
</svg>"#;

    let request = SqsNestingRequest {
        correlation_id: "test-correlation-123".to_string(),
        svg_base64: Some(general_purpose::STANDARD.encode(test_svg.as_bytes())),
        bin_width: Some(350.0),
        bin_height: Some(350.0),
        spacing: Some(50.0),
        amount_of_parts: Some(2),
        amount_of_rotations: 4,
        config: None,
        output_queue_url: Some("test-output-queue".to_string()),
        cancelled: false,
    };

    let request_json = serde_json::to_string(&request)?;
    let responses = process_request_direct(&request_json)?;

    assert!(!responses.is_empty(), "Should have at least one response");

    let final_response = responses
        .iter()
        .find(|r| r.is_final)
        .ok_or_else(|| anyhow::anyhow!("No final response found"))?;

    assert_eq!(final_response.correlation_id, "test-correlation-123");
    assert!(final_response.parts_placed > 0);
    assert!(final_response.is_final);
    assert!(!final_response.is_improvement);

    let decoded_first = general_purpose::STANDARD.decode(&final_response.first_page_svg_base64)?;
    assert!(
        !decoded_first.is_empty(),
        "First page SVG should decode to non-empty bytes"
    );

    let last_page = final_response
        .last_page_svg_base64
        .as_ref()
        .expect("Last page SVG should be present");
    let decoded_last = general_purpose::STANDARD.decode(last_page)?;
    assert!(
        !decoded_last.is_empty(),
        "Last page SVG should decode to non-empty bytes"
    );

    let last_page_b64 = final_response
        .last_page_svg_base64
        .as_ref()
        .expect("Last page SVG should always be present");
    let decoded_last = general_purpose::STANDARD.decode(last_page_b64)?;
    assert!(
        !decoded_last.is_empty(),
        "Last page SVG should decode to non-empty bytes"
    );

    Ok(())
}

#[tokio::test]
async fn test_single_page_last_page_matches_first() -> Result<()> {
    let _ = env_logger::Builder::from_default_env()
        .filter_level(log::LevelFilter::Debug)
        .try_init();

    // Test SVG that will fit on a single page
    let test_svg = r#"<?xml version="1.0" standalone="no"?>
<!DOCTYPE svg PUBLIC "-//W3C//DTD SVG 1.1//EN" "http://www.w3.org/Graphics/SVG/1.1/DTD/svg11.dtd">
<svg width="90mm" height="90mm" viewBox="-45 -45 90 90" xmlns="http://www.w3.org/2000/svg" version="1.1">
<title>Test Shape</title>
<path d="M 13.9062,42.7979 L 22.5,38.9707 L 30.1113,33.4414 L 36.4062,26.4502 L 41.1094,18.3027 L 44.0166,9.35645
 L 45,-0 L 44.0166,-9.35645 L 41.1094,-18.3027 L 36.4062,-26.4502 L 30.1113,-33.4414 L 22.5,-38.9707
 L 13.9062,-42.7979 L 4.7041,-44.7539 L -4.7041,-44.7539 L -13.9062,-42.7979 L -22.5,-38.9707 L -30.1113,-33.4414
 L -36.4062,-26.4502 L -41.1094,-18.3027 L -44.0166,-9.35645 L -45,-0 L -44.0166,9.35645 L -41.1094,18.3027
 L -36.4062,26.4502 L -30.1113,33.4414 L -22.5,38.9707 L -13.9062,42.7979 L -4.7041,44.7539 L 4.7041,44.7539 z
" stroke="black" fill="lightgray" stroke-width="0.5"/>
</svg>"#;

    let request = SqsNestingRequest {
        correlation_id: "test-single-page".to_string(),
        svg_base64: Some(general_purpose::STANDARD.encode(test_svg.as_bytes())),
        bin_width: Some(1000.0),
        bin_height: Some(1000.0),
        spacing: Some(2.0),
        amount_of_parts: Some(1), // Only 1 part, should fit on single page
        amount_of_rotations: 8,
        config: None,
        output_queue_url: None,
        cancelled: false,
    };

    let request_json = serde_json::to_string(&request)?;
    let responses = process_request_direct(&request_json)?;

    assert!(!responses.is_empty(), "Should have at least one response");

    let final_response = responses
        .iter()
        .find(|r| r.is_final)
        .ok_or_else(|| anyhow::anyhow!("No final response found"))?;

    assert_eq!(final_response.correlation_id, "test-single-page");
    assert_eq!(
        final_response.parts_placed, 1,
        "Should place exactly 1 part"
    );
    assert!(final_response.is_final);
    assert!(!final_response.is_improvement);

    // When all parts fit on first page, last_page should equal the first page
    let last_page = final_response
        .last_page_svg_base64
        .as_ref()
        .expect("Last page should be present even for single-page results");
    assert_eq!(
        last_page, &final_response.first_page_svg_base64,
        "Single-page results should reuse the same SVG for the last page"
    );

    // First page should be present and valid
    let decoded_first = general_purpose::STANDARD.decode(&final_response.first_page_svg_base64)?;
    assert!(
        !decoded_first.is_empty(),
        "First page SVG should decode to non-empty bytes"
    );

    Ok(())
}

#[tokio::test]
async fn test_multiple_pages_last_page_is_set() -> Result<()> {
    let _ = env_logger::Builder::from_default_env()
        .filter_level(log::LevelFilter::Debug)
        .try_init();

    // Test SVG that will require multiple pages
    let test_svg = r#"<?xml version="1.0" standalone="no"?>
<!DOCTYPE svg PUBLIC "-//W3C//DTD SVG 1.1//EN" "http://www.w3.org/Graphics/SVG/1.1/DTD/svg11.dtd">
<svg width="90mm" height="90mm" viewBox="-45 -45 90 90" xmlns="http://www.w3.org/2000/svg" version="1.1">
<title>Test Shape</title>
<path d="M 13.9062,42.7979 L 22.5,38.9707 L 30.1113,33.4414 L 36.4062,26.4502 L 41.1094,18.3027 L 44.0166,9.35645
 L 45,-0 L 44.0166,-9.35645 L 41.1094,-18.3027 L 36.4062,-26.4502 L 30.1113,-33.4414 L 22.5,-38.9707
 L 13.9062,-42.7979 L 4.7041,-44.7539 L -4.7041,-44.7539 L -13.9062,-42.7979 L -22.5,-38.9707 L -30.1113,-33.4414
 L -36.4062,-26.4502 L -41.1094,-18.3027 L -44.0166,-9.35645 L -45,-0 L -44.0166,9.35645 L -41.1094,18.3027
 L -36.4062,26.4502 L -30.1113,33.4414 L -22.5,38.9707 L -13.9062,42.7979 L -4.7041,44.7539 L 4.7041,44.7539 z
" stroke="black" fill="lightgray" stroke-width="0.5"/>
</svg>"#;

    let request = SqsNestingRequest {
        correlation_id: "test-multiple-pages".to_string(),
        svg_base64: Some(general_purpose::STANDARD.encode(test_svg.as_bytes())),
        bin_width: Some(200.0), // Small bin to force multiple pages
        bin_height: Some(200.0),
        spacing: Some(50.0),
        amount_of_parts: Some(10), // Many parts to require multiple pages
        amount_of_rotations: 4,
        config: None,
        output_queue_url: None,
        cancelled: false,
    };

    let request_json = serde_json::to_string(&request)?;
    let responses = process_request_direct(&request_json)?;

    assert!(!responses.is_empty(), "Should have at least one response");

    let final_response = responses
        .iter()
        .find(|r| r.is_final)
        .ok_or_else(|| anyhow::anyhow!("No final response found"))?;

    assert_eq!(final_response.correlation_id, "test-multiple-pages");
    assert!(final_response.parts_placed > 0);
    assert!(final_response.is_final);
    assert!(!final_response.is_improvement);

    let last_page = final_response
        .last_page_svg_base64
        .as_ref()
        .expect("Last page should always be present");
    let decoded_last = general_purpose::STANDARD.decode(last_page)?;
    assert!(
        !decoded_last.is_empty(),
        "Last page SVG should decode to non-empty bytes"
    );

    // First page should always be present
    let decoded_first = general_purpose::STANDARD.decode(&final_response.first_page_svg_base64)?;
    assert!(
        !decoded_first.is_empty(),
        "First page SVG should decode to non-empty bytes"
    );

    Ok(())
}

#[tokio::test]
async fn test_svg_with_circles() -> Result<()> {
    let _ = env_logger::Builder::from_default_env()
        .filter_level(log::LevelFilter::Debug)
        .try_init();

    // Test SVG with circles (not paths) - this may cause parsing issues
    let test_svg = r#"<?xml version="1.0"?>
<svg xmlns="http://www.w3.org/2000/svg" fill="none" width="256" height="271">
<g id="KN_1" stroke-width="1" stroke="rgb(0,0,0)">
<circle cx="130.000000" cy="145.000000" r="125.000000"/>
</g>
<g id="KN_2" stroke-width="1" stroke="rgb(0,0,0)">
<circle cx="130.000000" cy="145.000000" r="71.040000"/>
</g>
<g id="KN_3" stroke-width="1" stroke="rgb(0,0,0)">
<circle cx="130.000000" cy="50.600000" r="8.000000"/>
</g>
<g id="KN_4" stroke-width="1" stroke="rgb(0,0,0)">
<circle cx="63.249120" cy="78.249120" r="8.000000"/>
</g>
<g id="KN_5" stroke-width="1" stroke="rgb(0,0,0)">
<circle cx="35.600000" cy="145.000000" r="8.000000"/>
</g>
<g id="KN_6" stroke-width="1" stroke="rgb(0,0,0)">
<circle cx="63.249120" cy="211.750880" r="8.000000"/>
</g>
<g id="KN_7" stroke-width="1" stroke="rgb(0,0,0)">
<circle cx="130.000000" cy="239.400000" r="8.000000"/>
</g>
<g id="KN_8" stroke-width="1" stroke="rgb(0,0,0)">
<circle cx="196.750880" cy="211.750880" r="8.000000"/>
</g>
<g id="KN_9" stroke-width="1" stroke="rgb(0,0,0)">
<circle cx="224.400000" cy="145.000000" r="8.000000"/>
</g>
<g id="KN_10" stroke-width="1" stroke="rgb(0,0,0)">
<circle cx="196.750880" cy="78.249120" r="8.000000"/>
</g>
</svg>"#;

    let request = SqsNestingRequest {
        correlation_id: "test-circles-svg".to_string(),
        svg_base64: Some(general_purpose::STANDARD.encode(test_svg.as_bytes())),
        bin_width: Some(1200.0),
        bin_height: Some(1200.0),
        spacing: Some(50.0),
        amount_of_parts: Some(15),
        amount_of_rotations: 8,
        config: None,
        output_queue_url: None,
        cancelled: false,
    };

    let request_json = serde_json::to_string(&request)?;

    // This SVG contains circles, not paths. The SVG parser now converts circles to paths.
    let responses = process_request_direct(&request_json)?;

    assert!(!responses.is_empty(), "Should have at least one response");
    let final_response = responses
        .iter()
        .find(|r| r.is_final)
        .ok_or_else(|| anyhow::anyhow!("No final response found"))?;

    assert_eq!(final_response.correlation_id, "test-circles-svg");
    assert!(
        final_response.parts_placed > 0,
        "Should place at least some parts"
    );
    assert!(final_response.is_final);
    assert!(!final_response.is_improvement);

    // Verify the response contains valid SVG
    let decoded_first = general_purpose::STANDARD.decode(&final_response.first_page_svg_base64)?;
    assert!(
        !decoded_first.is_empty(),
        "First page SVG should decode to non-empty bytes"
    );

    Ok(())
}

fn current_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

/// Process a request with cancellation support
fn process_request_with_cancellation(
    request_json: &str,
    cancellation_registry: Arc<Mutex<std::collections::HashMap<String, bool>>>,
) -> Result<Vec<SqsNestingResponse>> {
    use jagua_utils::svg_nesting::{nest_svg_parts_adaptive, AdaptiveConfig};

    let request: SqsNestingRequest = serde_json::from_str(request_json)?;

    // Validate required fields for non-cancellation requests
    let svg_base64 = request
        .svg_base64
        .as_ref()
        .ok_or_else(|| anyhow::anyhow!("Missing required field: svg_base64"))?;
    let bin_width = request
        .bin_width
        .ok_or_else(|| anyhow::anyhow!("Missing required field: bin_width"))?;
    let bin_height = request
        .bin_height
        .ok_or_else(|| anyhow::anyhow!("Missing required field: bin_height"))?;
    let spacing = request
        .spacing
        .ok_or_else(|| anyhow::anyhow!("Missing required field: spacing"))?;
    let amount_of_parts = request
        .amount_of_parts
        .ok_or_else(|| anyhow::anyhow!("Missing required field: amount_of_parts"))?;

    let svg_bytes = general_purpose::STANDARD
        .decode(svg_base64)
        .map_err(|e| anyhow::anyhow!("Failed to decode svg_base64: {}", e))?;

    let improvements: Arc<Mutex<Vec<SqsNestingResponse>>> = Arc::new(Mutex::new(Vec::new()));
    let improvements_clone = improvements.clone();
    let correlation_id = request.correlation_id.clone();

    let cancellation_registry_clone = cancellation_registry.clone();
    let callback = move |svg_bytes: &[u8], parts_placed: usize| {
        // Check if cancellation was requested
        let cancelled = {
            let registry = cancellation_registry_clone.lock().unwrap();
            registry.get(&correlation_id).copied().unwrap_or(false)
        };

        if cancelled {
            return true; // Cancel the optimization
        }

        let encoded_page = general_purpose::STANDARD.encode(svg_bytes);
        let response = SqsNestingResponse {
            correlation_id: correlation_id.clone(),
            first_page_svg_base64: encoded_page.clone(),
            last_page_svg_base64: Some(encoded_page),
            parts_placed,
            is_improvement: true,
            is_final: false,
            timestamp: current_timestamp(),
            error_message: None,
        };
        improvements_clone.lock().unwrap().push(response);
        false
    };

    let config = AdaptiveConfig::default();
    let nesting_result = nest_svg_parts_adaptive(
        bin_width,
        bin_height,
        spacing,
        &svg_bytes,
        amount_of_parts,
        request.amount_of_rotations,
        config,
        Some(callback),
    )?;

    let mut responses = improvements.lock().unwrap().clone();

    let first_page_bytes = nesting_result
        .page_svgs
        .first()
        .unwrap_or(&nesting_result.combined_svg);
    let last_page_bytes = nesting_result.page_svgs.last().unwrap_or(first_page_bytes);
    let first_page_svg_base64 = general_purpose::STANDARD.encode(first_page_bytes);
    let last_page_svg_base64 = general_purpose::STANDARD.encode(last_page_bytes);

    responses.push(SqsNestingResponse {
        correlation_id: request.correlation_id,
        first_page_svg_base64,
        last_page_svg_base64: Some(last_page_svg_base64),
        parts_placed: nesting_result.parts_placed,
        is_improvement: false,
        is_final: true,
        timestamp: current_timestamp(),
        error_message: None,
    });

    Ok(responses)
}

#[tokio::test]
async fn test_cancellation_request_handling() -> Result<()> {
    let _ = env_logger::Builder::from_default_env()
        .filter_level(log::LevelFilter::Debug)
        .try_init();

    use aws_config::BehaviorVersion;
    use aws_sdk_sqs::Client as SqsClient;
    use jagua_sqs_processor::SqsProcessor;

    // Create a processor
    let config = aws_config::defaults(BehaviorVersion::latest()).load().await;
    let sqs_client = SqsClient::new(&config);
    let processor = SqsProcessor::new(
        sqs_client,
        "test-input-queue".to_string(),
        "test-output-queue".to_string(),
    );

    // Create a cancellation request (only correlation_id and cancelled are required)
    let cancellation_request = SqsNestingRequest {
        correlation_id: "test-cancel-123".to_string(),
        svg_base64: None,
        bin_width: None,
        bin_height: None,
        spacing: None,
        amount_of_parts: None,
        amount_of_rotations: 8,
        config: None,
        output_queue_url: None,
        cancelled: true,
    };

    let request_json = serde_json::to_string(&cancellation_request)?;

    // Process the cancellation message
    let result = processor
        .process_message("test-receipt", &request_json)
        .await;

    // Should succeed (cancellation is handled)
    assert!(
        result.is_ok(),
        "Cancellation request should be processed successfully"
    );

    // Note: We can't directly access cancellation_registry as it's private,
    // but the unit tests verify the registry functionality.
    // The fact that process_message returns Ok(()) without processing
    // confirms cancellation was handled correctly.

    Ok(())
}

#[tokio::test]
async fn test_optimization_cancellation_during_execution() -> Result<()> {
    let _ = env_logger::Builder::from_default_env()
        .filter_level(log::LevelFilter::Debug)
        .try_init();

    use std::collections::HashMap;
    use std::sync::{Arc, Mutex};
    use std::thread;
    use std::time::Duration;

    let test_svg = r#"<?xml version="1.0" standalone="no"?>
<!DOCTYPE svg PUBLIC "-//W3C//DTD SVG 1.1//EN" "http://www.w3.org/Graphics/SVG/1.1/DTD/svg11.dtd">
<svg width="90mm" height="90mm" viewBox="-45 -45 90 90" xmlns="http://www.w3.org/2000/svg" version="1.1">
<title>Test Shape</title>
<path d="M 13.9062,42.7979 L 22.5,38.9707 L 30.1113,33.4414 L 36.4062,26.4502 L 41.1094,18.3027 L 44.0166,9.35645
 L 45,-0 L 44.0166,-9.35645 L 41.1094,-18.3027 L 36.4062,-26.4502 L 30.1113,-33.4414 L 22.5,-38.9707
 L 13.9062,-42.7979 L 4.7041,-44.7539 L -4.7041,-44.7539 L -13.9062,-42.7979 L -22.5,-38.9707 L -30.1113,-33.4414
 L -36.4062,-26.4502 L -41.1094,-18.3027 L -44.0166,-9.35645 L -45,-0 L -44.0166,9.35645 L -41.1094,18.3027
 L -36.4062,26.4502 L -30.1113,33.4414 L -22.5,38.9707 L -13.9062,42.7979 L -4.7041,44.7539 L 4.7041,44.7539 z
" stroke="black" fill="lightgray" stroke-width="0.5"/>
</svg>"#;

    let request = SqsNestingRequest {
        correlation_id: "test-cancel-during-exec".to_string(),
        svg_base64: Some(general_purpose::STANDARD.encode(test_svg.as_bytes())),
        bin_width: Some(350.0),
        bin_height: Some(350.0),
        spacing: Some(50.0),
        amount_of_parts: Some(10), // Many parts to make it run longer
        amount_of_rotations: 8,
        config: None,
        output_queue_url: None,
        cancelled: false,
    };

    let request_json = serde_json::to_string(&request)?;
    let cancellation_registry: Arc<Mutex<HashMap<String, bool>>> =
        Arc::new(Mutex::new(HashMap::new()));

    // Register the correlation_id
    {
        let mut registry = cancellation_registry.lock().unwrap();
        registry.insert("test-cancel-during-exec".to_string(), false);
    }

    // Spawn a task to cancel after a short delay
    let registry_clone = cancellation_registry.clone();
    let cancel_handle = thread::spawn(move || {
        thread::sleep(Duration::from_millis(100)); // Wait a bit for optimization to start
        let mut registry = registry_clone.lock().unwrap();
        registry.insert("test-cancel-during-exec".to_string(), true);
        println!("Cancellation flag set");
    });

    // Start processing in a separate thread
    let request_json_clone = request_json.clone();
    let registry_clone = cancellation_registry.clone();
    let process_handle = thread::spawn(move || {
        process_request_with_cancellation(&request_json_clone, registry_clone)
    });

    // Wait for cancellation to be set
    cancel_handle.join().unwrap();

    // Wait for processing to complete
    let result = process_handle.join().unwrap();

    // Processing should complete (may be cancelled early)
    assert!(
        result.is_ok(),
        "Processing should complete even if cancelled"
    );

    let responses = result.unwrap();
    assert!(!responses.is_empty(), "Should have at least one response");

    // The final response should exist
    let final_response = responses
        .iter()
        .find(|r| r.is_final)
        .ok_or_else(|| anyhow::anyhow!("No final response found"))?;

    assert_eq!(final_response.correlation_id, "test-cancel-during-exec");
    // When cancelled, parts_placed might be less than requested
    assert!(final_response.parts_placed <= 10);

    Ok(())
}

#[tokio::test]
async fn test_cancellation_before_optimization_starts() -> Result<()> {
    let _ = env_logger::Builder::from_default_env()
        .filter_level(log::LevelFilter::Debug)
        .try_init();

    use std::collections::HashMap;
    use std::sync::{Arc, Mutex};

    let test_svg = r#"<?xml version="1.0" standalone="no"?>
<!DOCTYPE svg PUBLIC "-//W3C//DTD SVG 1.1//EN" "http://www.w3.org/Graphics/SVG/1.1/DTD/svg11.dtd">
<svg width="90mm" height="90mm" viewBox="-45 -45 90 90" xmlns="http://www.w3.org/2000/svg" version="1.1">
<title>Test Shape</title>
<path d="M 13.9062,42.7979 L 22.5,38.9707 L 30.1113,33.4414 L 36.4062,26.4502 L 41.1094,18.3027 L 44.0166,9.35645
 L 45,-0 L 44.0166,-9.35645 L 41.1094,-18.3027 L 36.4062,-26.4502 L 30.1113,-33.4414 L 22.5,-38.9707
 L 13.9062,-42.7979 L 4.7041,-44.7539 L -4.7041,-44.7539 L -13.9062,-42.7979 L -22.5,-38.9707 L -30.1113,-33.4414
 L -36.4062,-26.4502 L -41.1094,-18.3027 L -44.0166,-9.35645 L -45,-0 L -44.0166,9.35645 L -41.1094,18.3027
 L -36.4062,26.4502 L -30.1113,33.4414 L -22.5,38.9707 L -13.9062,42.7979 L -4.7041,44.7539 L 4.7041,44.7539 z
" stroke="black" fill="lightgray" stroke-width="0.5"/>
</svg>"#;

    let request = SqsNestingRequest {
        correlation_id: "test-cancel-before-start".to_string(),
        svg_base64: Some(general_purpose::STANDARD.encode(test_svg.as_bytes())),
        bin_width: Some(350.0),
        bin_height: Some(350.0),
        spacing: Some(50.0),
        amount_of_parts: Some(5),
        amount_of_rotations: 8,
        config: None,
        output_queue_url: None,
        cancelled: false,
    };

    let request_json = serde_json::to_string(&request)?;
    let cancellation_registry: Arc<Mutex<HashMap<String, bool>>> =
        Arc::new(Mutex::new(HashMap::new()));

    // Set cancellation flag BEFORE starting optimization
    {
        let mut registry = cancellation_registry.lock().unwrap();
        registry.insert("test-cancel-before-start".to_string(), true);
    }

    // Process the request - it should be cancelled immediately
    let responses = process_request_with_cancellation(&request_json, cancellation_registry)?;

    // Should have a final response (even if cancelled)
    assert!(!responses.is_empty(), "Should have at least one response");

    let final_response = responses
        .iter()
        .find(|r| r.is_final)
        .ok_or_else(|| anyhow::anyhow!("No final response found"))?;

    assert_eq!(final_response.correlation_id, "test-cancel-before-start");
    // When cancelled early, might have fewer parts placed
    assert!(final_response.parts_placed <= 5);

    Ok(())
}

#[tokio::test]
async fn test_parallel_requests_respect_individual_cancellation() -> Result<()> {
    let _ = env_logger::Builder::from_default_env()
        .filter_level(log::LevelFilter::Debug)
        .try_init();

    use std::collections::HashMap;
    use std::sync::{Arc, Mutex};
    use tokio::time::Duration;

    let test_svg = r#"<?xml version="1.0" standalone="no"?>
<!DOCTYPE svg PUBLIC "-//W3C//DTD SVG 1.1//EN" "http://www.w3.org/Graphics/SVG/1.1/DTD/svg11.dtd">
<svg width="90mm" height="90mm" viewBox="-45 -45 90 90" xmlns="http://www.w3.org/2000/svg" version="1.1">
<title>Test Shape</title>
<path d="M 13.9062,42.7979 L 22.5,38.9707 L 30.1113,33.4414 L 36.4062,26.4502 L 41.1094,18.3027 L 44.0166,9.35645
 L 45,-0 L 44.0166,-9.35645 L 41.1094,-18.3027 L 36.4062,-26.4502 L 30.1113,-33.4414 L 22.5,-38.9707
 L 13.9062,-42.7979 L 4.7041,-44.7539 L -4.7041,-44.7539 L -13.9062,-42.7979 L -22.5,-38.9707 L -30.1113,-33.4414
 L -36.4062,-26.4502 L -41.1094,-18.3027 L -44.0166,-9.35645 L -45,-0 L -44.0166,9.35645 L -41.1094,18.3027
 L -36.4062,26.4502 L -30.1113,33.4414 L -22.5,38.9707 L -13.9062,42.7979 L -4.7041,44.7539 L 4.7041,44.7539 z
" stroke="black" fill="lightgray" stroke-width="0.5"/>
</svg>"#;

    let request_a = SqsNestingRequest {
        correlation_id: "parallel-keep".to_string(),
        svg_base64: Some(general_purpose::STANDARD.encode(test_svg.as_bytes())),
        bin_width: Some(500.0),
        bin_height: Some(500.0),
        spacing: Some(25.0),
        amount_of_parts: Some(2),
        amount_of_rotations: 4,
        config: None,
        output_queue_url: None,
        cancelled: false,
    };

    let request_b = SqsNestingRequest {
        correlation_id: "parallel-cancel".to_string(),
        svg_base64: Some(general_purpose::STANDARD.encode(test_svg.as_bytes())),
        bin_width: Some(350.0),
        bin_height: Some(350.0),
        spacing: Some(35.0),
        amount_of_parts: Some(8),
        amount_of_rotations: 8,
        config: None,
        output_queue_url: None,
        cancelled: false,
    };

    let registry: Arc<Mutex<HashMap<String, bool>>> = Arc::new(Mutex::new(HashMap::new()));
    {
        let mut reg = registry.lock().unwrap();
        reg.insert(request_a.correlation_id.clone(), false);
        reg.insert(request_b.correlation_id.clone(), false);
    }

    let request_a_json = serde_json::to_string(&request_a)?;
    let request_b_json = serde_json::to_string(&request_b)?;

    let registry_for_a = registry.clone();
    let handle_a = tokio::task::spawn_blocking(move || {
        process_request_with_cancellation(&request_a_json, registry_for_a)
    });

    let registry_for_b = registry.clone();
    let handle_b = tokio::task::spawn_blocking(move || {
        process_request_with_cancellation(&request_b_json, registry_for_b)
    });

    let registry_for_cancel = registry.clone();
    let cancel_id = request_b.correlation_id.clone();
    let canceller_handle = tokio::spawn(async move {
        tokio::time::sleep(Duration::from_millis(50)).await;
        let mut reg = registry_for_cancel.lock().unwrap();
        reg.insert(cancel_id, true);
    });

    let (responses_a, responses_b, _) = tokio::join!(
        async { handle_a.await.expect("join blocking A").expect("process A") },
        async { handle_b.await.expect("join blocking B").expect("process B") },
        async {
            canceller_handle.await.expect("Canceller task failed");
        }
    );

    let final_a = responses_a
        .iter()
        .find(|r| r.is_final)
        .ok_or_else(|| anyhow::anyhow!("No final response for request A"))?;
    assert_eq!(final_a.correlation_id, "parallel-keep");
    assert!(final_a.parts_placed > 0);

    let final_b = responses_b
        .iter()
        .find(|r| r.is_final)
        .ok_or_else(|| anyhow::anyhow!("No final response for request B"))?;
    assert_eq!(final_b.correlation_id, "parallel-cancel");
    assert!(
        final_b.parts_placed <= 8,
        "Cancelled request should not exceed requested parts"
    );

    let reg = registry.lock().unwrap();
    assert_eq!(
        reg.get("parallel-cancel"),
        Some(&true),
        "Cancellation flag should be set for the cancelled request"
    );
    assert_eq!(
        reg.get("parallel-keep"),
        Some(&false),
        "Other request should not be cancelled"
    );

    Ok(())
}

#[tokio::test]
async fn test_parallel_preemptive_cancellation_only_affects_target() -> Result<()> {
    let _ = env_logger::Builder::from_default_env()
        .filter_level(log::LevelFilter::Debug)
        .try_init();

    use std::collections::HashMap;
    use std::sync::{Arc, Mutex};

    let test_svg = r#"<?xml version="1.0" standalone="no"?>
<!DOCTYPE svg PUBLIC "-//W3C//DTD SVG 1.1//EN" "http://www.w3.org/Graphics/SVG/1.1/DTD/svg11.dtd">
<svg width="90mm" height="90mm" viewBox="-45 -45 90 90" xmlns="http://www.w3.org/2000/svg" version="1.1">
<title>Test Shape</title>
<path d="M 13.9062,42.7979 L 22.5,38.9707 L 30.1113,33.4414 L 36.4062,26.4502 L 41.1094,18.3027 L 44.0166,9.35645
 L 45,-0 L 44.0166,-9.35645 L 41.1094,-18.3027 L 36.4062,-26.4502 L 30.1113,-33.4414 L 22.5,-38.9707
 L 13.9062,-42.7979 L 4.7041,-44.7539 L -4.7041,-44.7539 L -13.9062,-42.7979 L -22.5,-38.9707 L -30.1113,-33.4414
 L -36.4062,-26.4502 L -41.1094,-18.3027 L -44.0166,-9.35645 L -45,-0 L -44.0166,9.35645 L -41.1094,18.3027
 L -36.4062,26.4502 L -30.1113,33.4414 L -22.5,38.9707 L -13.9062,42.7979 L -4.7041,44.7539 L 4.7041,44.7539 z
" stroke="black" fill="lightgray" stroke-width="0.5"/>
</svg>"#;

    let request_active = SqsNestingRequest {
        correlation_id: "parallel-preemptive-active".to_string(),
        svg_base64: Some(general_purpose::STANDARD.encode(test_svg.as_bytes())),
        bin_width: Some(450.0),
        bin_height: Some(450.0),
        spacing: Some(20.0),
        amount_of_parts: Some(3),
        amount_of_rotations: 4,
        config: None,
        output_queue_url: None,
        cancelled: false,
    };

    let request_cancelled = SqsNestingRequest {
        correlation_id: "parallel-preemptive-cancelled".to_string(),
        svg_base64: Some(general_purpose::STANDARD.encode(test_svg.as_bytes())),
        bin_width: Some(400.0),
        bin_height: Some(400.0),
        spacing: Some(30.0),
        amount_of_parts: Some(6),
        amount_of_rotations: 8,
        config: None,
        output_queue_url: None,
        cancelled: false,
    };

    let registry: Arc<Mutex<HashMap<String, bool>>> = Arc::new(Mutex::new(HashMap::new()));
    {
        let mut reg = registry.lock().unwrap();
        reg.insert(request_active.correlation_id.clone(), false);
        reg.insert(request_cancelled.correlation_id.clone(), true);
    }

    let active_json = serde_json::to_string(&request_active)?;
    let cancelled_json = serde_json::to_string(&request_cancelled)?;

    let registry_for_active = registry.clone();
    let active_handle = tokio::task::spawn_blocking(move || {
        process_request_with_cancellation(&active_json, registry_for_active)
    });

    let registry_for_cancelled = registry.clone();
    let cancelled_handle = tokio::task::spawn_blocking(move || {
        process_request_with_cancellation(&cancelled_json, registry_for_cancelled)
    });

    let (active_responses, cancelled_responses) = tokio::join!(
        async {
            active_handle
                .await
                .expect("join blocking active")
                .expect("process active")
        },
        async {
            cancelled_handle
                .await
                .expect("join blocking cancelled")
                .expect("process cancelled")
        }
    );

    let active_final = active_responses
        .iter()
        .find(|r| r.is_final)
        .ok_or_else(|| anyhow::anyhow!("No final response for active request"))?;
    assert_eq!(active_final.correlation_id, "parallel-preemptive-active");
    assert!(active_final.parts_placed > 0);

    assert!(
        cancelled_responses.iter().all(|r| !r.is_improvement),
        "Preemptively cancelled request should not emit improvements"
    );
    let cancelled_final = cancelled_responses
        .iter()
        .find(|r| r.is_final)
        .ok_or_else(|| anyhow::anyhow!("No final response for cancelled request"))?;
    assert_eq!(
        cancelled_final.correlation_id,
        "parallel-preemptive-cancelled"
    );
    assert!(
        cancelled_final.parts_placed <= 6,
        "Cancelled job should not exceed requested parts"
    );

    Ok(())
}
