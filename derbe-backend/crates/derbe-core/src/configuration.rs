use secrecy::SecretBox;

/// Read from environment variables (a local `.env` is loaded first), so dev and prod share one path.
#[derive(serde::Deserialize)]
pub struct Settings {
    pub database_url: SecretBox<String>,
    pub jwt_secret: SecretBox<String>,
    pub app_env: AppEnvironment,
    #[serde(default = "default_host")]
    pub app_host: String,
    #[serde(default = "default_port")]
    pub app_port: u16,
    /// Comma separated in the environment: `CORS_ORIGINS=https://a.com,https://b.com`.
    pub cors_origins: Vec<String>,
    /// Missing or empty means email delivery is off.
    #[serde(default)]
    pub resend_api_key: Option<SecretBox<String>>,
    #[serde(default = "default_email_from")]
    pub email_from: String,
    #[serde(default = "default_app_base_url")]
    pub app_base_url: String,
    #[serde(default = "default_resend_base_url")]
    pub resend_base_url: String, // lets tests point at a fake server
}

#[derive(serde::Deserialize, Copy, Clone, PartialEq)]
pub enum AppEnvironment {
    Production,
    Development,
}

fn default_host() -> String {
    "127.0.0.1".to_string()
}

fn default_port() -> u16 {
    8001
}

fn default_email_from() -> String {
    "Derbe <notifications@notify.derbe.app>".into()
}
fn default_app_base_url() -> String {
    "http://localhost:4200".into()
}
fn default_resend_base_url() -> String {
    "https://api.resend.com".into()
}

pub fn get_configuration() -> Result<Settings, envy::Error> {
    dotenvy::dotenv().ok();
    envy::from_env()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn vars(extra: &[(&str, &str)]) -> Vec<(String, String)> {
        [
            ("DATABASE_URL", "postgres://u:p@localhost/db"),
            ("JWT_SECRET", "s"),
            ("APP_ENV", "Production"),
            (
                "CORS_ORIGINS",
                "https://a.example.com,https://b.example.com",
            ),
        ]
        .iter()
        .chain(extra)
        .map(|(k, v)| (k.to_string(), v.to_string()))
        .collect()
    }

    #[test]
    fn cors_origins_are_split_on_commas_and_defaults_apply() {
        let settings: Settings = envy::from_iter(vars(&[])).expect("settings should parse");

        assert_eq!(
            settings.cors_origins,
            vec!["https://a.example.com", "https://b.example.com"]
        );
        assert_eq!(settings.app_host, "127.0.0.1");
        assert_eq!(settings.app_port, 8001);
        assert!(settings.app_env == AppEnvironment::Production);
    }

    #[test]
    fn missing_secret_is_an_error() {
        let without_jwt = vars(&[]).into_iter().filter(|(k, _)| k != "JWT_SECRET");

        assert!(envy::from_iter::<_, Settings>(without_jwt).is_err());
    }

    #[test]
    fn email_settings_are_optional() {
        let settings: Settings = envy::from_iter(vars(&[])).expect("settings should parse");

        assert!(settings.resend_api_key.is_none());
        assert_eq!(
            settings.email_from,
            "Derbe <notifications@notify.derbe.app>"
        );
        assert_eq!(settings.resend_base_url, "https://api.resend.com");
    }
}
