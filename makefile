# Makefile for Rust project

# Binary name, optional: set to your project name if needed
BIN_NAME = calculator

# Default target
all: build

# Build the project
build:
	cargo build

# Run the project
run:
	cargo run

# Run tests
test:
	cargo test

# Format the code
fmt:
	cargo fmt

# Lint the code using Clippy
lint:
	cargo clippy -- -D warnings

# Clean target (removes target directory)
clean:
	cargo clean

# Build release version
release:
	cargo build --release

# Run release binary
run-release:
	./target/release/$(BIN_NAME)


.PHONY: all build run test fmt lint clean release run-release watch
