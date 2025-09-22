use poem_openapi::Tags;

mod health;
mod qr;
mod redirect;
mod version;
mod image;

pub use health::HealthApi;
pub use qr::QrCodeApi;
pub use redirect::RedirectApi;
pub use version::VersionApi;
pub use image::ImageApi;

#[derive(Tags)]
pub enum ApiTags {
    Health,
    QrCode,
    Redirect,
    Version,
    Image,
}


