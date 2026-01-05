# Stage 1: Build Rust workspace ----------------------------------------------------------------------------------
FROM rust:1.92-slim AS rust-builder

# Install required dependencies
RUN \
  apt-get update \
  && apt-get install -y pkg-config libssl-dev curl \
  && rm -rf /var/lib/apt/lists/*
RUN \
  cargo install wasm-pack \
  && rustup target add wasm32-unknown-unknown

WORKDIR /app

# Copy Cargo files
RUN mkdir -p src crates/cli/src crates/core/src crates/openscad/src crates/wasm/src webapp/backend/src
COPY Cargo.toml Cargo.lock ./
COPY crates/cli/Cargo.toml crates/cli/
COPY crates/core/Cargo.toml crates/core/
COPY crates/openscad/Cargo.toml crates/openscad/
COPY crates/wasm/Cargo.toml crates/wasm/
COPY webapp/backend/Cargo.toml webapp/backend/

# Create dummy source files to build dependencies
RUN \
  echo "fn main() {}" > crates/cli/src/main.rs \
  && echo "fn main() {}" > webapp/backend/src/main.rs \
  && echo "pub fn dummy() {}" > src/lib.rs \
  && echo "pub fn dummy() {}" > crates/core/src/lib.rs \
  && echo "pub fn dummy() {}" > crates/openscad/src/lib.rs \
  && echo "pub fn dummy() {}" > crates/wasm/src/lib.rs

# Build dependencies (this layer will be cached)
RUN cargo build --release --workspace --exclude caustic-wasm
RUN cargo build --release --package caustic-wasm --target wasm32-unknown-unknown

# Remove the dummy build artifacts
RUN rm -rf \
  target/release/.fingerprint/caustic-* \
  target/release/deps/caustic-* \
  target/wasm32-unknown-unknown/release/.fingerprint/caustic-* \
  target/wasm32-unknown-unknown/release/deps/caustic-*

# Copy all workspace members
COPY src/ ./src
COPY crates/cli/src/ ./crates/cli/src
COPY crates/core/src/ ./crates/core/src
COPY crates/openscad/src/ ./crates/openscad/src
COPY crates/wasm/src/ ./crates/wasm/src
COPY webapp/backend/src/ ./webapp/backend/src

# Build in release mode
RUN \
  cargo build --release --workspace --exclude caustic-wasm \
  && cd /app/crates/wasm \
  && wasm-pack build --target web --release \
  && cd /app \
  && cp /app/target/release/caustic-webapp /app/target/release/caustic-cli /app/ \
  && mv /app/crates/wasm/pkg /app/wasm \
  && rm -rf target Cargo.lock Cargo.toml backend crates src webapp /usr/local/cargo/registry
RUN /app/caustic-webapp --write-swagger /app/openapi.json

# Stage 2: Build React frontend ----------------------------------------------------------------------------------
FROM ubuntu:noble AS frontend-builder

WORKDIR /frontend

# Install required dependencies
RUN \
  apt-get update \
  && apt-get install -y pkg-config libssl-dev curl openjdk-17-jre \
  && rm -rf /var/lib/apt/lists/*

# Install node
COPY webapp/frontend/.nvmrc ./
RUN \
  curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.40.3/install.sh | bash \
  && . /root/.nvm/nvm.sh || echo "ok" \
  && nvm install \
  && nvm use \
  && nvm alias default $(cat .nvmrc) \
  && ln -s "$NVM_DIR/versions/node/$(nvm current)/bin/node" /usr/local/bin/node \
  && ln -s "$NVM_DIR/versions/node/$(nvm current)/bin/npm" /usr/local/bin/npm \
  && ln -s "$NVM_DIR/versions/node/$(nvm current)/bin/npx" /usr/local/bin/npx

# Install dependencies
COPY webapp/frontend/package*.json ./
RUN npm ci

# Copy frontend source
COPY webapp/frontend/ ./
COPY --from=rust-builder /app/openapi.json /app/
COPY --from=rust-builder /app/wasm/* /frontend/src/wasm/

# Build the React app
RUN npm run build

# # Stage 3: Runtime image -----------------------------------------------------------------------------------------
FROM debian:trixie-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=rust-builder /app/caustic-webapp /app/caustic-webapp
COPY --from=frontend-builder /frontend/dist/assets/ /app/static
EXPOSE 8080

# Run the webserver
CMD ["/app/caustic-webapp"]
