FROM rust:alpine3.17
ENV PORT=8080

RUN apk update

RUN apk add libpq gcc
RUN apk add --no-cache musl-dev
RUN  rustup toolchain install nightly-2023-02-15
WORKDIR /app
COPY . /app
RUN cargo build --release --all-features
ENTRYPOINT [ "cargo" ]
CMD [ "run" ]