use anyhow::{anyhow, Context, Result};
use aws_sdk_sqs::Client as SqsClient;
use base64::{engine::general_purpose, Engine as _};
use jagua_utils::svg_nesting::{nest_svg_parts_adaptive, AdaptiveConfig};
use log::{debug, error, info};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::sync::mpsc;

/// Request message structure for SQS queue
#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SqsNestingRequest {
    /// Unique identifier for tracking the request
    pub correlation_id: String,
    /// Base64-encoded SVG payload
    pub svg_base64: String,
    /// Bin width for nesting
    pub bin_width: f32,
    /// Bin height for nesting
    pub bin_height: f32,
    /// Spacing between parts
    pub spacing: f32,
    /// Number of parts to nest
    pub amount_of_parts: usize,
    /// Number of rotations to try (default: 8)
    #[serde(default = "default_rotations")]
    pub amount_of_rotations: usize,
    /// Adaptive configuration as JSON string (uses defaults if not provided)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub config: Option<String>,
    /// Output queue URL for results (falls back to default if omitted)
    pub output_queue_url: Option<String>,
}

fn encode_svg(bytes: &[u8]) -> String {
    general_purpose::STANDARD.encode(bytes)
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
    /// Optional base64-encoded SVG for the last page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_page_svg_base64: Option<String>,
    /// Number of parts placed
    pub parts_placed: usize,
    /// Whether this is an intermediate improvement
    pub is_improvement: bool,
    /// Whether this is the final result
    pub is_final: bool,
    /// Timestamp in seconds since epoch
    pub timestamp: u64,
    /// Error message if processing failed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
}

/// Shared state for callback handler
struct CallbackState {
    best_parts_placed: usize,
    best_svg_bytes: Vec<u8>,
}

/// Message for async processing from callback
struct ImprovementMessage {
    correlation_id: String,
    svg_bytes: Vec<u8>,
    parts_placed: usize,
}

/// SQS Processor for handling SVG nesting requests
#[derive(Clone)]
pub struct SqsProcessor {
    sqs_client: SqsClient,
    input_queue_url: String,
    output_queue_url: String,
}

impl SqsProcessor {
    /// Create a new SQS processor
    pub fn new(sqs_client: SqsClient, input_queue_url: String, output_queue_url: String) -> Self {
        Self {
            sqs_client,
            input_queue_url,
            output_queue_url,
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

        self.sqs_client
            .send_message()
            .queue_url(queue_url)
            .message_body(&message_body)
            .send()
            .await
            .context("Failed to send message to output queue")?;

        info!(
            "Emitted response to {}: correlation_id={}, is_final={}",
            queue_url, response.correlation_id, response.is_final
        );

        Ok(())
    }

    /// Create callback handler for nest_svg_parts_adaptive
    fn create_callback_handler(
        &self,
        correlation_id: String,
        sender: mpsc::UnboundedSender<ImprovementMessage>,
    ) -> impl Fn(&[u8], usize) -> bool {
        let state = Arc::new(Mutex::new(CallbackState {
            best_parts_placed: 0,
            best_svg_bytes: Vec::new(),
        }));

        move |svg_bytes: &[u8], parts_placed: usize| {
            let mut state = state.lock().unwrap();

            // Only process if this is an improvement
            if parts_placed > state.best_parts_placed {
                state.best_parts_placed = parts_placed;
                state.best_svg_bytes = svg_bytes.to_vec();

                // Send to channel for async processing
                let msg = ImprovementMessage {
                    correlation_id: correlation_id.clone(),
                    svg_bytes: svg_bytes.to_vec(),
                    parts_placed,
                };

                if let Err(e) = sender.send(msg) {
                    error!("Failed to send improvement message to channel: {}", e);
                }
            }

            false // Don't cancel
        }
    }

    /// Process improvement messages from callback
    async fn process_improvements(
        &self,
        mut receiver: mpsc::UnboundedReceiver<ImprovementMessage>,
        output_queue_url: String,
    ) {
        while let Some(msg) = receiver.recv().await {
            let response = SqsNestingResponse {
                correlation_id: msg.correlation_id.clone(),
                first_page_svg_base64: encode_svg(&msg.svg_bytes),
                last_page_svg_base64: None,
                parts_placed: msg.parts_placed,
                is_improvement: true,
                is_final: false,
                timestamp: current_timestamp(),
                error_message: None,
            };

            if let Err(e) = self
                .sqs_client
                .send_message()
                .queue_url(&output_queue_url)
                .message_body(
                    &serde_json::to_string(&response).expect("SqsNestingResponse should serialize"),
                )
                .send()
                .await
            {
                error!("Failed to send improvement message to queue: {}", e);
            } else {
                info!(
                    "Sent improvement message: {} parts placed",
                    msg.parts_placed
                );
            }
        }
    }

    /// Process a single message from the queue
    /// Returns Ok(()) on success, or sends error response and returns Ok(()) on error
    /// (message should always be acknowledged after calling this)
    pub async fn process_message(&self, _receipt_handle: &str, body: &str) -> Result<()> {
        // Parse request - if this fails, we can't get correlation_id, so we'll log and return error
        let request: SqsNestingRequest = match serde_json::from_str(body) {
            Ok(req) => req,
            Err(e) => {
                let error_msg = format!("Failed to parse request message: {}. Body (first 200 chars): {}", 
                    e, body.chars().take(200).collect::<String>());
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
                        
                        if let Err(send_err) = self.send_to_output_queue(&output_queue_url, &error_response).await {
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

        // Determine output queue (use request override if provided)
        let output_queue_url = request
            .output_queue_url
            .clone()
            .unwrap_or_else(|| self.output_queue_url.clone());

        // Process the request and handle errors by sending error response
        let result = self.process_nesting_request(&request, &output_queue_url).await;
        
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
            
            if let Err(send_err) = self.send_to_output_queue(&output_queue_url, &error_response).await {
                error!("Failed to send error response: {}", send_err);
            } else {
                info!("Sent error response to queue for correlation_id={}", request.correlation_id);
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
        // Decode SVG payload
        let svg_bytes = decode_svg(&request.svg_base64)?;
        info!("Decoded SVG payload: {} bytes", svg_bytes.len());

        // Get configuration - for now, just use defaults
        let config = AdaptiveConfig::default();

        // Create channel for improvement messages
        let (tx, rx) = mpsc::unbounded_channel();

        // Spawn task to process improvements
        let processor_clone = self.clone();
        let output_queue_url_clone = output_queue_url.to_string();
        let improvement_task = tokio::spawn(async move {
            processor_clone
                .process_improvements(rx, output_queue_url_clone)
                .await;
        });

        // Create callback handler (tx will be moved into the closure)
        let callback = self.create_callback_handler(request.correlation_id.clone(), tx);

        // Process nesting
        let nesting_result = nest_svg_parts_adaptive(
            request.bin_width,
            request.bin_height,
            request.spacing,
            &svg_bytes,
            request.amount_of_parts,
            request.amount_of_rotations,
            config,
            Some(callback),
        )
        .with_context(|| format!("Failed to process SVG nesting for correlation_id={}", request.correlation_id))?;

        // Wait a bit for any remaining improvement messages to be processed
        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
        improvement_task.abort();

        info!(
            "Nesting complete: {} parts placed",
            nesting_result.parts_placed
        );

        // Prepare final response images
        let first_page_svg_base64 = nesting_result
            .page_svgs
            .first()
            .map(|page| encode_svg(page))
            .unwrap_or_else(|| encode_svg(&nesting_result.combined_svg));

        // Only set last_page if there are multiple pages (more than 1 page)
        let last_page_svg_base64 = if nesting_result.parts_placed > 0 && nesting_result.page_svgs.len() > 1 {
            nesting_result
                .page_svgs
                .last()
                .map(|page| encode_svg(page))
        } else {
            None
        };

        // Send final result to queue
        let response = SqsNestingResponse {
            correlation_id: request.correlation_id.clone(),
            first_page_svg_base64,
            last_page_svg_base64,
            parts_placed: nesting_result.parts_placed,
            is_improvement: false,
            is_final: true,
            timestamp: current_timestamp(),
            error_message: None,
        };

        self.send_to_output_queue(output_queue_url, &response)
            .await
            .context("Failed to send final result to queue")?;

        info!("Sent final result to queue");

        Ok(())
    }

    /// Listen and process messages from the queue
    pub async fn listen_and_process(
        &self,
        mut shutdown_rx: tokio::sync::broadcast::Receiver<()>,
    ) -> Result<()> {
        info!("Starting to listen on queue: {}", self.input_queue_url);

        loop {
            // Check for shutdown signal
            tokio::select! {
                _ = shutdown_rx.recv() => {
                    info!("Shutdown signal received, finishing current operations...");
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
                            // Check for shutdown before processing each message
                            if shutdown_rx.try_recv().is_ok() {
                                info!("Shutdown signal received, stopping message processing");
                                // Return the message to the queue by not deleting it
                                break;
                            }

                            let receipt_handle = message.receipt_handle()
                                .ok_or_else(|| anyhow::anyhow!("Message missing receipt handle"))?;
                            let body = message.body()
                                .ok_or_else(|| anyhow::anyhow!("Message missing body"))?;

                            if let Some(message_id) = message.message_id() {
                                info!("Received message from queue: {}", message_id);
                            } else {
                                info!("Received message from queue (no message_id present)");
                            }

                            // Process message (always sends response, success or error)
                            let process_result = self.process_message(receipt_handle, body).await;
                            
                            if let Err(e) = &process_result {
                                error!("Error during message processing: {}", e);
                            }
                            
                            // Check for shutdown before deleting message
                            if shutdown_rx.try_recv().is_ok() {
                                info!("Shutdown signal received, message will be reprocessed");
                                break;
                            }

                            // Always delete message after processing (success or error)
                            // Error responses have already been sent to output queue
                            if let Err(e) = self.sqs_client
                                .delete_message()
                                .queue_url(&self.input_queue_url)
                                .receipt_handle(receipt_handle)
                                .send()
                                .await
                            {
                                error!("Failed to delete message: {}", e);
                            } else {
                                info!("Acknowledged message from queue");
                            }
                        }
                    }
                }
            }
        }

        info!("Shutdown complete, exiting gracefully");
        Ok(())
    }
}
