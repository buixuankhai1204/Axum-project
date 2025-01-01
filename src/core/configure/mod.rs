use self::{db::DatabaseConfig, server::ServerConfig};
use crate::util::dir::get_project_root;
use config::{ConfigError, Environment};
use serde::{Deserialize, Serialize};
use std::str::FromStr;

pub mod db;
pub mod env;
pub mod sentry;
pub mod server;
pub mod trace;

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub profile: Profile,
    pub server: ServerConfig,
    pub db: DatabaseConfig,
    pub sentry: Sentry,
}

impl AppConfig {
    pub fn read(env_src: Environment) -> Result<Self, config::ConfigError> {
        let config_dir = get_settings_dir()?;
        let profile = std::env::var("APP_PROFILE")
            .map(|env| Profile::from_str(&env).map_err(|e| ConfigError::Message(e.to_string())))
            .unwrap_or_else(|_e| Ok(Profile::Dev))?;
        let profile_filename = format!("{profile}.toml");
        let config = config::Config::builder()
            .add_source(config::File::from(config_dir.join("dev.toml")))
            .add_source(config::File::from(config_dir.join(profile_filename)))
            .add_source(env_src)
            .build()?;
        tracing::info!("Successfully read config profile: {profile}.");
        config.try_deserialize()
    }

    pub fn get_sentry_dsn(&self) -> &str {
        &self.sentry.dsn
    }
}

pub fn get_settings_dir() -> Result<std::path::PathBuf, ConfigError> {
    Ok(get_project_root().map_err(|e| ConfigError::Message(e.to_string()))?.join("settings"))
}

pub fn get_static_dir() -> Result<std::path::PathBuf, ConfigError> {
    Ok(get_project_root().map_err(|e| ConfigError::Message(e.to_string()))?.join("static"))
}

#[derive(
    Debug,
    strum::Display,
    strum::EnumString,
    Deserialize,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Clone,
    Copy,
)]
pub enum Profile {
    #[serde(rename = "dev")]
    #[strum(serialize = "dev")]
    Dev,
    #[serde(rename = "stag")]
    #[strum(serialize = "stag")]
    Stag,
    #[serde(rename = "prod")]
    #[strum(serialize = "prod")]
    Prod,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Sentry {
    pub dsn: String,
}
