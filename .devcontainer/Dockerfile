# Use the official Rust image.
FROM rust:latest

# Set the working directory in the Docker container.
WORKDIR /usr/src/app

RUN DEBIAN_FRONTEND="noninteractive" apt-get update -y && \
    apt-get install -y \
    curl \
    netcat-traditional \
    procps \
    ssh \
    git \
    net-tools \
    htop \
    build-essential \
    nodejs \
    npm \
    sudo \
    wget \
    sqlite3 \
    libsqlite3-dev 

RUN apt-get clean && rm -rf /var/lib/apt/lists/*

# Install GitHub CLI
RUN ARCH=$(dpkg --print-architecture) && \
    GH_VERSION=$(curl -s https://api.github.com/repos/cli/cli/releases/latest | grep tag_name | cut -d '"' -f 4 | cut -c 2-) && \
    curl -sSL "https://github.com/cli/cli/releases/latest/download/gh_${GH_VERSION}_linux_${ARCH}.deb" -o gh.deb && \
    dpkg -i gh.deb && \
    rm gh.deb

# Install Goose CLI (non-interactive)
RUN curl -fsSL https://github.com/block/goose/releases/download/stable/download_cli.sh | CONFIGURE=false bash

# Install Aider
RUN curl -LsSf https://aider.chat/install.sh | sh

# Install Diesel CLI with SQLite features
RUN cargo install diesel_cli --no-default-features --features "sqlite"



# Install cargo-expand
# RUN cargo install cargo-expand

# Copy the contents of your project to the container.
COPY . .

# Since we don't have the project initialized yet, we don't run `cargo build` here.
# Initialization will be done later from VS Code.
