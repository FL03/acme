pub struct Logger {
    pub level: String,
}

pub trait LoggerSpec {
    type Level;
    type Settings;

    fn setup(&self, settings: Self::Settings) -> Self;
}