.PHONY: setup build run clean help

TARGET := wasm32-wasip2
BINARY_NAME := s3-wasm
WASM_FILE := target/$(TARGET)/release/$(BINARY_NAME).wasm

# Try to find wasmtime in common locations
WASMTIME := $(shell which wasmtime 2>/dev/null || echo /opt/homebrew/opt/wasmtime/bin/wasmtime)

# Default S3 configuration (can be overridden)
S3_BUCKET ?= wasi-s3-dm
S3_KEY ?= data.csv

help:
	@echo "Available targets:"
	@echo "  setup  - Install wasip2 target and wasmtime"
	@echo "  build  - Build the WASM binary"
	@echo "  run    - Run the WASM binary with wasmtime"
	@echo "  clean  - Clean build artifacts"
	@echo ""
	@echo "Environment variables:"
	@echo "  S3_BUCKET - S3 bucket name (default: my-bucket)"
	@echo "  S3_KEY    - S3 object key (default: data.csv)"

setup:
	@echo "Installing wasip2 target..."
	rustup target add $(TARGET)
	@echo "Checking for wasmtime..."
	@test -f $(WASMTIME) || (echo "Please install wasmtime: https://wasmtime.dev/" && exit 1)
	@echo "Found wasmtime at $(WASMTIME)"

build:
	@echo "Building for $(TARGET)..."
	cargo build --release --target $(TARGET)

run: build
	@echo "Running WASM component with wasmtime..."
	@echo "Using S3_BUCKET=$(S3_BUCKET) and S3_KEY=$(S3_KEY)"
	$(WASMTIME) run \
		--wasi http \
		--env S3_BUCKET=$(S3_BUCKET) \
		--env S3_KEY=$(S3_KEY) \
		--env AWS_REGION=$(AWS_REGION) \
		--env AWS_ACCESS_KEY_ID=$(AWS_ACCESS_KEY_ID) \
		--env AWS_SECRET_ACCESS_KEY=$(AWS_SECRET_ACCESS_KEY) \
		--env AWS_SESSION_TOKEN=$(AWS_SESSION_TOKEN) \
		--dir=. \
		$(WASM_FILE)

clean:
	cargo clean
	@echo "Clean complete"
