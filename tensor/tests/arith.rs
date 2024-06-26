/*
    Appellation: arith <test>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#![cfg(test)]
extern crate acme_tensor as acme;

use acme::prelude::Tensor;

#[test]
fn test_add() {
    let shape = (2, 2);
    let a = Tensor::<f64>::ones(shape);
    let b = Tensor::<f64>::ones(shape);
    let c = &a + &b;

    assert_eq!(c, Tensor::fill(shape, 2_f64));

    let a = Tensor::<f64>::ones(shape);
    let b = a + 1_f64;
    assert_eq!(b, Tensor::fill(shape, 2_f64));
}

#[test]
fn test_div() {
    let shape = (2, 2);
    let a = Tensor::<f64>::ones(shape);
    let b = Tensor::<f64>::fill(shape, 2_f64);
    let c = a / b;

    assert_eq!(c, Tensor::<f64>::fill(shape, 0.5));
}

#[test]
fn test_mul() {
    let shape = (2, 2);
    let a = Tensor::<f64>::ones(shape);
    let b = Tensor::<f64>::ones(shape);
    let c = a * b;

    assert_eq!(c, Tensor::<f64>::ones(shape));
}

#[test]
fn test_sub() {
    let shape = (2, 2);
    let a = Tensor::<f64>::ones(shape);
    let b = Tensor::<f64>::ones(shape);
    let c = a - &b;

    assert_eq!(c, Tensor::<f64>::zeros(shape));
}

#[test]
fn test_trig() {
    let a = Tensor::<f64>::ones((2, 2));
    let b = a.sin();
    let c = a.cos();

    assert_eq!(b[[0, 0]], 1_f64.sin());
    assert_eq!(c[[0, 0]], 1_f64.cos());
}
