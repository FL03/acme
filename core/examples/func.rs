/*
    Appellation: simple <example>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
extern crate acme_core as acme;

use acme::prelude::BoxResult;

fn main() -> BoxResult {
    sample();
    Ok(())
}

#[macro_use]
mod macros {
    macro_rules! autodiff {
        (eval $f:expr) => {
            $f()
        };
        (grad $df:expr) => {
            $df()
        };
    }

    macro_rules! func {
        ($f:expr) => {
            $f
        };
        (eval $f:expr, ($($xs:literal),*)) => {
            $f($($xs),*)
        }

    }
}

fn sample() {
    let f = func!(|x, y, z| z * (x + y));
    println!("{:?}", func!(eval f, (1.0, 2.0, 3.0)));
}
