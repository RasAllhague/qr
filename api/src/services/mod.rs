use poem_openapi::Tags;

mod health;
mod qr;
mod redirect;
mod version;

pub use health::HealthApi;
pub use qr::QrCodeApi;
pub use redirect::RedirectApi;
pub use version::VersionApi;

#[derive(Tags)]
pub enum ApiTags {
    Health,
    QrCode,
    Redirect,
    Version,
}