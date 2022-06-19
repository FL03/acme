use config::{Config, ConfigBuilder, ConfigError, builder::DefaultState};

pub trait ConfiguratorSpec {
    type Actor;
    type Context;
    type Container;
    type Data;

    fn builder(pattern: String) -> Result<ConfigBuilder<DefaultState>, ConfigError>;
}

pub trait ControllerSpec {
    type Actor;
    type Context;
}