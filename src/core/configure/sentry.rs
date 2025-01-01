use sentry::ClientInitGuard;
use serde::Deserialize;
use crate::core::configure::AppConfig;

#[derive(Debug, Deserialize, Clone)]
pub struct SentryConfig {
    key: String,
}

pub fn init(app_config: &AppConfig) -> ClientInitGuard {
    let sentry_dsn = app_config.get_sentry_dsn();
    sentry::init((
        sentry_dsn,
        sentry::ClientOptions { release: sentry::release_name!(), ..Default::default() },
    ))
}
