FROM rust:alpine

RUN apk update
RUN apk add musl-dev

WORKDIR /usr/src/userservice
COPY . .

RUN cargo install --path .
# todo for static linking
# RUN RUSTFLAGS='-C target-feature=-crt-static' cargo build --release

CMD ["userservice"]
