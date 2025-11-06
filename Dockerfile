# Build stage
FROM rust:latest as builder

WORKDIR /app

# Install dependencies
RUN apt-get update && apt-get install -y \
    build-essential \
    pkg-config \
    libssl-dev \
    clang \
    && rm -rf /var/lib/apt/lists/*

# Copy source
COPY . .

# Build
RUN rustup target add wasm32-unknown-unknown
RUN rustup component add rust-src
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

WORKDIR /app

RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/reputechain-node /usr/local/bin/

EXPOSE 9944 9933 30333

CMD ["reputechain-node", "--dev", "--ws-external", "--rpc-external"]
