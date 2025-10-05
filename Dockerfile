# Use the official Rust image
FROM rust:1.90-slim as builder

# Set the working directory
WORKDIR /app

# Copy the workspace files
COPY Cargo.toml Cargo.lock ./
COPY crates/ ./crates/

# Build the API binary
RUN cargo build --release --bin api

# Use a minimal runtime image
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Copy the binary from the builder stage
COPY --from=builder /app/target/release/api /usr/local/bin/api

# Expose the port
EXPOSE 3000

# Set the startup command
CMD ["api"]
