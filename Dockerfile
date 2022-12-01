FROM rust:1.65 AS chef
ADD .cargo $CARGO_HOME/
RUN cargo install cargo-chef
# RUN rustup target add wasm32-unknown-unknown
# RUN cargo install trunk wasm-bindgen-cli

WORKDIR /app

FROM chef AS planner
ADD . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
RUN rustup target add wasm32-unknown-unknown
RUN cargo install trunk wasm-bindgen-cli

COPY . .
RUN cd web && trunk build --release
RUN cargo build --release

FROM debian:11.5
COPY --from=builder /app/target/release/tiny-url /app/server
COPY --from=builder /app/config /app/config
COPY --from=builder /app/dist /app/dist

WORKDIR /app
CMD ["./server"]