use poem_openapi::{OpenApi, payload::PlainText};

use crate::services::ApiTags;

pub struct VersionApi;

#[OpenApi]
impl VersionApi {
    #[oai(path = "/version", method = "get", tag = "ApiTags::Version")]
    async fn version(&self) -> PlainText<String> {
        let api_version = format!("{}: {}", crate::PACKAGE_NAME, crate::PACKAGE_VERSION);
        let service_version = format!("{}: {}", service::PACKAGE_NAME, service::PACKAGE_VERSION);

        let version = format!("{api_version}\n{service_version}");

        PlainText(version)
    }
}
