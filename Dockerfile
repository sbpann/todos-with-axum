FROM rust:1.77.0-alpine3.19

WORKDIR /build
RUN apk add --no-cache musl-dev
RUN apk add --no-cache postgresql-dev

COPY src src
COPY Cargo.toml Cargo.toml

ENV RUSTFLAGS="-C target-feature=-crt-static"
RUN cargo build --release

FROM alpine:3.19

RUN apk add --no-cache libpq
RUN apk add --no-cache libgcc

WORKDIR /app
COPY --from=0 /build/target/release .
COPY .env .env

ENTRYPOINT ["./todos-with-axum"]