# 1. Build stage
FROM rust:1.71 AS builder
WORKDIR /app

# Cache dependencies
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release

# Copy real source and build
COPY . .
RUN cargo sqlx prepare -- --lib   # prepare offline SQLx if using macros
RUN cargo build --release

# 2. Runtime stage
FROM alpine:3.18
RUN apk add --no-cache libgcc openssl
WORKDIR /app

# Copy binary from builder
COPY --from=builder /app/target/release/url_shortener .

# Expose port and run
ENV RUST_LOG=info
CMD ["./url_shortener"]
