# Build stage
FROM rust:1.85 as builder

WORKDIR /usr/src/app

# Copy workspace manifest and lockfile first for better layer caching
COPY Cargo.toml Cargo.lock ./

# Copy all service directories (needed for workspace resolution)
COPY services/ ./services/

# Build the timer-service binary
RUN cargo build --release --bin timer-service

# Runtime stage
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && \
    apt-get install -y libssl-dev ca-certificates && \
    rm -rf /var/lib/apt/lists/*

# Copy the built binary from builder stage
COPY --from=builder /usr/src/app/target/release/timer-service /usr/local/bin/timer-service

# Run the service
CMD ["timer-service"]
