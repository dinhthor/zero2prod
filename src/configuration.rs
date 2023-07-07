#[derive(serde::Deserialize)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub application_port: u16
}

#[derive(serde::Deserialize)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: String,
    pub host: String,
    pub port: u16,
    pub database_name: String
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    // Initialize our configuration reader
    let mut settings = config::Config::default();
    // Add configuration values from a file named `configuration`
    // with the .yaml extension
    settings.merge(config::File::with_name("configuration"))?;
    // Try to convert the configuration values it read into
    // our Settings type
    settings.try_into()
}

impl DatabaseSettings {
    pub fn connection_string(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username,
            self.password,
            self.host,
            self.port,
            self.database_name
        )
    }

    pub fn connection_string_without_db(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}",
            self.username, self.password, self.host, self.port
        )
    }
}

// pub fn get_configuration() -> Result<Settings, config::ConfigError> {
//     let settings = config::Config::builder()
//         .add_source(config::File::new(
//             "configuration.yaml",
//             config::FileFormat::Yaml,
//         ))
//         .build()?;
//     settings.try_deserialize::<Settings>()
// }