use crate::util;
use serde::Deserialize;
use std::{fs, path::PathBuf};

#[derive(Debug, Deserialize, Clone)]
pub struct SecretConfig {
    pub private_access_key: PathBuf,
    pub public_access_key: PathBuf,
    pub private_refresh_key: PathBuf,
    pub public_refresh_key: PathBuf,
}

impl SecretConfig {
    pub fn read_private_access_key(&self) -> Result<String, std::io::Error> {
        fs::read_to_string(util::dir::get_project_root()?.join(&self.private_access_key))
    }

    pub fn read_public_access_key(&self) -> Result<String, std::io::Error> {
        fs::read_to_string(util::dir::get_project_root()?.join(&self.public_access_key))
    }

    pub fn read_private_refresh_key(&self) -> Result<String, std::io::Error> {
        fs::read_to_string(util::dir::get_project_root()?.join(&self.private_refresh_key))
    }

    pub fn read_public_refresh_key(&self) -> Result<String, std::io::Error> {
        fs::read_to_string(util::dir::get_project_root()?.join(&self.public_refresh_key))
    }
}
