use config;

#[derive(serde::Deserialize)]
pub struct Settings {
    pub application_port: u16,
    pub database: DatabaseSettings,
}

#[derive(serde::Deserialize)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: String,
    pub port: u16,
    pub host: String,
    pub database_name: String,
}

impl DatabaseSettings {
    /// Returns the db connection strig previously read from the config file.
    pub fn connection_string(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.database_name
        )
    }
}

/// Creates a Settings object as a Result if it managed to deserialize
/// it from a config file.
///
/// This function uses the config crate to create a default config object
/// and append a config file to it from where the deserialization will be
/// attempted.
///
/// return: Result<Settings, config::ConfigError>
///
/// usage: configuration::get_configuration.unwrap()
pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    let mut settings = config::Config::default();

    settings.merge(config::File::with_name("configuration"))?;

    settings.try_into()
}
