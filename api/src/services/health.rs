use poem_openapi::{OpenApi, payload::PlainText};

use crate::services::ApiTags;

pub struct HealthApi;

#[OpenApi]
impl HealthApi {
    #[oai(path = "/health", method = "get", tag = "ApiTags::Health")]
    async fn health_check(&self) -> PlainText<String> {
        PlainText("I am still alive!".to_string())
    }
}
