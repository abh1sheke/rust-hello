FROM rust:1.70.0-alpine

ARG PORT
ARG POOL_SIZE

WORKDIR /usr/src/app

COPY . .

RUN cargo install --path .

EXPOSE ${PORT}

CMD [ "RUST_LOG=\"info,warn,error\"", "rs_serve" ]
