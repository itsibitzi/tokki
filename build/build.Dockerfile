
FROM ubuntu:24.04

# Install dependencies
RUN apt-get update && apt-get install -y \
    curl \
    build-essential \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Install rustup
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

# Install cargo-deb
RUN cargo install cargo-deb

# Set working directory
WORKDIR /app

# Copy source code
COPY \
    --exclude=scripts \
    --exclude=target \
    . .

ENV RUSTFLAGS "-C target-feature=+aes,+neon".
