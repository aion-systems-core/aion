FROM rust:1.86-bookworm AS builder
WORKDIR /app
COPY . .
RUN cargo build -p aion-cli --release

FROM debian:bookworm-slim
RUN useradd -m -u 10001 aion
WORKDIR /work
COPY --from=builder /app/target/release/aion /usr/local/bin/aion
USER aion
ENTRYPOINT ["aion"]
