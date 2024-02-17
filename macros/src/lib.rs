/*
    Appellation: acme-macros <library>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # acme-macros
//!
//!
extern crate proc_macro;
use proc_macro::TokenStream;
use proc_macro2::TokenStream as Ts;
use quote::quote;
use syn::{parse_macro_input, Expr};

pub(crate) mod ast;
pub(crate) mod cmp;

pub(crate) mod autodiff;
pub(crate) mod gradient;
pub(crate) mod graph;
pub(crate) mod partial;

use partial::Partial;

#[proc_macro_attribute]
pub fn show_streams(attr: TokenStream, item: TokenStream) -> TokenStream {
    println!("attr: \"{}\"", attr.to_string());
    println!("item: \"{}\"", item.to_string());
    item
}

#[proc_macro]
pub fn compute(input: TokenStream) -> TokenStream {
    use graph::Context;
    // Parse the input expression into a syntax tree
    let expr = parse_macro_input!(input as Expr);

    // Build a computational graph representing the expression
    let mut graph = Context::new();
    graph.traverse(&expr);

    // Generate code to compute gradients and return as a HashMap
    let grad = graph.backward();
    let grads = grad
        .into_iter()
        .map(|(k, v)| {
            let k = k.index();
            quote! { (#k, #v) }
        })
        .collect::<Vec<_>>();
    quote! { [#(#grads),*] }.into()
}

#[proc_macro]
pub fn grad(input: TokenStream) -> TokenStream {
    // Parse the input expression into a syntax tree
    let expr = parse_macro_input!(input as Expr);

    // Generate code to compute the gradient
    let result = gradient::compute_grad(&expr);

    // Return the generated code as a token stream
    TokenStream::from(result)
}

#[proc_macro]
pub fn partial(input: TokenStream) -> TokenStream {
    // Parse the input token stream into a structured syntax tree
    let partial = parse_macro_input!(input as Partial);

    // Generate code to perform partial differentiation
    let result = partial::generate_partial(&partial);

    // Return the generated code as a token stream
    TokenStream::from(result)
}
