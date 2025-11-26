use anyhow::{anyhow, Context, Result};
use aws_sdk_sqs::Client as SqsClient;
use base64::{engine::general_purpose, Engine as _};
use jagua_utils::svg_nesting::{NestingStrategy, SimpleNestingStrategy};
use log::{debug, error, info};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::sync::broadcast;

/// Request message structure for SQS queue
/// For cancellation requests, only `correlation_id` and `cancelled: true` are required.
/// All other fields are required only when `cancelled` is false or not present.
#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SqsNestingRequest {
    /// Unique identifier for tracking the request
    pub correlation_id: String,
    /// Base64-encoded SVG payload (required if not cancelled)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub svg_base64: Option<String>,
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

fn sanitize_svg_fields(response: &SqsNestingResponse) -> SqsNestingResponse {
    let mut sanitized = response.clone();
    sanitized.first_page_svg_base64 =
        format!("<{} bytes stripped>", response.first_page_svg_base64.len());
    sanitized.last_page_svg_base64 = response
        .last_page_svg_base64
        .as_ref()
        .map(|svg| format!("<{} bytes stripped>", svg.len()));
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
    /// Base64-encoded SVG for the first page
    pub first_page_svg_base64: String,
    /// Base64-encoded SVG for the last page (same as first when single page)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_page_svg_base64: Option<String>,
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
    pub fn new(sqs_client: SqsClient, input_queue_url: String, output_queue_url: String) -> Self {
        Self {
            sqs_client,
            input_queue_url,
            output_queue_url,
            cancellation_registry: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Send message to output queue
    pub async fn send_to_output_queue(
        &self,
        queue_url: &str,
        response: &SqsNestingResponse,
    ) -> Result<()> {
        let message_body =
            serde_json::to_string(response).context("Failed to serialize response")?;

        debug!(
            "Sending message to output queue: correlation_id={}, is_final={}",
            response.correlation_id, response.is_final
        );

        let sanitized_response = sanitize_svg_fields(response);
        let sanitized_body = serde_json::to_string(&sanitized_response)
            .unwrap_or_else(|_| "<failed to serialize sanitized response>".to_string());

        self.sqs_client
            .send_message()
            .queue_url(queue_url)
            .message_body(&message_body)
            .send()
            .await
            .context("Failed to send message to output queue")?;

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
                            first_page_svg_base64: String::new(),
                            last_page_svg_base64: None,
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

        info!(
            "Processing request: correlation_id={}",
            request.correlation_id
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

        // Validate required fields for non-cancellation requests
        if request.svg_base64.is_none() {
            return Err(anyhow!("Missing required field: svg_base64"));
        }
        if request.bin_width.is_none() {
            return Err(anyhow!("Missing required field: bin_width"));
        }
        if request.bin_height.is_none() {
            return Err(anyhow!("Missing required field: bin_height"));
        }
        if request.spacing.is_none() {
            return Err(anyhow!("Missing required field: spacing"));
        }
        if request.amount_of_parts.is_none() {
            return Err(anyhow!("Missing required field: amount_of_parts"));
        }

        // Determine output queue (use request override if provided)
        let output_queue_url = request
            .output_queue_url
            .clone()
            .unwrap_or_else(|| self.output_queue_url.clone());

        // Process the request and handle errors by sending error response
        let result = self
            .process_nesting_request(&request, &output_queue_url)
            .await;

        if let Err(e) = &result {
            let error_msg = format!("{}", e);
            error!("Failed to process message: {}", error_msg);

            // Send error response
            let error_response = SqsNestingResponse {
                correlation_id: request.correlation_id.clone(),
                first_page_svg_base64: String::new(),
                last_page_svg_base64: None,
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
            let svg_base64 = request.svg_base64.as_ref().unwrap();
            let bin_width = request.bin_width.unwrap();
            let bin_height = request.bin_height.unwrap();
            let spacing = request.spacing.unwrap();
            let amount_of_parts = request.amount_of_parts.unwrap();

            // Decode SVG payload
            let svg_bytes = decode_svg(svg_base64)?;
            info!("Decoded SVG payload: {} bytes", svg_bytes.len());

            // Use simple nesting strategy
            let strategy = SimpleNestingStrategy::new();
            let nesting_result = strategy
                .nest(
                    bin_width,
                    bin_height,
                    spacing,
                    &svg_bytes,
                    amount_of_parts,
                    request.amount_of_rotations,
                )
                .with_context(|| {
                    format!(
                        "Failed to process SVG nesting for correlation_id={}",
                        request.correlation_id
                    )
                })?;

            info!(
                "Nesting complete: {} parts placed out of {} requested ({} page SVGs generated)",
                nesting_result.parts_placed,
                nesting_result.total_parts_requested,
                nesting_result.page_svgs.len()
            );

            // Prepare final response images
            // Use first page SVG for first sheet (same logic as last sheet)
            let first_page_bytes = nesting_result.page_svgs.first()
                .unwrap_or_else(|| &nesting_result.combined_svg);
            
            // Use unplaced parts SVG for last page if available, otherwise use last filled page
            let last_page_bytes = nesting_result
                .unplaced_parts_svg
                .as_ref()
                .unwrap_or_else(|| nesting_result.page_svgs.last().unwrap_or(first_page_bytes));
            
            let first_page_svg_base64 = encode_svg(first_page_bytes);
            let last_page_svg_base64 = encode_svg(last_page_bytes);

            // Send final result to queue
            let response = SqsNestingResponse {
                correlation_id: request.correlation_id.clone(),
                first_page_svg_base64,
                last_page_svg_base64: Some(last_page_svg_base64),
                parts_placed: nesting_result.parts_placed,
                is_improvement: false,
                is_final: true,
                timestamp: current_timestamp(),
                error_message: None,
            };

            info!(
                "Sending response with parts_placed: {} (from nesting_result.parts_placed: {})",
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

    /// Listen and process messages from the queue (single-threaded)
    pub async fn listen_and_process(
        &self,
        _worker_count: usize, // Ignored, kept for compatibility
        mut shutdown_rx: broadcast::Receiver<()>,
    ) -> Result<()> {
        info!("Starting single worker on queue: {}", self.input_queue_url);

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
                            // Check for shutdown before processing
                            if shutdown_rx.try_recv().is_ok() {
                                info!("Stopping before processing message due to shutdown");
                                break;
                            }

                            let receipt_handle = message
                                .receipt_handle()
                                .ok_or_else(|| anyhow::anyhow!("Message missing receipt handle"))?;
                            let body = message
                                .body()
                                .ok_or_else(|| anyhow::anyhow!("Message missing body"))?;

                            if let Some(message_id) = message.message_id() {
                                info!("Received message {}", message_id);
                            } else {
                                info!("Received message without message_id");
                            }

                            let process_result = self.process_message(receipt_handle, body).await;
                            if let Err(e) = &process_result {
                                error!("Error during message processing: {}", e);
                            }

                            // Check for shutdown before deleting
                            if shutdown_rx.try_recv().is_ok() {
                                info!("Stopping before deleting message due to shutdown");
                                break;
                            }

                            if let Err(e) = self.sqs_client
                                .delete_message()
                                .queue_url(&self.input_queue_url)
                                .receipt_handle(receipt_handle)
                                .send()
                                .await
                            {
                                error!("Failed to delete message: {}", e);
                            } else {
                                info!("Acknowledged message");
                            }
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
        let processor = SqsProcessor::new(
            sqs_client,
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
}
