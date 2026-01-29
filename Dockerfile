FROM rust:1.93-slim

ENV CARGO_HOME=/usr/local/cargo \
    RUSTUP_HOME=/usr/local/rustup \
    PATH=/usr/local/cargo/bin:$PATH

WORKDIR /workspaces/raytrace_rs

# Install essential development tools and dependencies
RUN apt-get update && apt-get install -y --no-install-recommends \
    git \
    openssh-client \
    curl \
    ca-certificates \
    build-essential \
    pkg-config \
    libssl-dev \
  && rm -rf /var/lib/apt/lists/*

# Install useful Rust tools
RUN cargo install cargo-watch cargo-edit \
ke  && rustup component add clippy rustfmt
