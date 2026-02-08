/// The [`RawEngine`] trait defines the base interface for all implemented engines
pub trait RawEngine {
    type Ctx;
    // prohibit external implementations
    private! {}
}
