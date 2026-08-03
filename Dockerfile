# =========================
# Build Stage
# =========================
FROM rust:1.88 AS builder

WORKDIR /app

# Enable SQLx offline mode
ENV SQLX_OFFLINE=true

# Copy dependency files
COPY Cargo.toml Cargo.lock ./

# Copy the SQLx cache
COPY .sqlx ./.sqlx

# Create dummy source to cache dependencies
RUN mkdir src && echo "fn main() {}" > src/main.rs

RUN cargo build --release

# Remove dummy source
RUN rm -rf src

# Copy actual source
COPY src ./src

# Build the real application
RUN cargo build --release


# =========================
# Runtime Stage
# =========================
FROM debian:bookworm-slim

WORKDIR /app

COPY --from=builder /app/target/release/task_flow_api .

EXPOSE 4000

CMD ["./task_flow_api"]
