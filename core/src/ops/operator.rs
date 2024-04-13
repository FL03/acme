/*
    Appellation: operator <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::kinds::OpKind;

pub trait Operator {
    fn kind(&self) -> OpKind;

    fn name(&self) -> &str;
}

#[allow(dead_code)]
pub(crate) struct Expr {
    kind: OpKind,
    name: String,
}

pub trait Args {
    type Pattern;

    fn args(self) -> Self::Pattern;
}

macro_rules! args_impl {
    (@loop $($n:ident),*) => {
        impl<$($n),*> Args for ($($n),*) {
            type Pattern = ($($n),*);

            fn args(self) -> Self::Pattern {
                self
            }
        }
    };
}

macro_rules! impl_args {
    ($($n:ident),*) => {
        impl_args!(@loop $($n),*);
    };
    (alt: $(($($n:ident),*)),*) => {
        $(
            impl_args!(@loop $($n),*);
        )*
    };
    (@loop $($n:ident),*) => {
        impl<$($n),*> Args for ($($n),*) {
            type Pattern = ($($n),*);

            fn args(self) -> Self::Pattern {
                self
            }
        }
    };
    (@loop $($n:ident),*) => {
        $($n),*
    };
}
impl_args!(A, B);
impl_args!(A, B, C);

// impl Args for () {
//     type Pattern = ();

//     fn args(self) -> Self::Pattern {
//         ()
//     }
// }

// impl<A> Args for (A,) {
//     type Pattern = (A,);

//     fn args(self) -> Self::Pattern {
//         self
//     }
// }

// impl<A, B> Args for (A, B) {
//     type Pattern = (A, B);

//     fn args(self) -> Self::Pattern {
//         self
//     }
// }

pub trait Evaluator<Args> {
    type Output;

    fn eval(&self, args: Args) -> Self::Output;
}

pub trait Operand {
    type Args: Args;
    type Output;

    fn eval(&self, args: Self::Args) -> Self::Output;

    fn kind(&self) -> OpKind;

    fn name(&self) -> &str;
}

pub trait Differentiable: Operand {
    type Jacobian;

    fn grad(&self, args: Self::Args) -> Self::Jacobian;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_args() {
        let args = (0f64, 0f32);
        let pattern = args.args();
        assert_eq!(pattern, args);
        let args = (0f64, 0f32, 0usize);
        let pattern = args.args();
        assert_eq!(pattern, args);
    }
}
