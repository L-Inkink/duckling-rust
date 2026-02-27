# Multi-stage Dockerfile for Rustling HTTP Server
# Optimized for small image size and production deployment

# ============================================================================
# Stage 1: Builder - Compile Rust application
# ============================================================================
FROM rust:1.70-slim AS builder

WORKDIR /app

# Install build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Copy manifests
COPY Cargo.toml Cargo.lock ./
COPY core/Cargo.toml ./core/
COPY ml/Cargo.toml ./ml/

# Copy source code
COPY core ./core
COPY ml ./ml
COPY src ./src
COPY examples ./examples

# Build release binary with server feature
# Use --release for optimized build
# Link statically to reduce runtime dependencies
RUN cargo build --release --features server --bin http_server

# Strip debug symbols to reduce binary size
RUN strip /app/target/release/http_server

# ============================================================================
# Stage 2: Runtime - Minimal image with only the binary
# ============================================================================
FROM alpine:3.19

# Install runtime dependencies (alpine uses apk)
RUN apk add --no-cache ca-certificates curl

# Create non-root user for security
RUN adduser -D -u 1000 rustling && \
    mkdir -p /app /app/rules && \
    chown -R rustling:rustling /app

# Set working directory
WORKDIR /app

# Copy binary from builder
COPY --from=builder --chown=rustling:rustling \
    /app/target/release/http_server /usr/local/bin/rustling-server

# Copy rules directory (if exists)
COPY --chown=rustling:rustling rules ./rules

# Switch to non-root user
USER rustling

# Environment variables with sensible defaults
ENV RUST_LOG=info \
    BIND_ADDRESS=0.0.0.0:8080 \
    RUST_BACKTRACE=1

# Expose HTTP port
EXPOSE 8080

# Health check using the /health endpoint
HEALTHCHECK --interval=30s \
            --timeout=3s \
            --start-period=5s \
            --retries=3 \
    CMD curl -f http://localhost:8080/health || exit 1

# Run the HTTP server
CMD ["http_server"]
