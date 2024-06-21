FROM rust:1.79.0
RUN mkdir /rust-api-tests
WORKDIR /rust-api-tests
COPY ./ ./

RUN cargo build
ENTRYPOINT ["cargo", "test"]
