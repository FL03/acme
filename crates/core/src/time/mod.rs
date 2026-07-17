/*
    Appellation: time <module>
    Created At: 2026.02.07:18:17:35
    Contrib: @FL03
*/
#[doc(inline)]
pub use self::{traits::*, ts::*, utils::*};

mod ts;
pub mod utils;

mod impls {
    mod impl_timestamp;
    mod impl_timestamp_repr;
}

mod traits {
    #[doc(inline)]
    pub use self::now::*;

    mod now;
}

// prelude (local)
pub(crate) mod prelude {
    pub use super::traits::*;
    pub use super::ts::*;
}
