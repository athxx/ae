FROM rust:latest AS builder
WORKDIR /app
COPY ./ .
RUN cargo build --release

##### Runtime alpine:edge
FROM alpine:latest
WORKDIR /www
COPY --from=builder /acorn/target/x86_64-unknown-linux-musl/release/acorn ./
CMD ["/www/acorn"]


# run 
# docker build -t allo:alpine -f Dockerfile.alpine .
# docker run -ti --rm --env-file .env allo:alpine