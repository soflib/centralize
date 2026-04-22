# ============================================================
#  Dockerfile — TicketChain Backend Rust
# ============================================================

# ── Stage 1: Build ───────────────────────────────────────────
FROM rust:1.88-alpine AS builder

# solana-client necesita estas dependencias extra
RUN apk add --no-cache \
    musl-dev \
    pkgconfig \
    openssl-dev \
    protobuf-dev \
    clang \
    llvm \
    make \
    perl \
    libpq-dev

WORKDIR /app

COPY Cargo.toml Cargo.lock ./

# Main falso para cachear deps
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release 2>&1 || true
RUN rm -rf src

# Código real
COPY src ./src
RUN touch src/main.rs && cargo build --release

# ── Stage 2: Runtime ─────────────────────────────────────────
FROM alpine:3.19

RUN apk add --no-cache ca-certificates libgcc

WORKDIR /app

COPY --from=builder /app/target/release/backend /app/server

RUN mkdir -p /app/data /app/secrets
RUN addgroup -S appgroup && adduser -S appuser -G appgroup
RUN chown -R appuser:appgroup /app
USER appuser

EXPOSE 3001

CMD ["/app/server"]