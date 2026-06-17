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
