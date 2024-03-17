/*
    Appellation: arith <test>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#![cfg(test)]
extern crate acme_tensor as tensor;

use tensor::TensorBase;

#[test]
fn test_add() {
    let shape = (2, 2);
    let a = TensorBase::<f64>::ones(shape);
    let b = TensorBase::<f64>::ones(shape);
    let c = a + &b;

    assert_eq!(c, TensorBase::<f64>::ones(shape) * 2.0);
}

#[test]
fn test_div() {
    let shape = (2, 2);
    let a = TensorBase::<f64>::ones(shape);
    let b = TensorBase::<f64>::ones(shape) * 2.0;
    let c = a / b;

    assert_eq!(c, TensorBase::<f64>::fill(shape, 0.5));
}

#[test]
fn test_mul() {
    let shape = (2, 2);
    let a = TensorBase::<f64>::ones(shape);
    let b = TensorBase::<f64>::ones(shape);
    let c = a * b;

    assert_eq!(c, TensorBase::<f64>::ones(shape));
}

#[test]
fn test_sub() {
    let shape = (2, 2);
    let a = TensorBase::<f64>::ones(shape);
    let b = TensorBase::<f64>::ones(shape);
    let c = a - &b;

    assert_eq!(c, TensorBase::<f64>::zeros(shape));
}

#[test]
fn test_matmul() {
    let a = TensorBase::<f64>::fill((3, 2), 2.0);
    let b = TensorBase::<f64>::ones((2, 3));
    let c = a.matmul(&b);

    assert_eq!(c, TensorBase::<f64>::fill((3, 3), 4.0));
}