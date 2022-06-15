use serde::{Deserialize, Serialize};

use crate::settings::Settings;

pub trait ContextSpec {
    type Configuration;
    fn new(settings: Self::Configuration) -> Self;
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Context {
    pub settings: Settings,
}

impl Context {
    pub fn new(settings: Settings) -> Self {
        Self {
            settings
        }
    }
}