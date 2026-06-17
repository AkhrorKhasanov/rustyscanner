# RustyScanner

A high-performance, asynchronous port scanner written in Rust using Tokio.

## Features
- **Async Execution**: Scans multiple ports concurrently without blocking.
- **Concurrency Control**: Limit the number of simultaneous connections to avoid OS limits.
- **Fast & Lightweight**: Compiled to a single binary with zero dependencies.

## Installation

Ensure you have Rust installed. Then:

```bash
git clone https://github.com/AkhrorKhasanov/rustyscanner.git
cd rustyscanner
cargo build --release
```

## Usage

Run the scanner by providing the target IP and the port range:

```bash
# Basic scan
cargo run -- --ip 127.0.0.1 --range 1-1000

# Scanning a specific target with custom concurrency
cargo run -- --ip scanme.nmap.org --range 20-80 --concurrency 50
