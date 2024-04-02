/*
    Appellation: dtype <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use acme::prelude::TypeError;
use std::any::TypeId;

pub trait TypeOf {
    fn of<T>(_value: &T) -> Result<Self, TypeError>
    where
        T: 'static,
        Self: Sized;
}

pub enum DType {
    Float(Float),
    Integer(Integer),
}

impl DType {
    pub fn of<T>(val: &T) -> Result<Self, TypeError>
    where
        T: 'static,
    {
        if let Ok(float) = Float::from_type(val) {
            Ok(DType::Float(float))
        } else if let Ok(integer) = Integer::from_type(val) {
            Ok(DType::Integer(integer))
        } else {
            Err(TypeError::InvalidType)
        }
    }
}

pub enum Float {
    F32,
    F64,
}

impl Float {
    pub fn from_type<T>(_value: &T) -> Result<Self, TypeError>
    where
        T: 'static,
    {
        if TypeId::of::<T>() == TypeId::of::<f32>() {
            Ok(Float::F32)
        } else if TypeId::of::<T>() == TypeId::of::<f64>() {
            Ok(Float::F64)
        } else {
            Err(TypeError::InvalidType)
        }
    }
}

impl From<f32> for Float {
    fn from(_: f32) -> Self {
        Float::F32
    }
}

impl From<f64> for Float {
    fn from(_: f64) -> Self {
        Float::F64
    }
}

pub struct Integer {
    pub bits: NumBits,
    pub signed: bool,
}

impl Integer {
    pub fn from_type<T>(_value: &T) -> Result<Self, TypeError>
    where
        T: 'static,
    {
        if TypeId::of::<T>() == TypeId::of::<i8>() {
            Ok(Integer {
                bits: NumBits::B8,
                signed: true,
            })
        } else if TypeId::of::<T>() == TypeId::of::<i16>() {
            Ok(Integer {
                bits: NumBits::B16,
                signed: true,
            })
        } else if TypeId::of::<T>() == TypeId::of::<i32>() {
            Ok(Integer {
                bits: NumBits::B32,
                signed: true,
            })
        } else if TypeId::of::<T>() == TypeId::of::<i64>() {
            Ok(Integer {
                bits: NumBits::B64,
                signed: true,
            })
        } else if TypeId::of::<T>() == TypeId::of::<i128>() {
            Ok(Integer {
                bits: NumBits::B128,
                signed: true,
            })
        } else if TypeId::of::<T>() == TypeId::of::<u8>() {
            Ok(Integer {
                bits: NumBits::B8,
                signed: false,
            })
        } else if TypeId::of::<T>() == TypeId::of::<u16>() {
            Ok(Integer {
                bits: NumBits::B16,
                signed: false,
            })
        } else if TypeId::of::<T>() == TypeId::of::<u32>() {
            Ok(Integer {
                bits: NumBits::B32,
                signed: false,
            })
        } else if TypeId::of::<T>() == TypeId::of::<u64>() {
            Ok(Integer {
                bits: NumBits::B64,
                signed: false,
            })
        } else if TypeId::of::<T>() == TypeId::of::<u128>() {
            Ok(Integer {
                bits: NumBits::B128,
                signed: false,
            })
        } else {
            Err(TypeError::InvalidType)
        }
    }
}

#[repr(u8)]
pub enum NumBits {
    B8 = 8,
    B16 = 16,
    B32 = 32,
    B64 = 64,
    B128 = 128,
    BSize,
}

macro_rules! impl_from_bits {
    ($v:ident, $t:ty) => {
        impl From<$t> for NumBits {
            fn from(_: $t) -> Self {
                NumBits::$v
            }
        }
    };
    ($v:ident: [$($t:ty),*]) => {
        $(
            impl_from_bits!($v, $t);
        )*
    };
}

impl_from_bits!(B8: [u8, i8]);
impl_from_bits!(B16: [u16, i16]);
impl_from_bits!(B32: [u32, i32]);
impl_from_bits!(B64: [u64, i64]);
impl_from_bits!(B128: [u128, i128]);
impl_from_bits!(BSize: [usize, isize]);