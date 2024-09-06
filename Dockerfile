FROM lukemathwalker/cargo-chef:latest-rust-1 AS chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder 
COPY --from=planner /app/recipe.json recipe.json

RUN cargo chef cook --release --recipe-path recipe.json

COPY . .
RUN cargo build --release

FROM debian:bookworm-slim AS runtime
WORKDIR /app
COPY --from=builder /app/target/release/files /usr/local/bin

COPY favicon.ico /app/favicon.ico

ENV PORT=3000
ENV METRICS_PORT=3001
EXPOSE 3000
EXPOSE 3001

ENTRYPOINT ["/usr/local/bin/files"]