FROM rust:1.77-slim-bullseye as builder 

RUN mkdir -p /app 

RUN apt-get update && \
    apt-get install -y make cmake libssl-dev libsasl2-dev libzstd-dev curl librdkafka-dev g++ ca-certificates

WORKDIR /app

COPY . .

RUN cargo install --path .

FROM debian:bullseye-slim as final

RUN apt-get update

COPY --from=builder /usr/local/cargo/bin/be /usr/local/bin/be

EXPOSE 9090

CMD ["be"]
