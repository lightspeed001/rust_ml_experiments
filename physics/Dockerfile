# Base image
#FROM ubuntu:22.04
# Cuda Base Image
FROM nvidia/cuda:12.4.1-base-ubuntu22.04 

# Install system dependencies (libpng for MNIST, build tools, Rust)
ENV DEBIAN_FRONTEND=noninteractive
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
        curl \
        ca-certificates \
        build-essential \
        libpng-dev \
        pkg-config \
    && rm -rf /var/lib/apt/lists/*

# Install Rust
ENV RUSTUP_HOME=/usr/local/rustup \
    CARGO_HOME=/usr/local/cargo \
    PATH="/usr/local/cargo/bin:${PATH}"
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

# Copy project files
WORKDIR /app
COPY Cargo.toml .
COPY main.rs .

# Build and run
RUN cargo build --release
CMD ["cargo", "run", "--release"]
