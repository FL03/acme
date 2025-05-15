# acme

[![crates.io](https://img.shields.io/crates/v/acme?logo=rust&style=for-the-badge)](https://crates.io/crates/acme)
[![docs.rs](https://img.shields.io/docsrs/acme?style=for-the-badge&logo=rust)](https://docs.rs/acme)
![license](https://img.shields.io/crates/l/acme?logo=rust&style=for-the-badge)

***

_**Warning: the project is currently in the early stages of development and not yet suitable for production use-cases**_

`acme` is an automatic content management engine 

## Features

- [x] Feature 1

## Getting Started

### Building from the source

Start by cloning the repository:

```bash
git clone https://github.com/FL03/acme.git
```

Then, navigate to the project directory:

```bash
cd acme
```

#### _cargo_

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
