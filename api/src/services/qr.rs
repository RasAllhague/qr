use poem::web::Data;
use poem_openapi::{
    ApiResponse, Object, OpenApi,
    param::Path,
    payload::{Binary, Json, PlainText, Response},
};
use service::{QrCodeDatabase, QrCodeGenerator};
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
enum QrCodeImageResponse {
    #[oai(status = 200, content_type = "image/png")]
    Ok(Binary<Vec<u8>>),

    #[oai(status = 404)]
    NotFound(PlainText<String>),

    #[oai(status = 500)]
    InternalError(PlainText<String>),
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
enum QrCodeDeleteResponse {
    #[oai(status = 200)]
    Ok,

    #[oai(status = 404)]
    NotFound(PlainText<String>),

    #[oai(status = 500)]
    InternalError(PlainText<String>),
}

pub struct QrCodeApi;

#[OpenApi]
impl QrCodeApi {
    #[oai(path = "/qr", method = "post", tag = "ApiTags::QrCode")]
    async fn create(
        &self,
        Data(database): Data<&QrCodeDatabase>,
        Json(request): Json<QrCodePostRequest>,
    ) -> PlainText<Uuid> {
        let new_id = database.create(request.link).await.unwrap();

        PlainText(new_id)
    }

    #[oai(path = "/qr/image", method = "post", tag = "ApiTags::QrCode")]
    async fn create_image(
        &self,
        Data(database): Data<&QrCodeDatabase>,
        Data(generator): Data<&QrCodeGenerator>,
        Json(request): Json<QrCodePostRequest>,
    ) -> Binary<Vec<u8>> {
        let new_id = database.create(request.link).await.unwrap();
        let image = generator.generate(new_id).await.unwrap().unwrap();

        Binary(image)
    }

    #[oai(path = "/qr/:id", method = "get", tag = "ApiTags::QrCode")]
    async fn get(
        &self,
        Data(database): Data<&QrCodeDatabase>,
        Path(id): Path<Uuid>,
    ) -> QrCodeTextResponse<Url> {
        match database.get(id).await {
            Ok(Some(l)) => QrCodeTextResponse::Ok(PlainText(l)),
            Ok(None) => QrCodeTextResponse::NotFound(PlainText(
                "No qr code could be found for this id.".to_string(),
            )),
            Err(_) => QrCodeTextResponse::InternalError(PlainText(
                "Could not retrieve qr code information, because of an internal error.".to_string(),
            )),
        }
    }

    #[oai(path = "/qr/:id/image", method = "get", tag = "ApiTags::QrCode")]
    async fn get_image(
        &self,
        Data(generator): Data<&QrCodeGenerator>,
        Path(id): Path<Uuid>,
    ) -> QrCodeImageResponse {
        match generator.generate(id).await {
            Ok(Some(data)) => QrCodeImageResponse::Ok(Binary(data)),
            Ok(None) => QrCodeImageResponse::NotFound(PlainText(
                "No qr code could be found for this id.".to_string(),
            )),
            Err(_) => QrCodeImageResponse::InternalError(PlainText(
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
            Ok(Some(id)) => QrCodeTextResponse::Ok(PlainText(id)),
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
