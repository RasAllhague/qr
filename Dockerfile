# ---- build stage ----
FROM rust:1.80-bookworm AS builder
WORKDIR /app

# 1) Copy workspace manifests first (for layer caching)
COPY Cargo.toml Cargo.lock ./
COPY api/Cargo.toml api/Cargo.toml
COPY service/Cargo.toml service/Cargo.toml
COPY corelib/Cargo.toml corelib/Cargo.toml
COPY entity/Cargo.toml entity/Cargo.toml
COPY migration/Cargo.toml migration/Cargo.toml
COPY bot/Cargo.toml bot/Cargo.toml

# Create dummy files to build deps
RUN mkdir -p api/src service/src corelib/src entity/src migration/src bot/src
RUN echo 'fn main() {}' > api/src/main.rs
RUN echo 'pub fn _x(){}' > service/src/lib.rs
RUN echo 'pub fn _x(){}' > corelib/src/lib.rs
RUN echo 'pub fn _x(){}' > entity/src/lib.rs
RUN echo 'pub fn _x(){}' > migration/src/lib.rs
RUN echo 'fn main() {}' > bot/src/main.rs

# Pre-build dependencies for faster incremental builds
RUN cargo build -p api --release

# 2) Now copy real source code
RUN rm -rf api/src service/src corelib/src entity/src migration/src bot/src
COPY . .
RUN cargo build -p api --release

# ---- runtime stage ----
FROM debian:bookworm-slim AS runtime
RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates curl && rm -rf /var/lib/apt/lists/*
WORKDIR /app

# Copy built binary from builder
COPY --from=builder /app/target/release/api /app/api

# Copy templates (Askama usually embeds them, but copy for safety)
COPY api/templates /app/api/templates

ENV RUST_LOG=info
EXPOSE 3000

# Use a non-root user
RUN useradd -m appuser
USER appuser

HEALTHCHECK --interval=30s --timeout=3s --start-period=10s \
  CMD curl -fsS http://127.0.0.1:3000/api/health || exit 1

CMD ["/app/api"]
