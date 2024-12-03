/*
    Appellation: acme <library>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # acme
//!
//! rsdiff aims to be a complete autodifferentiation solution for Rust.
#![cfg_attr(not(feature = "std"), no_std)]
#![crate_name = "acme"]

#[cfg(feature = "alloc")]
extern crate alloc;


pub mod prelude {}
