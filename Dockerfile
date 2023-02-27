FROM rust:alpine3.17
ENV PORT=8080

RUN apk update

RUN apk add libpq gcc
RUN apk add --no-cache musl-dev

WORKDIR /app
COPY . /app
RUN rustup toolchain install nightly-2023-02-15
RUN rustup default nightly-2023-02-15

RUN cargo build --release --all-features
ENTRYPOINT [ "./" ]
CMD [ "target/debug/rustplatform" ]