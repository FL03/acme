/*
    Appellation: arithmetic <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::{BinOp, BinaryAssignOp};
use crate::ops::{Evaluator, OpKind, Operator, Params};
use num::traits::{NumOps, Pow};

macro_rules! impl_binary_op {
    // ($($args:tt),*) => {
    //     impl_binary_op!(@loop $($args),*);

    // };
    ($(($operand:ident, $($p:ident)::*.$op:ident)),*) => {
        $(
            impl_binary_op!(@loop $operand, $($p)::*.$op);
        )*
    };
    ($operand:ident, $($p:ident)::*.$op:ident) => {
        impl_binary_op!(@loop $operand, $($p)::*.$op);
    };
    (std $(($op:ident, $bound:ident, $operator:tt)),*) => {
        $(
            impl_binary_op!(@loop $op, core::ops::$bound, $operator);
        )*
    };
    (@loop $operand:ident, $($p:ident)::*.$op:ident) => {
        operator!($operand<Binary>, $op);
        impl_evaluator!($operand, $($p)::*.$op);

        impl<A, B, C> BinOp<A, B> for $operand
        where
            A: $($p)::*<B, Output = C>,
        {
            type Output = C;

            fn eval(&self, lhs: A, rhs: B) -> Self::Output {
                $($p)::*::$op(lhs, rhs)
            }
        }
    };
    (@loop $(($op:ident, $($p:ident)::*, $operator:tt)),*) => {
        $(
            impl_binary_op!($op, $($p)::*, $operator);
        )*

    };
    (@loop $operand:ident, $($p:ident)::*, $op:tt) => {
        operator!($operand, Binary);

        impl<A, B, C> BinOp<A, B> for $operand
        where
            A: $($p)::*<B, Output = C>,
        {
            type Output = C;

            fn eval(&self, lhs: A, rhs: B) -> Self::Output {
                lhs $op rhs
            }
        }
    };
}

macro_rules! assign_op {
    ($(($operand:ident, $($p:ident)::*, $op:tt)),*) => {
        $(assign_op!(@loop $operand, $($p)::*, $op);)*
    };
    ($operand:ident, $($p:ident)::*, $op:tt) => {
        assign_op!(@impl $operand, $($p)::*, $op);
    };
    (@impl $operand:ident, $($p:ident)::*, $op:tt) => {
        operator!($operand<Binary>);

        impl<A, B> BinOp<A, B> for $operand
        where
            A: Copy + $($p)::*<B>,
        {
            type Output = A;

            fn eval(&self, mut lhs: A, rhs: B) -> Self::Output {
                lhs $op rhs;
                lhs
            }
        }

        impl<A, B> BinaryAssignOp<A, B> for $operand
        where
            A: $($p)::*<B>,
        {
            fn eval(&self, mut lhs: A, rhs: B) {
                lhs $op rhs;
            }
        }
    };
}

macro_rules! impl_evaluator {
    ($(($operand:ident, $($p:ident)::*.$call:ident)),*) => {
        $(
            impl_evaluator!(@loop $operand, $($p)::*.$call);
        )*
    };
    ($operand:ident, $($p:ident)::*.$call:ident) => {
        impl_evaluator!(@loop $operand, $($p)::*.$call);
    };
    (@loop $operand:ident, $($p:ident)::*.$call:ident) => {
        impl<P, A, B, C> Evaluator<P> for $operand
        where
            A: $($p)::*<B, Output = C>,
            P: $crate::ops::Params<Pattern = (A, B)>
        {
            type Output = C;

            fn eval(&self, args: P) -> Self::Output {
                let (lhs, rhs) = args.into_pattern();
                $($p)::*::$call(lhs, rhs)
            }
        }
    };
}

impl_binary_op!(
    (Addition, core::ops::Add.add),
    (Division, core::ops::Div.div),
    (Multiplication, core::ops::Mul.mul),
    (Remainder, core::ops::Rem.rem),
    (Subtraction, core::ops::Sub.sub),
    (Power, num::traits::Pow.pow)
);

assign_op!(AddAssign, core::ops::AddAssign, +=);

// impl_binary_op!(
//     (BitAnd, BitAnd.
//     (BitOr, BitOr, |),
//     (BitXor, BitXor, &|),
//     (Shl, Shl, <<),
//     (Shr, Shr, >>)
// );

operator_group!(Arithmetic<Binary> {
    Add(Addition): add,
    Div(Division): div,
    Mul(Multiplication): mul,
    Pow(Power): pow,
    Rem(Remainder): rem,
    Sub(Subtraction): sub
});

impl Arithmetic {
    pub fn new(op: Arithmetic) -> Self {
        op
    }

    pub fn is_commutative(&self) -> bool {
        match self {
            Arithmetic::Add(_) | Arithmetic::Mul(_) => true,
            _ => false,
        }
    }
}

impl Default for Arithmetic {
    fn default() -> Self {
        Arithmetic::add()
    }
}

impl<P, A, B, C> Evaluator<P> for Arithmetic
where
    A: NumOps<B, C> + Pow<B, Output = C>,
    P: Params<Pattern = (A, B)>,
{
    type Output = C;

    fn eval(&self, args: P) -> Self::Output {
        match self {
            Arithmetic::Add(op) => Evaluator::eval(op, args),
            Arithmetic::Div(op) => Evaluator::eval(op, args),
            Arithmetic::Mul(op) => Evaluator::eval(op, args),
            Arithmetic::Pow(op) => Evaluator::eval(op, args),
            Arithmetic::Rem(op) => Evaluator::eval(op, args),
            Arithmetic::Sub(op) => Evaluator::eval(op, args),
        }
    }
}
