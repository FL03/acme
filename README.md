# acme

## About

Acme advances the scsys crate by carefully implementing a number of useful networking utilities designed to eventually
mimic libraries like Python's FastAPI, enabling developers to quickly spin up cloud-native applications written in Rust. 

## Developers

### Contributors

#### Design Guidelines

Generally, all crates intended for use within Simplicity contain passive modules* actors, components, core, and data;
reserving the next layer for developers to clearly outline the publicity of each submodule.


***_passive modules_** are invisible to the public, largely existing for cleanliness

### _Building from the Source_

#### Clone the repository

    git clone https://gitlab.com/FL03/acme

#### Crate

    cargo build --color always --release --workspace
    cargo test --all-features --color always --release --verbose --workspace
