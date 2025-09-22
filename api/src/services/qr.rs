use poem::web::Data;
use poem_openapi::{
    ApiResponse, Object, OpenApi,
    param::Path,
    payload::{Json, PlainText},
    types::ToJSON,
};
use service::QrCodeDatabase;
use tracing::error;
use url::Url;
use uuid::Uuid;

use crate::services::ApiTags;

#[derive(Object, Debug)]
struct QrCodePostRequest {
    pub link: Url,
}

#[derive(Object, Debug)]
struct QrCodePutRequest {
    pub link: Url,
    pub password: String,
}

#[derive(ApiResponse)]
enum QrCodeTextResponse<T: Into<String> + Send + Sync + 'static> {
    #[oai(status = 200)]
    Ok(PlainText<T>),

    #[oai(status = 404)]
    NotFound(PlainText<String>),

    #[oai(status = 500)]
    InternalError(PlainText<String>),
}

#[derive(ApiResponse)]
enum QrCodeJsonResponse<T: ToJSON + Send + Sync + 'static> {
    #[oai(status = 200)]
    Ok(Json<T>),

    #[oai(status = 404)]
    NotFound(PlainText<String>),

    #[oai(status = 500)]
    InternalError(PlainText<String>),
}

#[derive(ApiResponse)]
enum QrCodeDeleteResponse {
    #[oai(status = 200)]
    Ok,

    #[oai(status = 404)]
    NotFound(PlainText<String>),

    #[oai(status = 500)]
    InternalError(PlainText<String>),
}

#[derive(Object, Debug)]
pub struct QrCodeResponse {
    pub id: Uuid,
    pub link: String,
    pub passphrase: Option<String>,
}

#[derive(ApiResponse)]
pub enum QrCodeCreateResponse {
    #[oai(status = 201)]
    Created(Json<QrCodeResponse>),

    #[oai(status = 500)]
    Database(PlainText<String>),
}

pub struct QrCodeApi;

#[OpenApi]
impl QrCodeApi {
    #[oai(path = "/qr", method = "post", tag = "ApiTags::QrCode")]
    async fn create(
        &self,
        Data(database): Data<&QrCodeDatabase>,
        Json(request): Json<QrCodePostRequest>,
    ) -> QrCodeCreateResponse {
        match database.create(request.link).await {
            Ok(m) => QrCodeCreateResponse::Created(Json(QrCodeResponse {
                id: m.id,
                link: m.link,
                passphrase: Some(m.passphrase),
            })),
            Err(why) => {
                error!("Failed to create new qr code, {why}");
                return QrCodeCreateResponse::Database(PlainText(
                    "Could not create qr code because of an internal error.".to_string(),
                ));
            }
        }
    }

    #[oai(path = "/qr/:id", method = "get", tag = "ApiTags::QrCode")]
    async fn get(
        &self,
        Data(database): Data<&QrCodeDatabase>,
        Path(id): Path<Uuid>,
    ) -> QrCodeJsonResponse<QrCodeResponse> {
        match database.get(id).await {
            Ok(Some(model)) => QrCodeJsonResponse::Ok(Json(QrCodeResponse {
                id: model.id,
                link: model.link,
                passphrase: None,
            })),
            Ok(None) => QrCodeJsonResponse::NotFound(PlainText(
                "No qr code could be found for this id.".to_string(),
            )),
            Err(_) => QrCodeJsonResponse::InternalError(PlainText(
                "Could not retrieve qr code information, because of an internal error.".to_string(),
            )),
        }
    }

    #[oai(path = "/qr/:id", method = "put", tag = "ApiTags::QrCode")]
    async fn update(
        &self,
        Data(database): Data<&QrCodeDatabase>,
        Json(request): Json<QrCodePutRequest>,
        Path(id): Path<Uuid>,
    ) -> QrCodeTextResponse<Uuid> {
        match database.update(id, request.password, request.link).await {
            Ok(Some(model)) => QrCodeTextResponse::Ok(PlainText(model.id)),
            Ok(None) => QrCodeTextResponse::NotFound(PlainText(
                "No qr code could be found for this id.".to_string(),
            )),
            Err(_) => QrCodeTextResponse::InternalError(PlainText(
                "Could not retrieve qr code information, because of an internal error.".to_string(),
            )),
        }
    }

    #[oai(path = "/qr/:id/:pass", method = "delete", tag = "ApiTags::QrCode")]
    async fn delete(
        &self,
        Data(database): Data<&QrCodeDatabase>,
        Path(id): Path<Uuid>,
        Path(password): Path<String>,
    ) -> QrCodeDeleteResponse {
        match database.delete(id, password).await {
            Ok(Some(_)) => QrCodeDeleteResponse::Ok,
            Ok(None) => QrCodeDeleteResponse::NotFound(PlainText(
                "No qr code could be found with this id.".to_string(),
            )),
            Err(_) => QrCodeDeleteResponse::InternalError(PlainText(
                "Could not retrieve qr code information, because of an internal error.".to_string(),
            )),
        }
    }
}
