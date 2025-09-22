use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("could not load app configuration file, {0}")]
    LoadFailed(std::io::Error),
    #[error("could not deserialize configuration, {0}")]
    Deserialization(serde_json::error::Error),
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct AppConfig {
    pub server_endpoint: String,
    pub connection_string: String,
    pub image_url: PathBuf,
}

impl AppConfig {
    pub async fn load<P: AsRef<Path>>(path: P) -> Result<Self, ConfigError> {
        let contents = tokio::fs::read_to_string(path)
            .await
            .map_err(|x| ConfigError::LoadFailed(x))?;
        serde_json::de::from_str(&contents).map_err(|x| ConfigError::Deserialization(x))
    }
}
