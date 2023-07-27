FROM rust:1.70.0-alpine AS builder

RUN mkdir -p /app
WORKDIR /app
COPY . .

RUN cargo build --release

FROM alpine:latest

ARG PORT
ARG POOl_SIZE
ENV RUST_BACKTRACE=1
ENV RUST_LOG="info,warn,error,debug"

COPY --from=builder /app/target/release/rs-serve ./

EXPOSE ${PORT}

CMD ["./rs-serve"]
