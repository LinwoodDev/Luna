FROM rust:1-alpine AS builder
WORKDIR /usr/src/luna
COPY . .
# Install build tools and Node.js for npm dependencies required by cli/build.rs
RUN apk add --no-cache build-base nodejs npm pkgconfig openssl-dev openssl-libs-static
# Install pnpm
RUN npm install --global corepack@latest
# Install package.json dependencies in cli directoy
WORKDIR /usr/src/luna/cli
ENV CI=true
RUN pnpm install
# Build and install the CLI
WORKDIR /usr/src/luna
RUN cargo install --path ./cli --locked

FROM alpine:latest
COPY --from=builder /usr/local/cargo/bin/luna_cli /usr/local/bin/luna_cli
COPY cli/entrypoint.sh /entrypoint.sh
RUN chmod +x /entrypoint.sh
ENTRYPOINT ["/entrypoint.sh"]
