```
# ---- build stage ----
FROM rust:1.90.0-bookworm AS builder
WORKDIR /app

# Copy workspace manifests first (layer caching)
COPY Cargo.toml Cargo.lock ./
COPY api/Cargo.toml api/Cargo.toml
COPY service/Cargo.toml service/Cargo.toml
COPY corelib/Cargo.toml corelib/Cargo.toml
COPY entity/Cargo.toml entity/Cargo.toml
COPY migration/Cargo.toml migration/Cargo.toml
COPY bot/Cargo.toml bot/Cargo.toml

# Dummy sources to let cargo fetch dependencies
RUN mkdir -p api/src service/src corelib/src entity/src migration/src bot/src
RUN echo 'fn main() {}' > api/src/main.rs
RUN echo 'pub fn _x(){}' > service/src/lib.rs
RUN echo 'pub fn _x(){}' > corelib/src/lib.rs
RUN echo 'pub fn _x(){}' > entity/src/lib.rs
RUN echo 'pub fn _x(){}' > migration/src/lib.rs
RUN echo 'fn main() {}' > bot/src/main.rs

RUN cargo build -p api --release

# Copy real source code and build again
RUN rm -rf api/src service/src corelib/src entity/src migration/src bot/src
COPY . .
RUN cargo build -p api --release

# Install SeaORM CLI to run migrations in container
RUN cargo install sea-orm-cli@1.1.16

# ---- runtime stage ----
FROM debian:bookworm-slim AS runtime
RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates curl bash && rm -rf /var/lib/apt/lists/*
WORKDIR /app

COPY --from=builder /app/target/release/api /app/api
COPY --from=builder /usr/local/cargo/bin/sea-orm-cli /usr/local/bin/sea-orm-cli

# Copy migration crate if you want to run directory-based migrations
COPY migration /app/migration
COPY api/templates /app/api/templates

COPY entrypoint.sh /app/entrypoint.sh
RUN chmod +x /app/entrypoint.sh

ENV RUST_LOG=info
EXPOSE 3000

RUN useradd -m appuser
USER appuser

HEALTHCHECK --interval=30s --timeout=3s --start-period=10s \
  CMD curl -fsS http://127.0.0.1:3000/api/health || exit 1

ENTRYPOINT ["/app/entrypoint.sh"]

```