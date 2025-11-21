mod processor;

use anyhow::{Context, Result};
use aws_config::BehaviorVersion;
use aws_sdk_sqs::Client as SqsClient;
use log::{info, warn};
use processor::SqsProcessor;
use std::env;
use tokio::signal;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logger
    env_logger::Builder::from_default_env()
        .filter_level(log::LevelFilter::Info)
        .init();

    info!("Starting jagua-sqs-processor");

    // Get configuration from environment variables
    let input_queue_url =
        env::var("INPUT_QUEUE_URL").context("INPUT_QUEUE_URL environment variable is required")?;
    let output_queue_url = env::var("OUTPUT_QUEUE_URL")
        .context("OUTPUT_QUEUE_URL environment variable is required")?;

    info!("Configuration:");
    info!("  INPUT_QUEUE_URL: {}", input_queue_url);
    info!("  OUTPUT_QUEUE_URL: {}", output_queue_url);

    // Initialize AWS clients
    let mut config_loader = aws_config::defaults(BehaviorVersion::latest());
    
    // Configure LocalStack endpoint if provided
    if let Ok(endpoint_url) = env::var("AWS_ENDPOINT_URL") {
        config_loader = config_loader.endpoint_url(&endpoint_url);
        info!("Using AWS endpoint: {}", endpoint_url);
    } else if let Ok(endpoint_url) = env::var("AWS_ENDPOINT_URL_SQS") {
        config_loader = config_loader.endpoint_url(&endpoint_url);
        info!("Using SQS endpoint: {}", endpoint_url);
    }
    
    let config = config_loader.load().await;
    let sqs_client = SqsClient::new(&config);

    // Create processor
    let processor = SqsProcessor::new(sqs_client, input_queue_url, output_queue_url);

    // Create shutdown channel
    let (shutdown_tx, shutdown_rx) = tokio::sync::broadcast::channel::<()>(1);

    // Spawn signal handler
    let mut sigterm = signal::unix::signal(signal::unix::SignalKind::terminate())
        .context("Failed to register SIGTERM handler")?;
    let mut sigint = signal::unix::signal(signal::unix::SignalKind::interrupt())
        .context("Failed to register SIGINT handler")?;

    let shutdown_tx_clone = shutdown_tx.clone();
    tokio::spawn(async move {
        tokio::select! {
            _ = sigterm.recv() => {
                info!("Received SIGTERM, initiating graceful shutdown...");
                let _ = shutdown_tx_clone.send(());
            }
            _ = sigint.recv() => {
                info!("Received SIGINT, initiating graceful shutdown...");
                let _ = shutdown_tx_clone.send(());
            }
        }
    });

    // Start listening and processing
    let result = processor.listen_and_process(shutdown_rx).await;

    // Give a moment for any final cleanup
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

    if let Err(e) = &result {
        warn!("Processor exited with error: {}", e);
    }

    result
}
