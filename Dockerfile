FROM rust:1-alpine AS builder
WORKDIR /usr/src/luna
COPY . .
RUN apk add --no-cache build-base
RUN cargo install --path ./cli --locked

FROM alpine:latest
COPY --from=builder /usr/local/cargo/bin/luna_cli /usr/local/bin/luna_cli
CMD ["luna_cli"]
