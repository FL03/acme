/*
    Appellation: event_type <module>
    Created At: 2026.02.07:20:28:21
    Contrib: @FL03
*/

pub trait EventType {
    private! {}

    fn name() -> &'static str;
}
