# acme

[![crates.io](https://img.shields.io/crates/v/acme.svg)](https://crates.io/crates/acme)
[![docs.rs](https://docs.rs/acme/badge.svg)](https://docs.rs/acme)
[![license](https://img.shields.io/crates/l/acme.svg)](https://crates.io/crates/acme)

[![clippy](https://github.com/FL03/acme/actions/workflows/clippy.yml/badge.svg)](https://github.com/FL03/acme/actions/workflows/clippy.yml)
[![rust](https://github.com/FL03/acme/actions/workflows/rust.yml/badge.svg)](https://github.com/FL03/acme/actions/workflows/rust.yml)

***

_**The library is currently in the early stages of development and is not yet ready for production use.**_

rsdiff aims to be a complete autodifferentiation solution for Rust.

## Features

- [x] Feature 1

## Getting Started

### Building from the source

Start by cloning the repository

```bash
git clone https://github.com/FL03/acme.git
cd acme
```

#### _Building the project_

```bash
cargo build --all-features -r -v --workspace
```

#### _Running tests_

```bash
cargo test --all-features -r -v --workspace
```

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies.acme]
features = []
version = "0.1.0"
```

### Examples

#### _Basic Usage_

```rust
    extern crate acme;

    fn main() -> Result<(), Box<dyn std::error::Error>> {
        tracing_subscriber::fmt::init();
        tracing::info!("Welcome to {name}", name = acme);


        Ok(())
    }
```

## Contributing

Pull requests are welcome. For major changes, please open an issue first
to discuss what you would like to change.

Please make sure to update tests as appropriate.
