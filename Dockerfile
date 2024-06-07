FROM rust:latest as builder
RUN rustup target add x86_64-unknown-linux-musl
RUN apt update && apt install -y musl-tools musl-dev
RUN update-ca-certificates

WORKDIR /test-rest
COPY . .
RUN cargo build --target x86_64-unknown-linux-musl --release

FROM alpine:latest
WORKDIR /test-rest
COPY --from=builder /test-rest/target/x86_64-unknown-linux-musl/release/test-rest ./
CMD [ "/test-rest/test-rest" ]
EXPOSE 8080
ENV ADDRESS=0.0.0.0