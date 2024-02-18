/*
    Appellation: autodiff <example>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#![allow(dead_code, unused_variables)]
#![feature(fn_traits)]
extern crate acme;

use acme::{autodiff, show_item};
use acme::prelude::sigmoid;

macro_rules! eval {
    ($var:ident: $ex:expr) => {
        println!("Eval: {:?}", $ex);
        println!("Gradient: {:?}", autodiff!($var: $ex));
    }
}

fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let x = 2_f64;
    // samples(x);


    show_item!(acme::prelude::sigmoid::<f64>(x));
    println!("{:?}", sigmoid::<f64>(2_f64));
    

    Ok(())
}

fn samples(x: f64) {
    eval!(x: x.tan());

    eval!(x: x.sin());

    eval!(x: x.cos().sin());
}