use poem_openapi::{payload::PlainText, OpenApi};

use crate::services::ApiTags;

pub struct HealthApi;

#[OpenApi]
impl HealthApi {
    #[oai(path = "/health", method = "get", tag = "ApiTags::Health")] 
    async fn health_check(&self) -> PlainText<String> {
        PlainText("I am still alive!".to_string())
    }
}