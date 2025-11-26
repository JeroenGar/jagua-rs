# Build stage
FROM rustlang/rust:nightly-slim AS builder

WORKDIR /app

# Install build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Copy workspace configuration first (for better Docker layer caching)
COPY Cargo.toml Cargo.lock ./

# Copy all Cargo.toml files for dependency resolution
COPY jagua-rs/Cargo.toml ./jagua-rs/
COPY jagua-utils/Cargo.toml ./jagua-utils/
COPY lbf/Cargo.toml ./lbf/
COPY jagua-sqs-processor/Cargo.toml ./jagua-sqs-processor/

# Copy source code for all dependencies
COPY jagua-rs/src ./jagua-rs/src
COPY jagua-utils/src ./jagua-utils/src
COPY lbf/src ./lbf/src
COPY lbf/benches ./lbf/benches
COPY jagua-sqs-processor/src ./jagua-sqs-processor/src

# Build the application in release mode
# Use BuildKit cache mount for faster dependency downloads
# Note: Not caching /app/target to avoid stale build artifacts
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/local/cargo/git \
    cargo build --release --package jagua-sqs-processor

# Runtime stage
FROM debian:unstable-slim

WORKDIR /app

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

# Copy the binary from builder
COPY --from=builder /app/target/release/jagua-sqs-processor /app/jagua-sqs-processor

# Set environment variables (can be overridden)
ENV RUST_LOG=info
ENV RUST_BACKTRACE=1
ENV INPUT_QUEUE_URL=nesting-request
ENV OUTPUT_QUEUE_URL=nesting-response

# Run the application
ENTRYPOINT ["/app/jagua-sqs-processor"]

