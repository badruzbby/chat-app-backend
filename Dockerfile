# Tahap 1: Builder dengan target musl untuk static linking
FROM rust:slim as builder

WORKDIR /app
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    musl-tools \
    && rm -rf /var/lib/apt/lists/*

# Tambahkan target musl
RUN rustup target add x86_64-unknown-linux-musl

# Salin Cargo.toml terlebih dahulu
COPY Cargo.toml ./

# Buat Cargo.lock jika belum ada
RUN touch Cargo.lock
COPY Cargo.lock* ./

RUN mkdir src \
    && echo "fn main() {}" > src/main.rs \
    && cargo build --release --target x86_64-unknown-linux-musl \
    && rm -rf src

COPY . .

# Mengaktifkan flag CI untuk melewati verifikasi query SQLx
ENV SQLX_OFFLINE=true
ENV RUSTFLAGS="--cfg ci"

RUN cargo build --release --target x86_64-unknown-linux-musl

# Gunakan image minimal alpine untuk deployment
FROM alpine:latest

WORKDIR /app

# Instal SSL untuk koneksi database
RUN apk --no-cache add ca-certificates

COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/backend /app/backend
COPY --from=builder /app/migrations /app/migrations

EXPOSE 8080

ENV HOST=0.0.0.0
ENV PORT=8080

CMD ["./backend"] 