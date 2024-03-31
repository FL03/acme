/*
    Appellation: tensor <test>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#![cfg(test)]
extern crate acme_tensor as acme;

use acme::prelude::{IntoShape, Tensor};

#[test]
fn test_tensor() {
    let shape = (2, 2);
    let a = Tensor::<f64>::ones(shape);
    let b = a.zeros_like();

    assert_ne!(a.id(), b.id());
    assert_eq!(a.shape(), b.shape());
    assert_eq!(a.size(), b.size());
    assert_eq!(a.stride(), b.stride());
}

#[test]
fn test_index() {
    let shape = (2, 3).into_shape();
    let n = shape.size();
    let a = Tensor::<f64>::linspace(0f64, n as f64, n)
        .reshape(shape)
        .unwrap();

    assert_eq!(a[[0, 0]], 0f64);
    assert_eq!(a[[0, 1]], 1f64);
    assert_eq!(a[[1, 2]], 5f64);
}

#[test]
fn test_higher_dim() {
    let shape = (2, 2, 2, 2);
    let a = Tensor::<f64>::ones(shape);
    let b = a.zeros_like();

    assert_ne!(a.id(), b.id());
    assert_eq!(a.shape(), b.shape());
    assert_eq!(a.size(), b.size());
    assert_eq!(a.stride(), b.stride());
    assert_eq!(a.stride().len(), 4);
}
