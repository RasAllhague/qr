use std::path::Path;

use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("could not save app configuration file, {0}")]
    SaveFailed(std::io::Error),
    #[error("could not load app configuration file, {0}")]
    LoadFailed(std::io::Error),
    #[error("could not serialize configuration, {0}")]
    Serialization(serde_json::error::Error),
    #[error("could not deserialize configuration, {0}")]
    Deserialization(serde_json::error::Error),
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct AppConfig {
    pub server_endpoint: String,
    pub connection_string: String,
}

impl AppConfig {
    pub async fn load<P: AsRef<Path>>(path: P) -> Result<Self, ConfigError> {
        let contents = tokio::fs::read_to_string(path)
            .await
            .map_err(|x| ConfigError::LoadFailed(x))?;
        serde_json::de::from_str(&contents).map_err(|x| ConfigError::Deserialization(x))
    }

    pub async fn save<P: AsRef<Path>>(&self, path: P) -> Result<(), ConfigError> {
        let json =
            serde_json::ser::to_string_pretty(&self).map_err(|x| ConfigError::Serialization(x))?;
        tokio::fs::write(path, json.as_bytes())
            .await
            .map_err(|x| ConfigError::SaveFailed(x))
    }
}
