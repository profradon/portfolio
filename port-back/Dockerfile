FROM rust:1.85-slim-bookworm AS builder

WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
WORKDIR /app
COPY --from=builder /app/target/release/port-back /app/port-back

EXPOSE 8080
CMD ["./port-back"]
