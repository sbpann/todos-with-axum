FROM rust:1.77.0-bookworm

WORKDIR /build
COPY src src
COPY Cargo.toml Cargo.toml

RUN cargo build --release
RUN ls
RUN cat Cargo.toml

FROM debian:bookworm

RUN apt update && apt update -y
RUN apt install -y libpq-dev

WORKDIR /app
COPY --from=0 /build/target/release .
COPY .env .env

ENTRYPOINT ["./todos-with-axum"]