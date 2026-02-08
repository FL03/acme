# acme

[![crates.io](https://img.shields.io/crates/v/acme?logo=rust&style=for-the-badge)](https://crates.io/crates/acme)
[![docs.rs](https://img.shields.io/docsrs/acme?style=for-the-badge&logo=rust)](https://docs.rs/acme)
[![GitHub License](https://img.shields.io/github/license/FL03/template-nextjs-app?style=for-the-badge&logo=github)](LICENSE)

***

***Warning: the project is currently in the early stages of development and not yet suitable for production use-cases***

Welcome to `acme`! The `acme` protocol seeks to establish an automated context management engine used to guide and inform dynamic processes at scale. This is particularly useful for agentic systems, where individual agents often lack the ability to naturally inherit context and working knowledge from other agents, leading to inefficiencies and a lack of coordination. By providing a structured way to manage and share context, `acme` aims to enhance the capabilities of agentic systems, enabling them to operate more effectively and collaboratively.

## Features

- [`engine`](https://docs.rs/acme-engine/latest/acme_engine/): The core engine that orchestrates the data processing pipeline.
- [`core`](https://docs.rs/acme-core/latest/acme_core/): Provides the foundational components and utilities for the ACME platform.

### *Environmental Features*

- `alloc`: Provides support for heap allocation, enabling the use of dynamic data structures.
- `std`: The Rust standard library, providing essential types and functionalities for Rust programming.
- `nightly`: Enables features that require the Rust nightly compiler, allowing for the use of unstable features and optimizations.
- `wasi`: Enables support for WebAssembly System Interface (WASI), allowing the crate to run in WebAssembly environments.
- `wasm`: Enables support for WebAssembly, allowing the crate to be compiled to and run in WebAssembly environments.

### *Dependency-related Features*

In addition to the core features of the crate and the various *environmental* features, the crate integrates with several external crates for enhanced functionality:

- `serde`: Enables serialization and deserialization of data.
- `tracing`: Provides a framework for instrumenting Rust programs to collect structured, contextual, and async-aware diagnostics.
- `wasm_bindgen`: Enables `wasm-bindgen`, thus allowing interoperability between Rust and JavaScript. Allows the crate to be used in web applications.

## Getting Started

Add this to your `Cargo.toml`:

```toml
[dependencies.acme]
features = []
version = "0.4.x"
```

### Basic Usage

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

To get started with developing with `acme`, visit the [QUICKSTART](QUICKSTART.md) guide for a step-by-step introduction to using the crate in your projects and make sure to reference the [CONTRIBUTING](CONTRIBUTING.md) guidelines for information on how to contribute to the project.

## License

This project is [licensed](LICENSE) under the [Apache-2.0](https://opensource.org/license/apache-2-0).
