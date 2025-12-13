FROM mcr.microsoft.com/devcontainers/rust:2.0.3-trixie

RUN apt-get update && apt-get install -y linux-perf

RUN rustup target add wasm32-unknown-unknown && cargo install wasm-pack
RUN cargo install flamegraph
RUN rm -rf /usr/local/cargo/registry

# install node
RUN curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.40.3/install.sh | bash
