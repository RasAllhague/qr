use poem::web::Data;
use poem_openapi::{ApiResponse, OpenApi, param::Query, payload::PlainText};
use service::QrCodeDatabase;
use url::Url;
use uuid::Uuid;

use crate::services::ApiTags;

#[derive(ApiResponse)]
enum RedirectResponse {
    #[oai(status = 302)]
    Redirect(#[oai(header = "Location")] Url),
    #[oai(status = 404)]
    NotFound(PlainText<String>),
}

pub struct RedirectApi;

#[OpenApi]
impl RedirectApi {
    #[oai(path = "/redirect", method = "get", tag = "ApiTags::Redirect")]
    async fn redirect(
        &self,
        Data(database): Data<&QrCodeDatabase>,
        Query(id): Query<Uuid>,
    ) -> RedirectResponse {
        let locked_database = database.links.lock().await;

        if let Some((_, url)) = locked_database.get(&id) {
            return RedirectResponse::Redirect(url.clone());
        }

        RedirectResponse::NotFound(PlainText(
            "The requested qr code id could not be found.".to_string(),
        ))
    }
}
