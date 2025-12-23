.PHONY: all benchmark wasmtime python rust clean help build compile

# Paths
WASM_FILE := ../../target/wasm32-wasip2/release/s3-wasm.wasm
CWASM_FILE := ../../target/wasm32-wasip2/release/s3-wasm.cwasm
RUST_BINARY := ./target/aarch64-apple-darwin/release/s3-polars-benchmark
PYTHON_DIR := ../python
PYTHON_VENV := $(PYTHON_DIR)/.venv

# Try to find wasmtime in common locations
WASMTIME := $(shell which wasmtime 2>/dev/null || echo /opt/homebrew/opt/wasmtime/bin/wasmtime)

# Default S3 configuration (can be overridden)
S3_BUCKET ?= wasi-s3-dm
S3_KEY ?= data.csv

# Hyperfine configuration
WARMUP ?= 2
RUNS ?= 10

# Commands - use precompiled .cwasm file to skip JIT compilation
# Note: WASM code expects SPIN_CONFIG_ prefix for environment variables
WASMTIME_CMD := $(WASMTIME) run --wasi http \
	--env SPIN_CONFIG_S3_BUCKET=$(S3_BUCKET) \
	--env SPIN_CONFIG_S3_KEY=$(S3_KEY) \
	--env SPIN_CONFIG_AWS_REGION=$(AWS_REGION) \
	--env SPIN_CONFIG_AWS_ACCESS_KEY_ID=$(AWS_ACCESS_KEY_ID) \
	--env SPIN_CONFIG_AWS_SECRET_ACCESS_KEY=$(AWS_SECRET_ACCESS_KEY) \
	--env SPIN_CONFIG_AWS_SESSION_TOKEN=$(AWS_SESSION_TOKEN) \
	--dir=. --allow-precompiled $(CWASM_FILE)
PYTHON_CMD := $(PYTHON_VENV)/bin/python $(PYTHON_DIR)/main.py
RUST_CMD := $(RUST_BINARY)

help:
	@echo "Benchmark targets:"
	@echo "  all       - Run all benchmarks with hyperfine"
	@echo "  wasmtime  - Benchmark WASM with wasmtime"
	@echo "  python    - Benchmark Python"
	@echo "  rust      - Benchmark native Rust"
	@echo "  build     - Build native Rust binary"
	@echo "  compile   - AOT compile WASM to .cwasm (precompiled)"
	@echo "  clean     - Clean build artifacts"
	@echo ""
	@echo "Environment variables:"
	@echo "  S3_BUCKET - S3 bucket name (default: wasi-s3-dm)"
	@echo "  S3_KEY    - S3 object key (default: data.csv)"
	@echo "  WARMUP    - Hyperfine warmup runs (default: 2)"
	@echo "  RUNS      - Hyperfine benchmark runs (default: 10)"

all: benchmark

# AOT compile the WASM to .cwasm to skip JIT compilation during benchmarks
compile:
	@test -f $(WASM_FILE) || (echo "WASM file not found. Run 'make build' in project root first." && exit 1)
	@echo "Precompiling WASM to .cwasm..."
	$(WASMTIME) compile --wasi http $(WASM_FILE) -o $(CWASM_FILE)

benchmark: compile
	@test -f $(CWASM_FILE) || (echo "Precompiled WASM not found. Run 'make compile' first." && exit 1)
	@test -d $(PYTHON_VENV) || (echo "Python venv not found. Run 'uv sync' in $(PYTHON_DIR) first." && exit 1)
	@test -f $(RUST_BINARY) || $(MAKE) build
	hyperfine \
		--warmup $(WARMUP) \
		--runs $(RUNS) \
		-n "Native Rust" "$(RUST_CMD)" \
		-n "Python" "$(PYTHON_CMD)" \
		-n "WASM (wasmtime)" "$(WASMTIME_CMD)"

wasmtime: compile
	@test -f $(CWASM_FILE) || (echo "Precompiled WASM not found. Run 'make compile' first." && exit 1)
	hyperfine --warmup $(WARMUP) --runs $(RUNS) -n "WASM (wasmtime)" "$(WASMTIME_CMD)"

python:
	@test -d $(PYTHON_VENV) || (echo "Python venv not found. Run 'uv sync' in $(PYTHON_DIR) first." && exit 1)
	hyperfine --warmup $(WARMUP) --runs $(RUNS) -n "Python" "$(PYTHON_CMD)"

rust:
	@test -f $(RUST_BINARY) || $(MAKE) build
	hyperfine --warmup $(WARMUP) --runs $(RUNS) -n "Native Rust" "$(RUST_CMD)"

build:
	cargo build --release

clean:
	cargo clean
	rm -f $(CWASM_FILE)
	@echo "Clean complete"
