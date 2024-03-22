/*
    Appellation: kinds <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use strum::{Display, EnumCount, EnumIs, EnumIter, VariantNames};

#[cfg_attr(
    feature = "serde",
    derive(Deserialize, Serialize,),
    serde(rename_all = "lowercase", untagged)
)]
#[derive(
    Clone,
    Copy,
    Debug,
    Default,
    Display,
    EnumCount,
    EnumIs,
    EnumIter,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    VariantNames,
)]
#[repr(u8)]
#[strum(serialize_all = "lowercase")]
pub enum UnaryOp {
    #[default]
    Abs,
    Cos,
    Cosh,
    Exp,
    Floor,
    #[cfg_attr(
        feature = "serde",
        serde(alias = "inverse", alias = "recip", alias = "reciprocal")
    )]
    Inv,
    Ln,
    Neg,
    Sin,
    Sinh,
    Sqrt,
    Square,
    Tan,
    Tanh,
}

impl UnaryOp {
    pub fn differentiable(&self) -> bool {
        match self {
            UnaryOp::Floor | UnaryOp::Inv => false,
            _ => true,
        }
    }
}