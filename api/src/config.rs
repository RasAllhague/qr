use std::env;

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub database_url: String,
    pub server_url: String,
    pub image_base_path: String,
    pub domain_name: String, // New field
}

impl AppConfig {
    pub fn from_env() -> Self {
        Self {
            database_url: env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
            server_url: env::var("SERVER_URL").unwrap_or_else(|_| "http://localhost:3000".to_string()),
            image_base_path: env::var("IMAGE_BASE_PATH").unwrap_or_else(|_| "./images".to_string()),
            domain_name: env::var("DOMAIN_NAME").unwrap_or_else(|_| "localhost".to_string()), // New field
        }
    }
}