/*
    Appellation: raw_event <module>
    Created At: 2026.02.07:20:30:02
    Contrib: @FL03
*/
use super::EventType;

/// [`RawEvent`] defines the base interface for all compatible events within the `acme`
/// ecosystem.
pub trait RawEvent {
    type Kind: EventType;
}
