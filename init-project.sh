#!/bin/sh

# Check if Cargo.toml exists in the current directory
if [ ! -f Cargo.toml ]; then
    echo "Cargo.toml not found. Initializing Rust project..."
    cargo init --name myapp .
else
    echo "Cargo.toml found. Skipping cargo init."
fi

# Build the project
echo "Building the project..."
cargo build

# Check if the SQLite database file exists, and if not, run migrations
# first time in container, manually run:
# diesel migration run
# diesel print-schema > src/schema.rs
#
if [ ! -f "$DATABASE_URL" ]; then
  echo "Database file not found. Running migrations..."
  diesel migration run
else
  echo "Database file found. Skipping migrations."
fi

# Get latest dependencies
echo "Updating dependencies..."
cargo update
# Then start your application
exec cargo run

