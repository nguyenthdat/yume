# Yume Rust Backend Dockerfile
# Multi-stage build for small final image

FROM rust:1.86-slim-bookworm AS builder

WORKDIR /app

# Install build dependencies
RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

# Copy workspace manifests first for caching
COPY Cargo.toml Cargo.lock ./
COPY crates/yume-backend/Cargo.toml crates/yume-backend/
COPY crates/yume-contracts/Cargo.toml crates/yume-contracts/

# Create dummy source to cache dependency builds
RUN mkdir -p crates/yume-backend/src crates/yume-contracts/src && \
    echo "fn main() {}" > crates/yume-backend/src/main.rs && \
    echo "" > crates/yume-contracts/src/lib.rs && \
    cargo build --release -p yume-backend && \
    rm -rf crates/yume-backend/src crates/yume-contracts/src

# Copy real source
COPY crates/ crates/

# Build with timestamp fix for reproducible builds
RUN touch crates/yume-backend/src/main.rs && \
    cargo build --release -p yume-backend

# Runtime stage
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/yume-backend /usr/local/bin/yume-backend

EXPOSE 3000

CMD ["yume-backend"]
