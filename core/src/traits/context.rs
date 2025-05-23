/*
    Appellation: context <module>
    Contrib: @FL03
*/

/// The base context considered by the `acme` framework.
pub trait RawContext {
    type Item;

    private!();
}

pub trait Context {
    type Ctx: RawContext;
    /// returns an immutable reference to the current context.
    fn ctx(&self) -> Self::Ctx;
    /// returns a mutable reference to the current context
    fn ctx_mut(&mut self) -> &mut Self::Ctx;
    /// replace the current context with the given before returning the previous entry.
    fn replace_ctx(&mut self, ctx: Self::Ctx) -> Self::Ctx;
    /// update the current instance with the given context before returning a mutable reference
    fn set_ctx(&mut self, ctx: Self::Ctx) -> &mut Self;
}

/// [`Contextual`] is a trait for denoting types that are associated with a particular context.
pub trait Contextual {
    type Ctx: RawContext;
    /// returns an immutable reference to the current context.
    fn ctx(&self) -> Self::Ctx;
    /// returns a mutable reference to the current context
    fn ctx_mut(&mut self) -> &mut Self::Ctx;
    /// [`replace`](core::mem::replace) with another and return the previous context
    fn replace_ctx(&mut self, ctx: Self::Ctx) -> Self::Ctx {
        core::mem::replace(self.ctx_mut(), ctx)
    }
    /// update the current context and return a mutable reference to the instance
    fn set_ctx(&mut self, ctx: Self::Ctx) -> &mut Self {
        *self.ctx_mut() = ctx;
        self
    }
    /// [`swap`](core::mem::swap) out the context's of two instances.
    fn swap_ctx(&mut self, other: &mut Self) {
        core::mem::swap(self.ctx_mut(), other.ctx_mut());
    }
    /// [`take`](core::mem::take) the current context and return it, leaving the default
    /// context in its place.
    fn take_ctx(&mut self) -> Self::Ctx
    where
        Self::Ctx: Default,
    {
        core::mem::take(self.ctx_mut())
    }
}
