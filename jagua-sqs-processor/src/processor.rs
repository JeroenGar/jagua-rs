use anyhow::{anyhow, Context, Result};
use aws_sdk_sqs::Client as SqsClient;
use aws_sdk_s3::Client as S3Client;
use base64::{engine::general_purpose, Engine as _};
use futures::StreamExt;
use jagua_utils::svg_nesting::{NestingStrategy, AdaptiveNestingStrategy};
use log::{debug, error, info};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::sync::broadcast;
use tokio::sync::mpsc;
use tokio::sync::Semaphore;

/// Request message structure for SQS queue
/// For cancellation requests, only `correlation_id` and `cancelled: true` are required.
/// All other fields are required only when `cancelled` is false or not present.
#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SqsNestingRequest {
    /// Unique identifier for tracking the request
    pub correlation_id: String,
    /// Base64-encoded SVG payload (deprecated, use svg_url instead)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub svg_base64: Option<String>,
    /// S3 URL to the input SVG file (format: s3://bucket/key or https://bucket.s3.region.amazonaws.com/key)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub svg_url: Option<String>,
    /// Bin width for nesting (required if not cancelled)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bin_width: Option<f32>,
    /// Bin height for nesting (required if not cancelled)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bin_height: Option<f32>,
    /// Spacing between parts (required if not cancelled)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub spacing: Option<f32>,
    /// Number of parts to nest (required if not cancelled)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub amount_of_parts: Option<usize>,
    /// Number of rotations to try (default: 8)
    #[serde(default = "default_rotations")]
    pub amount_of_rotations: usize,
    /// Output queue URL for results (falls back to default if omitted)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub output_queue_url: Option<String>,
    /// Whether this is a cancellation request
    #[serde(default)]
    pub cancelled: bool,
}

fn encode_svg(bytes: &[u8]) -> String {
    general_purpose::STANDARD.encode(bytes)
}

/// Generate an empty page SVG (used when all parts are placed)
fn generate_empty_page_svg(bin_width: f32, bin_height: f32) -> Vec<u8> {
    format!(
        r#"<?xml version="1.0" encoding="UTF-8"?>
<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {} {}">
  <g id="container_0">
    <path d="M 0,0 L {},0 L {},{} L 0,{} z" fill="transparent" stroke="gray" stroke-width="1"/>
  </g>
  <text x="{}" y="{}" font-size="{}" font-family="monospace">Unplaced parts: 0</text>
</svg>"#,
        bin_width,
        bin_height,
        bin_width,
        bin_width,
        bin_height,
        bin_height,
        bin_width * 0.02,
        bin_height * 0.05,
        bin_width * 0.02
    )
    .into_bytes()
}

fn sanitize_svg_fields(response: &SqsNestingResponse) -> SqsNestingResponse {
    let mut sanitized = response.clone();
    // URLs are already small, no need to sanitize
    sanitized
}

fn decode_svg(encoded: &str) -> Result<Vec<u8>> {
    general_purpose::STANDARD
        .decode(encoded)
        .map_err(|e| anyhow!("Failed to decode svg_base64: {}", e))
}

fn current_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

fn default_rotations() -> usize {
    8
}

/// Response message structure for SQS queue
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SqsNestingResponse {
    /// Correlation ID from request
    pub correlation_id: String,
    /// S3 URL to the first page SVG (format: s3://bucket/nesting/{requestId}/first-page.svg)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_page_svg_url: Option<String>,
    /// S3 URL to the last page SVG (format: s3://bucket/nesting/{requestId}/last-page.svg)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_page_svg_url: Option<String>,
    /// Number of parts placed
    pub parts_placed: usize,
    /// Whether this is an intermediate improvement (always false for simple strategy)
    #[serde(rename = "improvement")]
    pub is_improvement: bool,
    /// Whether this is the final result (always true for simple strategy)
    #[serde(rename = "final")]
    pub is_final: bool,
    /// Timestamp in seconds since epoch
    pub timestamp: u64,
    /// Error message if processing failed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
}

/// SQS Processor for handling SVG nesting requests
#[derive(Clone)]
pub struct SqsProcessor {
    sqs_client: SqsClient,
    s3_client: S3Client,
    s3_bucket: String,
    input_queue_url: String,
    output_queue_url: String,
    cancellation_registry: Arc<Mutex<HashMap<String, bool>>>,
}

impl SqsProcessor {
    fn mark_cancelled(&self, correlation_id: &str) -> bool {
        let mut registry = self.cancellation_registry.lock().unwrap();
        registry.insert(correlation_id.to_string(), true).is_some()
    }

    /// Create a new SQS processor
    pub fn new(
        sqs_client: SqsClient,
        s3_client: S3Client,
        s3_bucket: String,
        input_queue_url: String,
        output_queue_url: String,
    ) -> Self {
        Self {
            sqs_client,
            s3_client,
            s3_bucket,
            input_queue_url,
            output_queue_url,
            cancellation_registry: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Upload SVG to S3 and return the S3 URL
    async fn upload_svg_to_s3(
        &self,
        svg_bytes: &[u8],
        request_id: &str,
        filename: &str,
    ) -> Result<String> {
        upload_svg_to_s3_internal(&self.s3_client, &self.s3_bucket, svg_bytes, request_id, filename).await
    }

    /// Download SVG from S3 URL
    async fn download_svg_from_s3(&self, s3_url: &str) -> Result<Vec<u8>> {
        // Parse S3 URL (supports both s3://bucket/key and https://bucket.s3.region.amazonaws.com/key)
        let (bucket, key) = parse_s3_url(s3_url)?;
        
        info!("Downloading SVG from S3: url={}, bucket={}, key={}", s3_url, bucket, key);

        let response = match self.s3_client
            .get_object()
            .bucket(&bucket)
            .key(&key)
            .send()
            .await
        {
            Ok(resp) => resp,
            Err(e) => {
                // Log detailed error information
                error!("S3 GetObject failed: {}", e);
                error!("S3 URL: {}, bucket: {}, key: {}", s3_url, bucket, key);
                
                // Try to extract more error details
                use aws_sdk_s3::error::ProvideErrorMetadata;
                if let Some(code) = e.code() {
                    error!("S3 error code: {}", code);
                }
                if let Some(message) = e.message() {
                    error!("S3 error message: {}", message);
                }
                
                // Log the full error
                error!("Full error details: {}", e);
                
                return Err(anyhow::anyhow!(
                    "Failed to download SVG from S3: bucket={}, key={}, error={}",
                    bucket, key, e
                ));
            }
        };

        // Collect the body stream into bytes
        let mut svg_bytes = Vec::new();
        let mut body_stream = response.body;
        use futures::StreamExt;
        while let Some(chunk_result) = body_stream.next().await {
            let chunk = chunk_result.context("Failed to read chunk from S3 object body")?;
            svg_bytes.extend_from_slice(&chunk);
        }
        info!("Downloaded SVG from S3: {} bytes", svg_bytes.len());
        Ok(svg_bytes)
    }
}

/// Parse S3 URL and extract bucket and key
fn parse_s3_url(s3_url: &str) -> Result<(String, String)> {
    // Handle s3://bucket/key format
    if s3_url.starts_with("s3://") {
        let path = &s3_url[5..];
        if let Some(slash_pos) = path.find('/') {
            let bucket = path[..slash_pos].to_string();
            let key = path[slash_pos + 1..].to_string();
            return Ok((bucket, key));
        }
        return Err(anyhow!("Invalid S3 URL format: {}", s3_url));
    }
    
    // Handle https://bucket.s3.region.amazonaws.com/key format
    if s3_url.starts_with("https://") {
        let url = s3_url.strip_prefix("https://").unwrap();
        // Extract bucket (first part before .s3)
        if let Some(s3_pos) = url.find(".s3") {
            let bucket = url[..s3_pos].to_string();
            // Extract key (everything after .amazonaws.com/)
            if let Some(aws_pos) = url.find(".amazonaws.com/") {
                let key = url[aws_pos + 15..].to_string();
                return Ok((bucket, key));
            }
        }
        return Err(anyhow!("Invalid S3 HTTPS URL format: {}", s3_url));
    }
    
    Err(anyhow!("Unsupported S3 URL format: {}", s3_url))
}

/// Internal helper function to upload SVG to S3 (used by both improvement and final responses)
async fn upload_svg_to_s3_internal(
    s3_client: &S3Client,
    s3_bucket: &str,
    svg_bytes: &[u8],
    request_id: &str,
    filename: &str,
) -> Result<String> {
    let s3_key = format!("nesting/{}/{}", request_id, filename);
    let s3_url = format!("s3://{}/{}", s3_bucket, s3_key);
    
    info!("Uploading SVG to S3: bucket={}, key={}, size={} bytes", 
        s3_bucket, s3_key, svg_bytes.len());

    s3_client
        .put_object()
        .bucket(s3_bucket)
        .key(&s3_key)
        .body(aws_sdk_s3::primitives::ByteStream::from(svg_bytes.to_vec()))
        .content_type("image/svg+xml")
        .send()
        .await
        .with_context(|| {
            format!(
                "Failed to upload SVG to S3: bucket={}, key={}",
                s3_bucket, s3_key
            )
        })?;

    info!("Successfully uploaded SVG to S3: {}", s3_url);
    Ok(s3_url)
}

impl SqsProcessor {
    /// Send message to output queue
    /// Helper function to send a message to SQS (used by both error and improvement responses)
    async fn send_message_to_sqs(
        sqs_client: &SqsClient,
        queue_url: &str,
        response: &SqsNestingResponse,
    ) -> Result<()> {
        let message_body =
            serde_json::to_string(response).context("Failed to serialize response")?;

        // Check message size (SQS limit is 1 MiB = 1,048,576 bytes)
        let message_size_kb = message_body.len() / 1024;
        const SQS_MAX_SIZE: usize = 1024 * 1024; // 1 MiB
        if message_body.len() > SQS_MAX_SIZE {
            return Err(anyhow!(
                "Message size {} KB exceeds SQS limit of {} KB (1 MiB)",
                message_size_kb,
                SQS_MAX_SIZE / 1024
            ));
        }

        debug!(
            "Sending message to output queue: correlation_id={}, is_final={}, size={} KB",
            response.correlation_id, response.is_final, message_size_kb
        );

        sqs_client
            .send_message()
            .queue_url(queue_url)
            .message_body(&message_body)
            .send()
            .await
            .with_context(|| {
                format!(
                    "Failed to send message to queue {}: correlation_id={}, size={} KB",
                    queue_url, response.correlation_id, message_size_kb
                )
            })?;

        Ok(())
    }

    pub async fn send_to_output_queue(
        &self,
        queue_url: &str,
        response: &SqsNestingResponse,
    ) -> Result<()> {
        let sanitized_response = sanitize_svg_fields(response);
        let sanitized_body = serde_json::to_string(&sanitized_response)
            .unwrap_or_else(|_| "<failed to serialize sanitized response>".to_string());

        Self::send_message_to_sqs(&self.sqs_client, queue_url, response).await?;

        info!(
            "Emitted response to {} with stripped payload: {}",
            queue_url, sanitized_body
        );

        Ok(())
    }

    /// Process a single message from the queue
    /// Returns Ok(()) on success, or sends error response and returns Ok(()) on error
    /// (message should always be acknowledged after calling this)
    pub async fn process_message(&self, _receipt_handle: &str, body: &str) -> Result<()> {
        // Parse request - if this fails, we can't get correlation_id, so we'll log and return error
        let request: SqsNestingRequest = match serde_json::from_str(body) {
            Ok(req) => req,
            Err(e) => {
                let error_msg = format!(
                    "Failed to parse request message: {}. Body (first 200 chars): {}",
                    e,
                    body.chars().take(200).collect::<String>()
                );
                error!("{}", error_msg);
                // Try to extract correlation_id from body if possible
                if let Ok(partial) = serde_json::from_str::<serde_json::Value>(body) {
                    if let Some(corr_id) = partial.get("correlationId").and_then(|v| v.as_str()) {
                        let output_queue_url = partial
                            .get("outputQueueUrl")
                            .and_then(|v| v.as_str())
                            .map(|s| s.to_string())
                            .unwrap_or_else(|| self.output_queue_url.clone());

                        let error_response = SqsNestingResponse {
                            correlation_id: corr_id.to_string(),
                            first_page_svg_url: None,
                            last_page_svg_url: None,
                            parts_placed: 0,
                            is_improvement: false,
                            is_final: true,
                            timestamp: current_timestamp(),
                            error_message: Some(error_msg.clone()),
                        };

                        if let Err(send_err) = self
                            .send_to_output_queue(&output_queue_url, &error_response)
                            .await
                        {
                            error!("Failed to send error response: {}", send_err);
                        }
                    }
                }
                return Err(anyhow::anyhow!(error_msg));
            }
        };

        // Calculate SVG size info for logging
        let svg_size_info = if let Some(ref svg_url) = request.svg_url {
            format!("S3 URL: {}", svg_url)
        } else if let Some(ref svg_b64) = request.svg_base64 {
            let base64_len = svg_b64.len();
            // Try to decode to get exact size, fall back to approximation if decoding fails
            match general_purpose::STANDARD.decode(svg_b64) {
                Ok(decoded) => format!("{} bytes (base64: {} bytes)", decoded.len(), base64_len),
                Err(_) => {
                    // Base64 encoding increases size by ~33%, so approximate decoded size
                    let approx_decoded_size = (base64_len * 3) / 4;
                    format!("~{} bytes (base64: {} bytes, decode failed)", approx_decoded_size, base64_len)
                }
            }
        } else {
            "N/A".to_string()
        };

        info!(
            "Processing request: correlation_id={}, bin_width={:?}, bin_height={:?}, spacing={:?}, amount_of_parts={:?}, amount_of_rotations={}, cancelled={}, svg_size={}, output_queue_url={:?}",
            request.correlation_id,
            request.bin_width,
            request.bin_height,
            request.spacing,
            request.amount_of_parts,
            request.amount_of_rotations,
            request.cancelled,
            svg_size_info,
            request.output_queue_url.as_ref().map(|s| s.as_str()).unwrap_or("default")
        );

        // Handle cancellation requests
        if request.cancelled {
            let was_running = self.mark_cancelled(&request.correlation_id);
            if was_running {
                info!(
                    "Cancellation request received and forwarded to running optimizer: correlation_id={}",
                    request.correlation_id
                );
            } else {
                info!(
                    "Cancellation request received for idle correlation_id={}, future runs will be skipped",
                    request.correlation_id
                );
            }
            return Ok(());
        }

        // Determine output queue (use request override if provided)
        let output_queue_url = request
            .output_queue_url
            .clone()
            .unwrap_or_else(|| self.output_queue_url.clone());

        // Validate required fields for non-cancellation requests
        // Either svg_base64 or svg_url must be provided
        if request.svg_base64.is_none() && request.svg_url.is_none() {
            let error_msg = "Missing required field: either svg_base64 or svg_url must be provided";
            error!("{}", error_msg);
            let error_response = SqsNestingResponse {
                correlation_id: request.correlation_id.clone(),
                first_page_svg_url: None,
                last_page_svg_url: None,
                parts_placed: 0,
                is_improvement: false,
                is_final: true,
                timestamp: current_timestamp(),
                error_message: Some(error_msg.to_string()),
            };
            if let Err(send_err) = self
                .send_to_output_queue(&output_queue_url, &error_response)
                .await
            {
                error!("Failed to send error response: {}", send_err);
            }
            return Ok(());
        }
        if request.bin_width.is_none() {
            let error_msg = "Missing required field: bin_width";
            error!("{}", error_msg);
            let error_response = SqsNestingResponse {
                correlation_id: request.correlation_id.clone(),
                first_page_svg_url: None,
                last_page_svg_url: None,
                parts_placed: 0,
                is_improvement: false,
                is_final: true,
                timestamp: current_timestamp(),
                error_message: Some(error_msg.to_string()),
            };
            if let Err(send_err) = self
                .send_to_output_queue(&output_queue_url, &error_response)
                .await
            {
                error!("Failed to send error response: {}", send_err);
            }
            return Ok(());
        }
        if request.bin_height.is_none() {
            let error_msg = "Missing required field: bin_height";
            error!("{}", error_msg);
            let error_response = SqsNestingResponse {
                correlation_id: request.correlation_id.clone(),
                first_page_svg_url: None,
                last_page_svg_url: None,
                parts_placed: 0,
                is_improvement: false,
                is_final: true,
                timestamp: current_timestamp(),
                error_message: Some(error_msg.to_string()),
            };
            if let Err(send_err) = self
                .send_to_output_queue(&output_queue_url, &error_response)
                .await
            {
                error!("Failed to send error response: {}", send_err);
            }
            return Ok(());
        }
        if request.spacing.is_none() {
            let error_msg = "Missing required field: spacing";
            error!("{}", error_msg);
            let error_response = SqsNestingResponse {
                correlation_id: request.correlation_id.clone(),
                first_page_svg_url: None,
                last_page_svg_url: None,
                parts_placed: 0,
                is_improvement: false,
                is_final: true,
                timestamp: current_timestamp(),
                error_message: Some(error_msg.to_string()),
            };
            if let Err(send_err) = self
                .send_to_output_queue(&output_queue_url, &error_response)
                .await
            {
                error!("Failed to send error response: {}", send_err);
            }
            return Ok(());
        }
        if request.amount_of_parts.is_none() {
            let error_msg = "Missing required field: amount_of_parts";
            error!("{}", error_msg);
            let error_response = SqsNestingResponse {
                correlation_id: request.correlation_id.clone(),
                first_page_svg_url: None,
                last_page_svg_url: None,
                parts_placed: 0,
                is_improvement: false,
                is_final: true,
                timestamp: current_timestamp(),
                error_message: Some(error_msg.to_string()),
            };
            if let Err(send_err) = self
                .send_to_output_queue(&output_queue_url, &error_response)
                .await
            {
                error!("Failed to send error response: {}", send_err);
            }
            return Ok(());
        }

        // Process the request and handle errors by sending error response
        let result = self
            .process_nesting_request(&request, &output_queue_url)
            .await;

        if let Err(e) = &result {
            let error_msg = format!("{}", e);
            error!("Failed to process message: {}", error_msg);

            // Send error response for internal processing errors
            let error_response = SqsNestingResponse {
                correlation_id: request.correlation_id.clone(),
                first_page_svg_url: None,
                last_page_svg_url: None,
                parts_placed: 0,
                is_improvement: false,
                is_final: true,
                timestamp: current_timestamp(),
                error_message: Some(error_msg),
            };

            if let Err(send_err) = self
                .send_to_output_queue(&output_queue_url, &error_response)
                .await
            {
                error!("Failed to send error response: {}", send_err);
            } else {
                info!(
                    "Sent error response to queue for correlation_id={}",
                    request.correlation_id
                );
            }
        }

        // Always return Ok so message gets acknowledged
        Ok(())
    }

    /// Internal method to process nesting request
    async fn process_nesting_request(
        &self,
        request: &SqsNestingRequest,
        output_queue_url: &str,
    ) -> Result<()> {
        // Register correlation_id in cancellation registry
        {
            let mut registry = self.cancellation_registry.lock().unwrap();
            registry.insert(request.correlation_id.clone(), false);
        }

        // Ensure cleanup happens even on error
        let result = {
            // Unwrap required fields (validation already done in process_message)
            let bin_width = request.bin_width.unwrap();
            let bin_height = request.bin_height.unwrap();
            let spacing = request.spacing.unwrap();
            let amount_of_parts = request.amount_of_parts.unwrap();

            // Get SVG bytes - either from S3 or from base64
            let decode_start = std::time::Instant::now();
            let svg_bytes = if let Some(ref svg_url) = request.svg_url {
                // Download from S3
                info!("Downloading SVG from S3: {}", svg_url);
                self.download_svg_from_s3(svg_url).await?
            } else if let Some(ref svg_base64) = request.svg_base64 {
                // Decode from base64
                decode_svg(svg_base64)?
            } else {
                return Err(anyhow!("Neither svg_url nor svg_base64 provided"));
            };
            info!("SVG payload ready: {} bytes (took {:?})", svg_bytes.len(), decode_start.elapsed());

            // Create cancellation checker closure
            let cancellation_registry = self.cancellation_registry.clone();
            let correlation_id_clone = request.correlation_id.clone();
            let cancellation_check_count = Arc::new(AtomicU64::new(0));
            let cancellation_check_count_for_log = cancellation_check_count.clone();
            let cancellation_checker = move || {
                let count = cancellation_check_count_for_log.fetch_add(1, Ordering::Relaxed) + 1;
                if count % 1000 == 0 {
                    log::debug!("Cancellation checker called {} times", count);
                }
                let registry = cancellation_registry.lock().unwrap();
                registry.get(&correlation_id_clone).copied().unwrap_or(false)
            };

            // Create channel for sending improvement results from sync callback to async task
            let (tx, mut rx) = mpsc::unbounded_channel::<jagua_utils::svg_nesting::NestingResult>();
            
            // Spawn async task to handle improvement messages
            info!("Spawning async task to handle improvement messages");
            let sqs_client_for_task = self.sqs_client.clone();
            let s3_client_for_task = self.s3_client.clone();
            let s3_bucket_for_task = self.s3_bucket.clone();
            let output_queue_url_for_task = output_queue_url.to_string();
            let correlation_id_for_task = request.correlation_id.clone();
            let bin_width_for_task = bin_width;
            let bin_height_for_task = bin_height;
            let _improvement_task_handle = tokio::spawn(async move {
                info!("Improvement task started, waiting for messages...");
                while let Some(result) = rx.recv().await {
                    info!("Improvement task received message: {} parts placed, {} pages", result.parts_placed, result.page_svgs.len());
                    
                    // Get the first and last page SVGs for uploading to S3
                    let first_page_bytes = result.page_svgs.first()
                        .unwrap_or_else(|| &result.combined_svg);
                    let last_page_bytes = result.page_svgs.last()
                        .unwrap_or_else(|| &result.combined_svg);
                    
                    // Upload first page SVG to S3
                    let first_page_svg_url = match upload_svg_to_s3_internal(
                        &s3_client_for_task,
                        &s3_bucket_for_task,
                        first_page_bytes,
                        &correlation_id_for_task,
                        "first-page.svg",
                    ).await {
                        Ok(url) => {
                            info!("Uploaded improvement first page SVG to S3: {}", url);
                            Some(url)
                        }
                        Err(e) => {
                            error!("Failed to upload improvement first page SVG to S3: {}", e);
                            None
                        }
                    };
                    
                    // Upload last page SVG to S3
                    let last_page_svg_url = match upload_svg_to_s3_internal(
                        &s3_client_for_task,
                        &s3_bucket_for_task,
                        last_page_bytes,
                        &correlation_id_for_task,
                        "last-page.svg",
                    ).await {
                        Ok(url) => {
                            info!("Uploaded improvement last page SVG to S3: {}", url);
                            Some(url)
                        }
                        Err(e) => {
                            error!("Failed to upload improvement last page SVG to S3: {}", e);
                            None
                        }
                    };

                    // Create improvement response with S3 URLs
                    let response = SqsNestingResponse {
                        correlation_id: correlation_id_for_task.clone(),
                        first_page_svg_url,
                        last_page_svg_url,
                        parts_placed: result.parts_placed,
                        is_improvement: true,
                        is_final: false,
                        timestamp: current_timestamp(),
                        error_message: None,
                    };

                    info!(
                        "Sending improvement response: {} parts placed",
                        response.parts_placed
                    );

                    // Send to SQS using the same method as error responses
                    info!("Attempting to send improvement to queue: {}", output_queue_url_for_task);
                    match Self::send_message_to_sqs(&sqs_client_for_task, &output_queue_url_for_task, &response).await {
                        Ok(()) => {
                            info!("Successfully sent improvement response to queue");
                        }
                        Err(e) => {
                            error!("Failed to send improvement to queue: {}", e);
                            // Log the error chain for debugging
                            let mut error_chain = format!("{}", e);
                            let mut source = e.source();
                            while let Some(err) = source {
                                error_chain.push_str(&format!(": {}", err));
                                source = err.source();
                            }
                            error!("Full error chain: {}", error_chain);
                        }
                    }
                }
                info!("Improvement task finished (channel closed)");
            });

            // Create improvement callback that sends to channel
            info!("Creating improvement callback");
            let tx_for_callback = tx.clone();
            let improvement_callback: Option<jagua_utils::svg_nesting::ImprovementCallback> = 
                Some(Box::new(move |result: jagua_utils::svg_nesting::NestingResult| -> Result<()> {
                    info!("Improvement callback called from blocking thread: {} parts placed, {} pages", result.parts_placed, result.page_svgs.len());
                    // Send result to channel (non-blocking for unbounded channel)
                    match tx_for_callback.send(result) {
                        Ok(()) => {
                            info!("Improvement result sent to channel successfully from blocking thread");
                            Ok(())
                        }
                        Err(e) => {
                            error!("Failed to send improvement result to channel: {}", e);
                            Err(anyhow!("Failed to send improvement result to channel: {}", e))
                        }
                    }
                }));

            // Use adaptive nesting strategy with cancellation checker
            info!("Creating AdaptiveNestingStrategy with cancellation checker");
            let strategy_start = std::time::Instant::now();
            let strategy = AdaptiveNestingStrategy::with_cancellation_checker(Box::new(cancellation_checker));
            info!("Strategy created (took {:?})", strategy_start.elapsed());
            
            // Clone cancellation_check_count for logging after spawn_blocking
            let cancellation_check_count_for_final_log = cancellation_check_count.clone();
            
            // Run nest() in a blocking task to avoid blocking the async runtime
            // This allows the improvement task to process messages while optimization runs
            info!("Starting nesting optimization in spawn_blocking task");
            let nest_start = std::time::Instant::now();
            let svg_bytes_for_nest = svg_bytes.clone();
            let amount_of_rotations = request.amount_of_rotations;
            let correlation_id_for_error = request.correlation_id.clone();
            let nesting_result = tokio::task::spawn_blocking(move || {
                info!("Inside spawn_blocking: calling strategy.nest()");
                let nest_call_start = std::time::Instant::now();
                let result = strategy.nest(
                    bin_width,
                    bin_height,
                    spacing,
                    &svg_bytes_for_nest,
                    amount_of_parts,
                    amount_of_rotations,
                    improvement_callback,
                );
                info!("Inside spawn_blocking: strategy.nest() completed (took {:?})", nest_call_start.elapsed());
                result
            })
            .await
            .context("Failed to spawn blocking task for nesting")?;
            info!("spawn_blocking task completed (took {:?})", nest_start.elapsed());
            let nesting_result = nesting_result.with_context(|| {
                format!(
                    "Failed to process SVG nesting for correlation_id={}",
                    correlation_id_for_error
                )
            })?;
            info!("Nesting result obtained successfully");
            info!("Cancellation checker was called {} times total", cancellation_check_count_for_final_log.load(Ordering::Relaxed));

            // Drop the sender to signal the async task that no more improvements will come
            drop(tx);
            
            // Wait a bit for any pending improvement messages to be sent
            // This ensures all improvements are sent before we send the final result
            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

            info!(
                "Nesting complete: {} parts placed out of {} requested ({} page SVGs generated)",
                nesting_result.parts_placed,
                nesting_result.total_parts_requested,
                nesting_result.page_svgs.len()
            );

            // Prepare final response images
            // Use first page SVG for first sheet
            let first_page_bytes = nesting_result.page_svgs.first()
                .unwrap_or_else(|| &nesting_result.combined_svg);
            let last_page_bytes = nesting_result.page_svgs.last()
                .unwrap_or_else(|| &nesting_result.combined_svg);
            
            // Upload first page SVG to S3
            let first_page_svg_url = match self.upload_svg_to_s3(first_page_bytes, &request.correlation_id, "first-page.svg").await {
                Ok(url) => {
                    info!("Uploaded final result first page SVG to S3: {}", url);
                    Some(url)
                }
                Err(e) => {
                    error!("Failed to upload final result first page SVG to S3: {}", e);
                    None
                }
            };
            
            // Upload last page SVG to S3
            let last_page_svg_url = match self.upload_svg_to_s3(last_page_bytes, &request.correlation_id, "last-page.svg").await {
                Ok(url) => {
                    info!("Uploaded final result last page SVG to S3: {}", url);
                    Some(url)
                }
                Err(e) => {
                    error!("Failed to upload final result last page SVG to S3: {}", e);
                    None
                }
            };

            // Send final result to queue (with S3 URLs)
            let response = SqsNestingResponse {
                correlation_id: request.correlation_id.clone(),
                first_page_svg_url,
                last_page_svg_url,
                parts_placed: nesting_result.parts_placed,
                is_improvement: false,
                is_final: true,
                timestamp: current_timestamp(),
                error_message: None,
            };

            info!(
                "Sending final response with parts_placed: {} (from nesting_result.parts_placed: {})",
                response.parts_placed, nesting_result.parts_placed
            );

            self.send_to_output_queue(output_queue_url, &response)
                .await
                .context("Failed to send final result to queue")?;

            info!("Sent final result to queue");

            Ok(())
        };

        // Cleanup: remove correlation_id from cancellation registry (always happens)
        {
            let mut registry = self.cancellation_registry.lock().unwrap();
            registry.remove(&request.correlation_id);
        }

        result
    }

    /// Listen and process messages from the queue (concurrent processing)
    /// Processes up to 20 messages concurrently using tokio tasks with semaphore-based concurrency control.
    pub async fn listen_and_process(
        &self,
        _worker_count: usize, // Ignored, kept for compatibility
        mut shutdown_rx: broadcast::Receiver<()>,
    ) -> Result<()> {
        info!("Starting concurrent worker on queue: {} (max 20 concurrent tasks)", self.input_queue_url);

        // Create semaphore to limit concurrent processing to 20 tasks
        let semaphore = Arc::new(Semaphore::new(20));

        loop {
            tokio::select! {
                _ = shutdown_rx.recv() => {
                    info!("Received shutdown signal");
                    break;
                }
                result = self.sqs_client
                    .receive_message()
                    .queue_url(&self.input_queue_url)
                    .max_number_of_messages(10)
                    .wait_time_seconds(20)
                    .send() => {
                    let response = result.context("Failed to receive messages from queue")?;

                    if let Some(messages) = response.messages {
                        for message in messages {
                            // Check for shutdown before spawning
                            if shutdown_rx.try_recv().is_ok() {
                                info!("Stopping before processing message due to shutdown");
                                break;
                            }

                            let receipt_handle = match message.receipt_handle() {
                                Some(h) => h.to_string(),
                                None => {
                                    error!("Message missing receipt handle, skipping");
                                    continue;
                                }
                            };
                            let body = match message.body() {
                                Some(b) => b.to_string(),
                                None => {
                                    error!("Message missing body, skipping");
                                    continue;
                                }
                            };
                            let message_id = message.message_id().map(|s| s.to_string());

                            // Acknowledge (delete) message immediately after receiving to prevent duplicate processing
                            let sqs_client_for_delete = self.sqs_client.clone();
                            let input_queue_url_for_delete = self.input_queue_url.clone();
                            let receipt_handle_for_delete = receipt_handle.clone();
                            let message_id_for_delete = message_id.clone();
                            
                            if let Err(e) = sqs_client_for_delete
                                .delete_message()
                                .queue_url(&input_queue_url_for_delete)
                                .receipt_handle(&receipt_handle_for_delete)
                                .send()
                                .await
                            {
                                error!("Failed to acknowledge message: {}", e);
                                continue; // Skip processing if we can't acknowledge
                            }

                            if let Some(msg_id) = &message_id {
                                info!("Acknowledged message {}, processing concurrently", msg_id);
                            } else {
                                info!("Acknowledged message, processing concurrently");
                            }

                            // Clone necessary data for the spawned task
                            let processor = self.clone();
                            let semaphore_clone = semaphore.clone();
                            let mut shutdown_rx_clone = shutdown_rx.resubscribe();

                            // Spawn concurrent task for processing
                            tokio::spawn(async move {
                                // Acquire semaphore permit (waits if 20 tasks are already running)
                                let _permit = match semaphore_clone.acquire().await {
                                    Ok(permit) => permit,
                                    Err(e) => {
                                        error!("Failed to acquire semaphore permit: {}", e);
                                        return;
                                    }
                                };

                                // Process the message
                                let process_result = processor.process_message(&receipt_handle, &body).await;
                                if let Err(e) = &process_result {
                                    error!("Error during message processing: {}", e);
                                }
                                // Permit is automatically released when dropped here
                            });
                        }
                    }
                }
            }
        }

        info!("Worker exiting gracefully");
        Ok(())
    }
}

#[cfg(test)]
impl SqsProcessor {
    pub(crate) fn cancellation_registry_handle(&self) -> Arc<Mutex<HashMap<String, bool>>> {
        self.cancellation_registry.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use aws_config::BehaviorVersion;
    use aws_sdk_sqs::Client as SqsClient;
    use std::collections::HashMap;
    use std::sync::{Arc, Mutex};
    use tokio::sync::broadcast;
    use tokio::time::{Duration, Instant};

    #[test]
    fn test_cancellation_registry_insert_and_get() {
        let registry: Arc<Mutex<HashMap<String, bool>>> = Arc::new(Mutex::new(HashMap::new()));

        // Insert a cancellation flag
        {
            let mut reg = registry.lock().unwrap();
            reg.insert("test-id-1".to_string(), true);
        }

        // Check that it's set
        {
            let reg = registry.lock().unwrap();
            assert_eq!(reg.get("test-id-1"), Some(&true));
            assert_eq!(reg.get("test-id-2"), None);
        }
    }

    #[test]
    fn test_cancellation_registry_remove() {
        let registry: Arc<Mutex<HashMap<String, bool>>> = Arc::new(Mutex::new(HashMap::new()));

        // Insert and then remove
        {
            let mut reg = registry.lock().unwrap();
            reg.insert("test-id-1".to_string(), false);
        }

        {
            let mut reg = registry.lock().unwrap();
            reg.remove("test-id-1");
        }

        // Verify it's gone
        {
            let reg = registry.lock().unwrap();
            assert_eq!(reg.get("test-id-1"), None);
        }
    }

    #[test]
    fn test_sqs_nesting_request_cancelled_field_default() {
        let request_json = r#"{
            "correlationId": "test-123",
            "svgBase64": "dGVzdA==",
            "binWidth": 100.0,
            "binHeight": 100.0,
            "spacing": 10.0,
            "amountOfParts": 1
        }"#;

        let request: SqsNestingRequest = serde_json::from_str(request_json).unwrap();
        assert_eq!(
            request.cancelled, false,
            "cancelled should default to false"
        );
    }

    #[test]
    fn test_sqs_nesting_request_cancelled_field_explicit() {
        let request_json = r#"{
            "correlationId": "test-123",
            "svgBase64": "dGVzdA==",
            "binWidth": 100.0,
            "binHeight": 100.0,
            "spacing": 10.0,
            "amountOfParts": 1,
            "cancelled": true
        }"#;

        let request: SqsNestingRequest = serde_json::from_str(request_json).unwrap();
        assert_eq!(request.cancelled, true, "cancelled should be true when set");
    }

    #[tokio::test]
    async fn test_parallel_cancellation_flag_shared_between_workers() {
        let config = aws_config::defaults(BehaviorVersion::latest()).load().await;
        let sqs_client = SqsClient::new(&config);
        let s3_client = aws_sdk_s3::Client::new(&config);
        let processor = SqsProcessor::new(
            sqs_client,
            s3_client,
            "test-bucket".to_string(),
            "test-input-queue".to_string(),
            "test-output-queue".to_string(),
        );

        let correlation_id = "parallel-cancelled".to_string();
        let registry = processor.cancellation_registry_handle();
        {
            let mut reg = registry.lock().unwrap();
            reg.insert(correlation_id.clone(), false);
        }

        let cancel_processor = processor.clone();
        let cancellation_request = SqsNestingRequest {
            svg_url: None,
            correlation_id: correlation_id.clone(),
            svg_base64: None,
            bin_width: None,
            bin_height: None,
            spacing: None,
            amount_of_parts: None,
            amount_of_rotations: 8,
            output_queue_url: None,
            cancelled: true,
        };
        let cancellation_body =
            serde_json::to_string(&cancellation_request).expect("serialize cancellation");

        let registry_clone = registry.clone();
        let correlation_id_clone = correlation_id.clone();
        let watcher = tokio::spawn(async move {
            let timeout = Duration::from_secs(2);
            let start = Instant::now();
            loop {
                {
                    let reg = registry_clone.lock().unwrap();
                    if reg.get(&correlation_id_clone).copied().unwrap_or(false) {
                        break;
                    }
                }

                if start.elapsed() > timeout {
                    panic!("Timed out waiting for cancellation flag to be set");
                }

                tokio::time::sleep(Duration::from_millis(20)).await;
            }
        });

        let canceller = tokio::spawn(async move {
            cancel_processor
                .process_message("receipt-handle", &cancellation_body)
                .await
                .expect("Cancellation request should be processed");
        });

        watcher.await.expect("Watcher task failed");
        canceller.await.expect("Canceller task failed");

        let reg = registry.lock().unwrap();
        assert_eq!(
            reg.get(&correlation_id),
            Some(&true),
            "Cancellation flag should be set to true"
        );
    }

    #[tokio::test]
    async fn test_s3_download() {
        use std::env;
        use aws_config::BehaviorVersion;
        use aws_sdk_s3::Client as S3Client;
        use aws_sdk_s3::error::ProvideErrorMetadata;

        // Initialize logger for test output
        let _ = env_logger::Builder::from_default_env()
            .filter_level(log::LevelFilter::Debug)
            .try_init();

        // Get configuration from environment variables
        let bucket = env::var("S3_BUCKET").unwrap_or_else(|_| "cutl-staging-uploads".to_string());
        let test_key = "22db4d1f-44cb-4c3d-917d-17836ba986ac/projectParts/9720e425-6a18-4a46-aa4c-7a7934ae9f23/project_part_internal_svg.svg";
        
        println!("Testing S3 download:");
        println!("  Bucket: {}", bucket);
        println!("  Key: {}", test_key);
        println!("  AWS_REGION: {:?}", env::var("AWS_REGION"));
        println!("  AWS_ENDPOINT_URL: {:?}", env::var("AWS_ENDPOINT_URL"));
        println!("  AWS_ACCESS_KEY_ID: {:?}", env::var("AWS_ACCESS_KEY_ID").map(|s| format!("{}...", &s[..10.min(s.len())])));

        // Initialize AWS config
        let mut config_loader = aws_config::defaults(BehaviorVersion::latest());
        
        // Configure LocalStack endpoint if provided
        if let Ok(endpoint_url) = env::var("AWS_ENDPOINT_URL") {
            config_loader = config_loader.endpoint_url(&endpoint_url);
            println!("Using AWS endpoint: {}", endpoint_url);
        }

        let config = config_loader.load().await;
        let s3_client = S3Client::new(&config);

        // Test 1: Try to download the file
        println!("\nTest 1: Downloading file from S3...");
        let result = s3_client
            .get_object()
            .bucket(&bucket)
            .key(test_key)
            .send()
            .await;

        match result {
            Ok(response) => {
                println!("✓ Successfully got object from S3");
                
                // Try to read the body
                let mut body_stream = response.body;
                use futures::StreamExt;
                let mut svg_bytes = Vec::new();
                while let Some(chunk_result) = body_stream.next().await {
                    match chunk_result {
                        Ok(chunk) => {
                            svg_bytes.extend_from_slice(&chunk);
                        }
                        Err(e) => {
                            println!("✗ Error reading chunk: {}", e);
                            return;
                        }
                    }
                }
                println!("✓ Successfully downloaded {} bytes", svg_bytes.len());
                
                // Try to parse as SVG
                let svg_content = String::from_utf8_lossy(&svg_bytes);
                if svg_content.contains("<svg") {
                    println!("✓ Content appears to be valid SVG");
                } else {
                    println!("⚠ Content doesn't appear to be SVG (first 100 chars: {})", 
                        svg_content.chars().take(100).collect::<String>());
                }
            }
            Err(e) => {
                println!("✗ Failed to download from S3: {}", e);
                println!("Error details:");
                
                // Try to get more error information
                if let Some(code) = e.code() {
                    println!("  Error code: {:?}", code);
                }
                if let Some(message) = e.message() {
                    println!("  Error message: {:?}", message);
                }
                
                // Test 2: Try to list objects in the bucket to verify connectivity
                println!("\nTest 2: Testing bucket connectivity by listing objects...");
                let list_result = s3_client
                    .list_objects_v2()
                    .bucket(&bucket)
                    .max_keys(5)
                    .send()
                    .await;
                
                match list_result {
                    Ok(list_response) => {
                        println!("✓ Successfully connected to bucket");
                        let contents = list_response.contents();
                        if !contents.is_empty() {
                            println!("  Found {} objects (showing first 5)", contents.len());
                            for (i, obj) in contents.iter().take(5).enumerate() {
                                println!("    {}. {}", i + 1, obj.key().map(|k| k.to_string()).unwrap_or_else(|| "(no key)".to_string()));
                            }
                        } else {
                            println!("  Bucket is empty");
                        }
                    }
                    Err(e) => {
                        println!("✗ Failed to list objects: {}", e);
                        println!("  This suggests a connectivity or permissions issue");
                    }
                }
                
                // Test 3: Try to check if bucket exists
                println!("\nTest 3: Checking if bucket exists...");
                let head_result = s3_client
                    .head_bucket()
                    .bucket(&bucket)
                    .send()
                    .await;
                
                match head_result {
                    Ok(_) => {
                        println!("✓ Bucket exists and is accessible");
                    }
                    Err(e) => {
                        println!("✗ Bucket check failed: {}", e);
                        if let Some(code) = e.code() {
                            println!("  Error code: {:?}", code);
                        }
                    }
                }
            }
        }
    }
}
