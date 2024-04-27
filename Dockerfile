FROM rust:1-alpine3.19 as builder

WORKDIR /app

RUN apk add musl-dev

COPY . .

RUN cargo build --release

FROM scratch

COPY --from=builder /app/target/release/is_ready .

ENTRYPOINT [ "./is_ready" ]