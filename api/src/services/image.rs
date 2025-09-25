use poem::web::Data;
use poem_openapi::{
    ApiResponse, Enum, OpenApi,
    param::{Path, Query},
    payload::{Binary, PlainText},
};
use serde::Deserialize;
use service::{QrCodeGenerator, QrImageType};
use uuid::Uuid;

use crate::services::ApiTags;

#[derive(ApiResponse)]
enum ImageResponse {
    #[oai(status = 200, content_type = "image/png")]
    Png(Binary<Vec<u8>>),

    #[oai(status = 200, content_type = "image/jpg")]
    Jpg(Binary<Vec<u8>>),

    #[oai(status = 200, content_type = "image/svg+xml")]
    Svg(
        Binary<Vec<u8>>,
        #[oai(header = "Content-Disposition")] String,
    ),

    #[oai(status = 404)]
    NotFound(PlainText<String>),

    #[oai(status = 500)]
    InternalError(PlainText<String>),
}

#[derive(Clone, Copy, Deserialize, Debug, PartialEq, Eq, Enum)]
#[oai(rename_all = "lowercase")]
enum ImageType {
    Png,
    Jpg,
    Svg,
}

impl Into<QrImageType> for ImageType {
    fn into(self) -> QrImageType {
        match self {
            ImageType::Png => QrImageType::Png,
            ImageType::Jpg => QrImageType::Jpg,
            ImageType::Svg => QrImageType::Svg,
        }
    }
}

pub struct ImageApi;

#[OpenApi]
impl ImageApi {
    #[oai(path = "/image/:id", method = "get", tag = "ApiTags::Image")]
    async fn get_image(
        &self,
        Data(generator): Data<&QrCodeGenerator>,
        Path(id): Path<Uuid>,
        Query(img_type): Query<ImageType>,
    ) -> ImageResponse {
        match generator.generate(id, img_type.into()).await {
            Ok(Some(data)) => match img_type {
                ImageType::Png => ImageResponse::Png(Binary(data)),
                ImageType::Jpg => ImageResponse::Jpg(Binary(data)),
                ImageType::Svg => ImageResponse::Svg(Binary(data), "inline".to_string()),
            },
            Ok(None) => ImageResponse::NotFound(PlainText(
                "No qr code could be found for this id.".to_string(),
            )),
            Err(_) => ImageResponse::InternalError(PlainText(
                "Could not retrieve qr code information, because of an internal error.".to_string(),
            )),
        }
    }
}
