# S3 WASM CSV Downloader

A Rust application that downloads CSV files from AWS S3, compiled to WebAssembly (WASI Preview 2) and runs with wasmtime.

This project demonstrates how to use the AWS SDK for Rust in a WASM environment by leveraging the `aws-smithy-wasm` crate, which provides WASI-compatible HTTP client and async runtime implementations.

## Prerequisites

- Rust toolchain (with `wasm32-wasip2` target)
- wasmtime runtime
- AWS credentials

## Setup

Install the required target and verify wasmtime is installed:

```bash
make setup
```

## Configuration

The application uses environment variables for configuration:

- `S3_BUCKET` - S3 bucket name (default: "my-bucket")
- `S3_KEY` - S3 object key (default: "data.csv")
- `AWS_REGION` - AWS region (e.g., "us-east-1")
- `AWS_ACCESS_KEY_ID` - AWS access key
- `AWS_SECRET_ACCESS_KEY` - AWS secret key
- `AWS_SESSION_TOKEN` - AWS session token (optional, for temporary credentials)

## Usage

### Build the WASM binary

```bash
make build
```

This compiles the Rust code to `wasm32-wasip2` target, creating a WASM component that can run in any WASI Preview 2 compatible runtime.

### Run the WASM binary

```bash
AWS_REGION=us-east-1 \
AWS_ACCESS_KEY_ID=your-key \
AWS_SECRET_ACCESS_KEY=your-secret \
make run S3_BUCKET=my-bucket S3_KEY=data.csv
```

Or using the Makefile with environment variables:

```bash
make run S3_BUCKET=my-bucket S3_KEY=data.csv
```

### Run directly with wasmtime

```bash
wasmtime run \
  --env S3_BUCKET=my-bucket \
  --env S3_KEY=data.csv \
  --env AWS_REGION=us-east-1 \
  --env AWS_ACCESS_KEY_ID=your-key \
  --env AWS_SECRET_ACCESS_KEY=your-secret \
  --dir=. \
  target/wasm32-wasip2/release/s3-wasm.wasm
```

### Clean build artifacts

```bash
make clean
```

## Architecture

This project uses:

- **AWS SDK for Rust** - Official AWS SDK with S3 client
- **aws-smithy-wasm** - Provides WASI-compatible HTTP client and sleep implementations
- **Tokio** - Async runtime for Rust (with `current_thread` flavor for WASM)
- **WASI Preview 2** - WebAssembly System Interface for WASM components
- **wasmtime** - WebAssembly runtime

### How it works

1. **WASI HTTP Client**: The `aws-smithy-wasm` crate provides a `wasi_http_client()` that implements the HTTP client trait using WASI Preview 2's HTTP capabilities.

2. **WASM Sleep Implementation**: For async operations that require sleep/delay, `aws-smithy-wasm::WasmSleep` provides a WASM-compatible implementation.

3. **AWS SDK Integration**: The SDK is configured to use these WASM-compatible implementations instead of the default ones (which depend on OS-level networking).

4. **Environment Variables**: AWS credentials and configuration are loaded from environment variables, which wasmtime passes to the WASM component.

The compiled WASM binary is portable and can run in any WASI Preview 2 compatible runtime with network access.
