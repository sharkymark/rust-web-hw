# Use the official Rust image.
FROM rust:latest

# Set the working directory in the Docker container.
WORKDIR /usr/src/app

# utility to query the database
RUN apt-get update && apt-get install libsqlite3-dev

# Install Diesel CLI with SQLite features
RUN cargo install diesel_cli --no-default-features --features "sqlite"

# Install cargo-expand
RUN cargo install cargo-expand

# Copy the contents of your project to the container.
COPY . .

# Since we don't have the project initialized yet, we don't run `cargo build` here.
# Initialization will be done later from VS Code.
