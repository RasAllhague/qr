use std::path::PathBuf;

use ::entity::qr_code::{self, Entity as DbQrCode};
use chrono::Utc;
use entity::qr_code::{ActiveModel, Model};
use image::{
    ImageEncoder, ImageError, Luma,
    codecs::{jpeg::JpegEncoder, png::PngEncoder},
};
use qrcode::{QrCode, render::svg, types::QrError};
use sea_orm::{ActiveModelTrait, ActiveValue::Set, DbConn, DbErr, EntityTrait};
use thiserror::Error;
use url::Url;
use uuid::Uuid;

#[derive(Debug, Error)]
pub enum QrGeneratorError {
    #[error("error during qr code generation, {0}")]
    QrError(#[from] QrError),
    #[error("qr code image saving failed, {0}")]
    ImageError(#[from] ImageError),
    #[error("database operation failed, {0}")]
    DataBaseError(#[from] DbErr),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum QrImageType {
    Png,
    Jpg,
    Svg,
}

#[derive(Clone, Debug, Default)]
pub struct QrCodeGenerator {
    pub db_conn: DbConn,
    pub image_base_path: PathBuf,
    pub server_url: String,
}

impl QrCodeGenerator {
    pub async fn generate_and_save(&self, id: Uuid) -> Result<Option<String>, QrGeneratorError> {
        let Some(qr_code) = DbQrCode::find_by_id(id).one(&self.db_conn).await? else {
            return Ok(None);
        };

        let code = QrCode::new(qr_code.link)?;
        let image = code.render::<Luma<u8>>().build();

        let mut path = self.image_base_path.clone();
        path.push(format!("{}.png", id));

        image.save(&path)?;

        Ok(path.to_str().map(|x| x.to_string()))
    }

    pub async fn generate(
        &self,
        id: Uuid,
        image_type: QrImageType,
    ) -> Result<Option<Vec<u8>>, QrGeneratorError> {
        let Some(qr_code) = DbQrCode::find_by_id(id).one(&self.db_conn).await? else {
            return Ok(None);
        };
        
        let code = QrCode::new(&format!("{}/redirect?id={}", self.server_url, qr_code.id))?;

        let image = code.render::<Luma<u8>>().build();
        let height = image.height();
        let width = image.width();

        let data = image.into_raw();
        let mut png_bytes = Vec::new();

        match image_type {
            QrImageType::Png => {
                let encoder = PngEncoder::new(&mut png_bytes);
                encoder.write_image(&data, width, height, image::ExtendedColorType::L8)?;
            }
            QrImageType::Jpg => {
                let encoder = JpegEncoder::new(&mut png_bytes);
                encoder.write_image(&data, width, height, image::ExtendedColorType::L8)?;
            }
            QrImageType::Svg => {
                let svg_str = code
                    .render::<svg::Color>()
                    .min_dimensions(200, 200)
                    .dark_color(svg::Color("#000000"))
                    .light_color(svg::Color("#ffffff"))
                    .build();
                png_bytes = svg_str.into_bytes();
            }
        };

        Ok(Some(png_bytes))
    }
}

#[derive(Clone, Debug, Default)]
pub struct QrCodeDatabase {
    pub db_conn: DbConn,
}

impl QrCodeDatabase {
    pub async fn create(&self, link: Url) -> Result<Model, DbErr> {
        let qr_code = qr_code::ActiveModel {
            id: Set(uuid::Uuid::new_v4()),
            link: Set(link.to_string()),
            passphrase: Set("TestPwd".to_string()),
            created_at: Set(Utc::now()),
            ..Default::default()
        }
        .insert(&self.db_conn)
        .await?;

        Ok(qr_code)
    }

    pub async fn get(&self, id: Uuid) -> Result<Option<Model>, DbErr> {
        let Some(qr_code) = DbQrCode::find_by_id(id).one(&self.db_conn).await? else {
            return Ok(None);
        };

        Ok(Some(qr_code))
    }

    pub async fn update(
        &self,
        id: Uuid,
        passphrase: String,
        link: Url,
    ) -> Result<Option<Model>, DbErr> {
        let Some(qr_code) = DbQrCode::find_by_id(id).one(&self.db_conn).await? else {
            return Ok(None);
        };

        if qr_code.passphrase != passphrase {
            return Ok(None);
        }

        let mut active: ActiveModel = qr_code.into();
        active.link = Set(link.to_string());
        active.modified_at = Set(Some(Utc::now()));
        let qr_code = active.update(&self.db_conn).await?;

        Ok(Some(qr_code))
    }

    pub async fn delete(&self, id: Uuid, passphrase: String) -> Result<Option<Model>, DbErr> {
        let Some(qr_code) = DbQrCode::find_by_id(id).one(&self.db_conn).await? else {
            return Ok(None);
        };

        if qr_code.passphrase != passphrase {
            return Ok(None);
        }

        DbQrCode::delete_by_id(id).exec(&self.db_conn).await?;

        Ok(Some(qr_code))
    }
}
