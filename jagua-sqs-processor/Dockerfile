# Build stage
FROM rust:1.75-slim as builder

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
COPY jagua-sqs-processor/src ./jagua-sqs-processor/src

# Build the application in release mode
WORKDIR /app
RUN cargo build --release --package jagua-sqs-processor

# Runtime stage
FROM debian:bookworm-slim

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

# Run the application
ENTRYPOINT ["/app/jagua-sqs-processor"]

