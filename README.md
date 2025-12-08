# S3 WASM CSV Downloader

A Rust application that downloads CSV files from AWS S3, compiled to WebAssembly (WASI Preview 2) and runs with wasmtime.

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