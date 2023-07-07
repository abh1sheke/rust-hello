FROM rust:1.49

ARG PORT
ARG POOL_SIZE

WORKDIR /usr/src/app

COPY . .

RUN cargo build --release

EXPOSE ${PORT}

CMD [ "RUST_LOG=\"info,warn,error\"", "./target/release/app" ]
