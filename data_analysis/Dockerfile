FROM rust:1.76.0 as builder

RUN apt-get update && apt-get install -y protobuf-compiler

COPY proto /proto

WORKDIR /data_analysis
COPY data_analysis .
COPY config.json .
RUN cargo build

CMD ["cargo", "run"]