use std::{collections::HashMap, path::PathBuf, sync::Arc};

use image::{ImageEncoder, ImageError, Luma, codecs::png::PngEncoder};
use qrcode::{QrCode, types::QrError};
use thiserror::Error;
use tokio::sync::Mutex;
use url::Url;
use uuid::Uuid;

#[derive(Debug, Error)]
pub enum QrGeneratorError {
    #[error("error during qr code generation, {0}")]
    QrError(#[from] QrError),
    #[error("qr code image saving failed, {0}")]
    ImageError(#[from] ImageError),
}

#[derive(Clone, Debug, Default)]
pub struct QrCodeGenerator {
    pub links: Arc<Mutex<HashMap<Uuid, (String, Url)>>>,
    pub image_base_path: PathBuf,
}

impl QrCodeGenerator {
    pub async fn generate_and_save(&self, id: Uuid) -> Result<Option<String>, QrGeneratorError> {
        let links = self.links.lock().await;

        let Some((_, link)) = links.get(&id) else {
            return Ok(None);
        };

        let code = QrCode::new(link.as_str())?;
        let image = code.render::<Luma<u8>>().build();

        let mut path = self.image_base_path.clone();
        path.push(format!("{}.png", id));

        image.save(&path)?;

        Ok(path.to_str().map(|x| x.to_string()))
    }

    pub async fn generate(&self, id: Uuid) -> Result<Option<Vec<u8>>, QrGeneratorError> {
        let links = self.links.lock().await;

        let Some((_, link)) = links.get(&id) else {
            return Ok(None);
        };

        let code = QrCode::new(link.as_str())?;

        let image = code.render::<Luma<u8>>().build();
        let height = image.height();
        let width = image.width();

        let mut path = self.image_base_path.clone();
        path.push(format!("{}.png", id));
        let data = image.into_raw();

        let mut png_bytes = Vec::new();
        let encoder = PngEncoder::new(&mut png_bytes);
        encoder.write_image(&data, width, height, image::ExtendedColorType::L8)?;

        Ok(Some(png_bytes))
    }
}

#[derive(Clone, Debug, Default)]
pub struct QrCodeDatabase {
    pub links: Arc<Mutex<HashMap<Uuid, (String, Url)>>>,
}

impl QrCodeDatabase {
    pub async fn create(&self, link: Url) -> Result<Uuid, ()> {
        let mut db = self.links.lock().await;

        let new_id = Uuid::new_v4();
        db.insert(new_id, ("TestPwd".to_string(), link));

        Ok(new_id)
    }

    pub async fn get(&self, id: Uuid) -> Result<Option<Url>, ()> {
        let db = self.links.lock().await;

        Ok(db.get(&id).map(|(_, link)| link.clone()))
    }

    pub async fn update(
        &self,
        id: Uuid,
        passphrase: String,
        link: Url,
    ) -> Result<Option<Uuid>, ()> {
        let mut db = self.links.lock().await;

        if let Some((pass, old_link)) = db.get_mut(&id)
            && pass.clone() == passphrase
        {
            *old_link = link;
            return Ok(Some(id));
        }

        return Ok(None);
    }

    pub async fn delete(&self, id: Uuid, passphrase: String) -> Result<Option<Url>, ()> {
        let mut db = self.links.lock().await;

        if let Some((pass, _)) = db.get_mut(&id)
            && pass.clone() == passphrase
        {
            let removed = db.remove(&id).map(|(_, l)| l);
            return Ok(removed);
        }

        Ok(None)
    }
}
