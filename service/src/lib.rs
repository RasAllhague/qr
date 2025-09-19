mod qrcode;

pub use qrcode::{QrCodeDatabase, QrCodeGenerator};

pub static PACKAGE_VERSION: &str = env!("CARGO_PKG_VERSION");
pub static PACKAGE_NAME: &str = env!("CARGO_PKG_NAME");