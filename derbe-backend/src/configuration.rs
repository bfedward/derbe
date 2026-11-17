use config::{Config, ConfigError, Environment, File, FileFormat};
use secrecy::{ExposeSecret, SecretBox};

#[derive(serde::Deserialize)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub application: ApplicationSettings,
    pub auth: AuthSettings,
}

#[derive(serde::Deserialize)]
pub struct ApplicationSettings {
    pub application_host: String,
    pub application_port: u16,
    pub cors_origins: Vec<String>,
    pub app_env: AppEnvironment,
}

#[derive(serde::Deserialize, Copy, Clone, PartialEq)]
pub enum AppEnvironment {
    Production,
    Development,
}

#[derive(serde::Deserialize)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: SecretBox<String>,
    pub port: u16,
    pub host: String,
    pub database_name: String,
}

#[derive(serde::Deserialize)]
pub struct AuthSettings {
    pub jwt_secret: SecretBox<String>,
}

impl DatabaseSettings {
    pub fn connection_string(&self) -> SecretBox<String> {
        SecretBox::new(Box::new(format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username,
            self.password.expose_secret(),
            self.host,
            self.port,
            self.database_name
        )))
    }

    pub fn connection_string_without_db(&self) -> SecretBox<String> {
        SecretBox::new(Box::new(format!(
            "postgres://{}:{}@{}:{}",
            self.username,
            self.password.expose_secret(),
            self.host,
            self.port
        )))
    }
}

const ENV_PREFIX: &str = "DERBE";
const ENV_PREFIX_SEPARATOR: &str = "_";
const ENV_NESTING_SEPARATOR: &str = "__";
const ENV_LIST_SEPARATOR: &str = ",";
const CORS_ORIGINS_KEY: &str = "application.cors_origins";

pub fn get_configuration() -> Result<Settings, ConfigError> {
    // init
    let settings = Config::builder()
        .add_source(File::new("derbe_config.yaml", FileFormat::Yaml))
        .add_source(
            Environment::with_prefix(ENV_PREFIX)
                .prefix_separator(ENV_PREFIX_SEPARATOR)
                .separator(ENV_NESTING_SEPARATOR)
                .list_separator(ENV_LIST_SEPARATOR)
                .with_list_parse_key(CORS_ORIGINS_KEY)
                .try_parsing(true),
        )
        .build()?;

    settings.try_deserialize::<Settings>()
}

#[cfg(test)]
mod tests {
    use super::*;

    const CORS_ORIGINS_ENV: &str = "DERBE_APPLICATION__CORS_ORIGINS";
    const FIRST_ORIGIN: &str = "https://derbe.example.com";
    const SECOND_ORIGIN: &str = "https://www.derbe.example.com";

    #[test]
    fn cors_origins_env_override_is_split_on_commas() {
        std::env::set_var(CORS_ORIGINS_ENV, format!("{FIRST_ORIGIN},{SECOND_ORIGIN}"));
        let settings = get_configuration().expect("configuration should load");
        std::env::remove_var(CORS_ORIGINS_ENV);

        assert_eq!(
            settings.application.cors_origins,
            vec![FIRST_ORIGIN.to_string(), SECOND_ORIGIN.to_string()]
        );
    }
}
