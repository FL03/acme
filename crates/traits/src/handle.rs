/*
    Appellation: handle <module>
    Created At: 2026.02.07:20:09:14
    Contrib: @FL03
*/
use crate::Context;

pub trait Handler<E> {
    type Ctx: Context<Space = E>;
    type Output;

    fn handle(&self, ctx: &mut Self::Ctx, event: &E) -> Self::Output;
}
