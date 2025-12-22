.PHONY: setup build run spin clean help publish k8s-secret k8s-deploy k8s-logs

TARGET := wasm32-wasip2
BINARY_NAME := s3-wasm
WASM_FILE := target/$(TARGET)/release/$(BINARY_NAME).wasm

# Try to find wasmtime in common locations
WASMTIME := $(shell which wasmtime 2>/dev/null || echo /opt/homebrew/opt/wasmtime/bin/wasmtime)
# And spin
SPIN := $(shell which spin 2>/dev/null || echo /opt/homebrew/opt/spin/bin/spin)

# Default S3 configuration (can be overridden)
S3_BUCKET ?= wasi-s3-dm
S3_KEY ?= data.csv

# OCI registry configuration
OCI_IMAGE ?= ttl.sh/polars-s3-wasi:24h

help:
	@echo "Available targets:"
	@echo "  setup      - Install wasip2 target and wasmtime"
	@echo "  build      - Build the WASM binary"
	@echo "  run        - Run the WASM binary with wasmtime"
	@echo "  spin       - Run the WASM binary with Spin"
	@echo "  publish    - Build and push OCI image to registry"
	@echo "  clean      - Clean build artifacts"
	@echo "  k8s-secret - Create/update AWS credentials secret in K8s"
	@echo "  k8s-deploy - Deploy the job to Kubernetes"
	@echo "  k8s-logs   - Fetch logs from the job"
	@echo ""
	@echo "Environment variables:"
	@echo "  S3_BUCKET  - S3 bucket name (default: wasi-s3-dm)"
	@echo "  S3_KEY     - S3 object key (default: data.csv)"
	@echo "  OCI_IMAGE  - OCI image reference (default: ttl.sh/polars-s3-wasi:24h)"

setup:
	@echo "Installing" $(TARGET) "target..."
	rustup target add $(TARGET)
	@echo "Checking for wasmtime..."
	@test -f $(WASMTIME) || (echo "Please install wasmtime: https://wasmtime.dev/" && exit 1)
	@echo "Found wasmtime at $(WASMTIME)"
	@echo "Checking for spin..."
	@test -f $(SPIN) || (echo "Please install spin: https://spinframework.dev/v3/install" && exit 1)
	@echo "Found spin at $(SPIN)"
	@echo "Setup complete."

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

spin: build
	@echo "Running WASM component with Spin..."
	$(SPIN) up \
		--variable s3_bucket="$(S3_BUCKET)" \
		--variable s3_key="$(S3_KEY)" \
		--variable aws_region="$(AWS_REGION)" \
		--variable aws_access_key_id="$(AWS_ACCESS_KEY_ID)" \
		--variable aws_secret_access_key="$(AWS_SECRET_ACCESS_KEY)" \
		--variable aws_session_token="$(AWS_SESSION_TOKEN)"

publish: build
	@echo "Publishing Spin app to $(OCI_IMAGE)..."
	$(SPIN) registry push $(OCI_IMAGE)

recreate-job:
	kubectl delete jobs.batch polars-s3-wasi; kubectl apply -f k8s-job.yaml

clean:
	cargo clean
	@echo "Clean complete"
