# Multi-stage Dockerfile for TraderGrader MCP Server
# Optimized for small image size and security

# Build stage
FROM rust:latest as builder

# Install dependencies for building
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Create app user
RUN adduser --disabled-password --gecos '' --uid 1000 appuser

# Set working directory
WORKDIR /app

# Copy Cargo files for dependency caching
COPY Cargo.toml Cargo.lock ./

# Create a dummy main.rs to build dependencies
RUN mkdir src && echo "fn main() {}" > src/main.rs

# Build dependencies (this layer will be cached)
RUN cargo build --release && rm -rf src

# Copy source code
COPY src ./src

# Build the application
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Create app user (same UID as builder stage)
RUN adduser --disabled-password --gecos '' --uid 1000 appuser

# Create app directory
WORKDIR /app

# Copy the binary from builder stage
COPY --from=builder /app/target/release/tradergrader /usr/local/bin/tradergrader

# Copy scripts (optional, for containerized utilities)
COPY scripts/ ./scripts/

# Change ownership to app user
RUN chown -R appuser:appuser /app && \
    chmod +x /usr/local/bin/tradergrader && \
    chmod +x ./scripts/*.sh

# Switch to non-root user
USER appuser

# Health check
HEALTHCHECK --interval=30s --timeout=10s --start-period=5s --retries=3 \
    CMD [\"/usr/local/bin/tradergrader\", \"--health\"]

# Labels for metadata
LABEL org.opencontainers.image.title="TraderGrader"
LABEL org.opencontainers.image.description="EVE Online Market Data MCP Server"
LABEL org.opencontainers.image.version="0.1.0"
LABEL org.opencontainers.image.vendor="TraderGrader Project"
LABEL org.opencontainers.image.source="https://github.com/fuuijin/tradergrader"

# Expose port (if needed for future HTTP API)
# EXPOSE 8080

# Set entrypoint for CLI tool
ENTRYPOINT ["/usr/local/bin/tradergrader"]
CMD []