/*
    Appellation: handle <module>
    Contrib: @FL03
*/
use crate::events::RawEvent;
use crate::traits::RawContext;

pub trait Handler<E>
where
    E: RawEvent,
{
    type Ctx: RawContext;

    fn handle(&self, ctx: &mut Self::Ctx, event: &E) -> Result<(), crate::Error>;
}
