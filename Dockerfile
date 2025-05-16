# syntax=docker/dockerfile:1.4

# ----------- Build Stage -----------
FROM rust:1.86-slim-bookworm AS builder

WORKDIR /app

# Install required build dependencies
RUN apt-get update && \
    apt-get install -y --no-install-recommends pkg-config libpq-dev libsqlite3-dev libmariadb-dev libmariadb-dev-compat ca-certificates curl xz-utils && \
    rm -rf /var/lib/apt/lists/*

# Install diesel CLI for migrations
RUN cargo install diesel_cli

# Cache dependencies
COPY backend-api/Cargo.toml backend-api/Cargo.lock ./
COPY backend-api/api/Cargo.toml api/
COPY backend-api/db/Cargo.toml db/
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release || true

# Copy source code
RUN mkdir -p api/src src db/src
COPY backend-api/src src
COPY backend-api/api/src api/src
COPY backend-api/db/src db/src

# Build the actual binary
RUN cargo build --release

# ----------- Runtime Stage -----------
FROM debian:bookworm-slim AS runtime

# Install runtime dependencies
RUN apt-get update && \
    apt-get install -y --no-install-recommends libpq-dev libsqlite3-dev libmariadb-dev libmariadb-dev-compat libpq5 ca-certificates curl && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy the compiled binary and migrations
COPY --from=builder /app/target/release/synnapse-db-api-cli .
COPY backend-api/db/migrations ./db/migrations
COPY --from=builder /usr/local/cargo/bin/diesel /usr/local/bin/diesel
RUN echo "[migrations_directory]\ndir = \"db/migrations\"" >> diesel.toml
COPY ./backend-api/.env .

# Entrypoint: seed the database, then run the API server
# Expects DATABASE_URL env var to be set (e.g., postgres://user:pass@db:5432/dbname)
ENV RUST_LOG=info
ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_PORT=8000

ENTRYPOINT ["/bin/sh", "-c", "diesel migration run || true && ./synnapse-db-api-cli seed || true && exec ./synnapse-db-api-cli serve"]

# Expose the port Rocket uses by default
EXPOSE 8000
