FROM rust:alpine3.17
ENV PORT=8080

RUN apk update

RUN apk add libpq  gcc
RUN apk add --no-cache musl-dev
WORKDIR /app
COPY . /app
RUN cargo build
ENTRYPOINT [ "cargo" ]
CMD [ "run" ]