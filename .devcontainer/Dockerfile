# Use the official Rust development container as base
FROM mcr.microsoft.com/devcontainers/rust:1-1-bookworm

# Install additional system packages that might be useful for astronomy calculations
RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
    && apt-get -y install --no-install-recommends \
        # Development tools
        build-essential \
        pkg-config \
        # Version control and utilities
        git \
        curl \
        wget \
        unzip \
        # Documentation and benchmarking tools
        # pandoc \
        # Network tools for potential data downloads
        ca-certificates \
        # Additional development utilities
        tree \
        htop \
        vim \
        nano \
    && apt-get autoremove -y && apt-get clean -y && rm -rf /var/lib/apt/lists/*

# Install additional Rust tools for better development experience
RUN rustup component add rustfmt clippy \
    && cargo install --locked \
        cargo-watch \
        cargo-edit \
        cargo-tree \
        cargo-outdated \
        cargo-audit \
        cargo-expand \
        cargo-machete \
        cargo-deny

# Set up development environment
USER vscode
WORKDIR /workspaces

# Set environment variables for better Rust development
ENV RUST_LOG=info
ENV RUST_BACKTRACE=1
ENV CARGO_TARGET_DIR=/tmp/target

# Create a .bashrc addition for better development experience
RUN echo 'alias ll="ls -alF"' >> ~/.bashrc \
    && echo 'alias la="ls -A"' >> ~/.bashrc \
    && echo 'alias l="ls -CF"' >> ~/.bashrc \
    && echo 'alias cb="cargo build"' >> ~/.bashrc \
    && echo 'alias ct="cargo test"' >> ~/.bashrc \
    && echo 'alias cc="cargo check"' >> ~/.bashrc \
    && echo 'alias cr="cargo run"' >> ~/.bashrc \
    && echo 'alias cf="cargo fmt"' >> ~/.bashrc \
    && echo 'alias cclippy="cargo clippy"' >> ~/.bashrc \
    && echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.bashrc