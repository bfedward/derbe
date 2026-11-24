use std::sync::Arc;

use secrecy::SecretBox;
use sqlx::PgPool;

use crate::configuration::AppEnvironment;

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
    pub jwt_secret: Arc<SecretBox<String>>,
    pub app_env: AppEnvironment,
}
