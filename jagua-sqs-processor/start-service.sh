#!/bin/bash
# Script to start jagua-sqs-processor service with PID management

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PID_FILE="${SCRIPT_DIR}/start-service.pid"
LOG_DIR="${SCRIPT_DIR}/log"
LOG_FILE="${LOG_DIR}/service.log"
PROJECT_ROOT="$(cd "${SCRIPT_DIR}/.." && pwd)"

# Function to kill previous instance
kill_previous_instance() {
    if [ -f "$PID_FILE" ]; then
        OLD_PID=$(cat "$PID_FILE")
        if kill -0 "$OLD_PID" 2>/dev/null; then
            echo "Killing previous instance (PID: $OLD_PID)..."
            kill "$OLD_PID" 2>/dev/null
            sleep 2
            if kill -0 "$OLD_PID" 2>/dev/null; then
                echo "Process still running, forcing kill..."
                kill -9 "$OLD_PID" 2>/dev/null
            fi
            echo "Previous instance stopped."
        else
            echo "Previous instance (PID: $OLD_PID) is not running, removing stale PID file."
        fi
        rm -f "$PID_FILE"
    fi
}

# Function to start new instance
start_service() {
    # Create log directory if it doesn't exist
    mkdir -p "$LOG_DIR" || {
        echo "Error: Failed to create log directory: $LOG_DIR"
        exit 1
    }
    
    cd "$PROJECT_ROOT" || {
        echo "Error: Failed to change to project root directory: $PROJECT_ROOT"
        exit 1
    }
    
    # Set environment variables
    export AWS_ACCESS_KEY_ID="${AWS_ACCESS_KEY_ID}"
    export AWS_SECRET_ACCESS_KEY="${AWS_SECRET_ACCESS_KEY}"
    export AWS_REGION="${AWS_REGION:-eu-north-1}"
    export AWS_DEFAULT_REGION="$AWS_REGION"
    export AWS_ENDPOINT_URL="http://localhost:4566"
    export INPUT_QUEUE_URL="http://sqs.eu-north-1.localhost.localstack.cloud:4566/000000000000/nesting-request"
    export OUTPUT_QUEUE_URL="http://sqs.eu-north-1.localhost.localstack.cloud:4566/000000000000/nesting-response"
    export S3_BUCKET="${S3_BUCKET:-cutl-staging-uploads}"
    # RUST_LOG can be set in environment or .env file to override default Info level
    # Default is Info level (set in main.rs via filter_level)
    # Example: export RUST_LOG="jagua_utils=debug,jagua_rs=debug,geo::algorithm::relate=warn" for debug logs
    # If not set, filter_level(Info) in main.rs will be used
    
    echo "Starting jagua-sqs-processor service..."
    echo "AWS_ACCESS_KEY_ID: ${AWS_ACCESS_KEY_ID:0:10}..." # Only show first 10 chars for security
    echo "AWS_SECRET_ACCESS_KEY: ***" # Don't show secret key
    echo "AWS_REGION: $AWS_REGION"
    echo "AWS_ENDPOINT_URL: $AWS_ENDPOINT_URL"
    echo "INPUT_QUEUE_URL: $INPUT_QUEUE_URL"
    echo "OUTPUT_QUEUE_URL: $OUTPUT_QUEUE_URL"
    echo "S3_BUCKET: $S3_BUCKET"
    echo "RUST_LOG: $RUST_LOG"
    cargo run --bin jagua-sqs-processor >> "$LOG_FILE" 2>&1 &
    NEW_PID=$!
    echo $NEW_PID > "$PID_FILE"
    echo "Service started with PID: $NEW_PID"
    echo "PID stored in: $PID_FILE"
    echo "Logs are being written to: $LOG_FILE"
}

# Main execution
kill_previous_instance
start_service

