use serde::{Deserialize, Serialize};

pub trait LoggerSpec {
    type Level;
    type Settings;

    fn setup(&self, settings: Self::Settings) -> Self;
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Logger {
    pub level: String,
}

