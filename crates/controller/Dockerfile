# --- Build stage ---
FROM rust:1-slim AS build
WORKDIR /app

# Build dependencies (context is repo root for workspace)
COPY Cargo.toml Cargo.lock ./
COPY crates ./crates

RUN cargo build --release -p controller

# --- Production stage ---
FROM debian:trixie-slim AS production
WORKDIR /app

# k8s client needs CA certs for TLS
RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY --from=build /app/target/release/controller /usr/local/bin/controller

EXPOSE 3000
CMD ["controller"]
