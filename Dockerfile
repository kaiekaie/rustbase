FROM rust:latest
ENV PORT=8080



WORKDIR /app
COPY . /app
RUN rustup toolchain install nightly-2023-02-15
RUN rustup default nightly-2023-02-15
RUN cargo build --release --all-features

ENTRYPOINT [ "./target/release" ]
CMD [ "rustplatform" ] 