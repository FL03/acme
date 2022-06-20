use config::{Config, ConfigError, Environment, File};
use glob::glob;
use serde::{Deserialize, Serialize};

use crate::actors::{Logger, Server};

pub trait Configurator {
    type Application;

    fn new(pattern: String) -> Result<Self, ConfigError> where Self: Sized;
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Settings {
    pub logger: Logger,
    pub server: Server,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let mut builder = Config::builder();

        builder = builder
            .set_default("logger.level", "info")?
            .set_default("server.port", 8888)?;

        builder = builder.add_source(glob("**/*.config.*")
            .unwrap()
            .map(|path| File::from(path.unwrap()).required(false))
            .collect::<Vec<_>>()
        );

        builder = builder.add_source(Environment::default().separator("__"));

        if let Ok(port) = std::env::var("PORT") {
            builder = builder
                .set_override("server.port", port)?;
        }
        builder.build()?.try_deserialize()
    }
}

impl std::fmt::Display for Settings {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Message: Successfully setup the application")
    }
}