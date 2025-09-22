use poem::web::Data;
use poem_openapi::{ApiResponse, OpenApi, param::Query, payload::PlainText};
use service::QrCodeDatabase;
use tracing::error;
use url::Url;
use uuid::Uuid;

use crate::services::ApiTags;

#[derive(ApiResponse)]
enum RedirectResponse {
    #[oai(status = 302)]
    Redirect(#[oai(header = "Location")] Url),
    #[oai(status = 404)]
    NotFound(PlainText<String>),
    #[oai(status = 500)]
    DatabaseError(PlainText<String>),
    #[oai(status = 500)]
    InvalidUrl(PlainText<String>),
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
        match database
            .get(id)
            .await
            .map(|x| x.map(|y| url::Url::parse(&y.link)))
        {
            Ok(Some(Ok(url))) => RedirectResponse::Redirect(url),
            Ok(Some(Err(why))) => {
                error!("Could not redirect user because of an malformed url, {why}");
                return RedirectResponse::InvalidUrl(PlainText(
                    "The redirect url is broken.".to_string(),
                ));
            }
            Ok(None) => RedirectResponse::NotFound(PlainText(
                "The requested qr code id could not be found.".to_string(),
            )),
            Err(why) => {
                error!("Could not redirect user because of {why}");
                return RedirectResponse::DatabaseError(PlainText(
                    "The redirection failed because of an internal error.".to_string(),
                ));
            }
        }
    }
}
