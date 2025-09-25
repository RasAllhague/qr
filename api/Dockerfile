# Single-stage: build & run in the same container (simple)
FROM rust:latest

# SQLite build/runtime deps + certs
RUN apt-get update && apt-get install -y --no-install-recommends \
    libsqlite3-dev ca-certificates && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy the entire workspace so path deps (../entity, ../service, etc.) resolve
COPY . .

# Build the API crate
RUN cargo build --release -p api

# Install SeaORM CLI so you can run migrations inside the container
RUN cargo install sea-orm-cli

# Defaults (override at runtime)
ENV DATABASE_URL=sqlite:///data/qrcode.db?mode=rwc \
    SERVER_URL=0.0.0.0:3000 \
    IMAGE_BASE_PATH=/data/images \
    DOMAIN_NAME=rasalhague.de \
    RUST_LOG=info

# Ensure data path exists (first run)
RUN mkdir -p /data/images

# Persist DB & images across container restarts/removals
VOLUME ["/data"]

EXPOSE 3000
CMD ["./target/release/api"]
