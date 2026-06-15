# Yume Rust Backend Dockerfile
# Multi-stage build: compile in builder image, ship minimal runtime.

FROM rust:1.86-slim-bookworm AS builder

WORKDIR /app

# Install build dependencies (openssl for reqwest TLS)
RUN apt-get update && \
    apt-get install -y pkg-config libssl-dev && \
    rm -rf /var/lib/apt/lists/*

# Copy the entire workspace
COPY Cargo.toml Cargo.lock ./
COPY crates/ crates/

# Build backend in release mode
RUN cargo build --release -p yume-backend

# ---------------------------------------------------------------------------
# Runtime stage
# ---------------------------------------------------------------------------
FROM debian:bookworm-slim

RUN apt-get update && \
    apt-get install -y ca-certificates && \
    rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/yume-backend /usr/local/bin/yume-backend

EXPOSE 3000

CMD ["yume-backend"]
