/*
    Appellation: context <module>
    Created At: 2026.02.07:20:11:13
    Contrib: @FL03
*/

/// [`Context`] is a base trait that allows for the definition of a context that can be used in
/// various components and handlers. It serves as a marker trait to indicate that a type can
/// be used as a context within the framework, providing a way to associate a specific "space"
/// or environment with the context.
pub trait Context {
    type Space;
}
