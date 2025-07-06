# acme

[![crates.io](https://img.shields.io/crates/v/acme?logo=rust&style=for-the-badge)](https://crates.io/crates/acme)
[![docs.rs](https://img.shields.io/docsrs/acme?style=for-the-badge&logo=rust)](https://docs.rs/acme)
![creates.io (License)](https://img.shields.io/crates/l/acme?logo=rust&style=for-the-badge)

***

_**Warning: the project is currently in the early stages of development and not yet suitable for production use-cases**_

Welcome to `acme`! This crate focuses on aggregating information from various sources and processing it in a secure, robust, and efficient manner. The project is designed to be modular and extensible, allowing for easy integration of new data sources and sinks.

This crate is one of the first extensions of the `eryon` framework, a Rust-based computational systems designed for high-performance data processing using topology with a hint of music-theory. The `acme` crate aims to provide a flexible and powerful platform for building data processing pipelines, leveraging the capabilities of the `eryon` framework.

## Features

- [`engine`](https://docs.rs/acme-engine/latest/acme_engine/): The core engine that orchestrates the data processing pipeline.
- [`core`](https://docs.rs/acme-core/latest/acme_core/): Provides the foundational components and utilities for the ACME platform.

### _Dependency-related Features_

In addition to the core features of the crate and the various _environmental_ features, the crate integrates with several external crates for enhanced functionality:

- `serde`: Enables serialization and deserialization of data.
- `tracing`: Provides a framework for instrumenting Rust programs to collect structured, contextual, and async-aware diagnostics.
- `tokio`: An asynchronous runtime for the Rust programming language, enabling concurrent programming.

## Getting Started

Before you start using `acme`, ensure you have the following prerequisites:

- [Git](https://git-scm.com/) for cloning the repository
- Rust installed on your machine. You can install it from [rustup.rs](https://rustup.rs/).
  - A compatible version of the Rust toolchain (preferably the latest stable version).
  - [Cargo](https://doc.rust-lang.org/cargo/) for building and managing Rust projects.
  - **Optionally**, install the [cargo-binstall](https://github.com/cargo-bins/cargo-binstall) utility for streamlined installation of Rust binaries.

For more information on how to setup your environment, refer to the [QUICKSTART](https://github.com/acme/blob/main/QUICKSTART.md) guide.

### From the source

Start by cloning the repository:

```bash
git clone https://github.com/FL03/acme.git
```

Then, navigate to the project directory:

```bash
cd acme
```

#### Using cargo

To build the project, you can use the following command:

```bash
cargo build --all-features --workspace [--release]
```

To run the tests, you can use the following command:

```bash
cargo test --all-features --workspace
```

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies.acme]
features = []
version = "0.4.0"
```

### Examples

#### _Basic Usage_

```rust
    use acme::engine::Engine;

    fn main() -> Result<(), Box<dyn std::error::Error>> {
        tracing_subscriber::fmt()
          .with_line_number(false)
          .with_max_level(tracing::Level::TRACE)
          .with_thread_ids(false)
          .with_target(true)
          .compact()
          .init();
        // verify the tracing layers are initialized
        tracing::info!("Welcome to {name}", name = acme);
        // initialize a new instance of the engine
        let acme = Engine::new()?;
        // finish
        Ok(())
    }
```

## Contributing

[CONTRIBUTORS](https://github.com/acme/blob/main/CONTRIBUTORS.md)

## License

This project is [licensed](https://github.com/FL03/acme/blob/main/LICENSE) under the [Apache-2.0](https://opensource.org/license/apache-2-0).
