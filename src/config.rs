use config::{ConfigError, FileFormat};

pub fn load() -> Result<Settings, ConfigError> {
    let settings = config::Config::builder()
        .add_source(config::File::new("config.yaml", FileFormat::Yaml))
        .build()?;
    settings.try_deserialize::<Settings>()
}

#[derive(serde::Deserialize)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub application_port: u16,
}

#[derive(serde::Deserialize)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: String,
    pub port: u16,
    pub host: String,
    pub database_name: String,
}
