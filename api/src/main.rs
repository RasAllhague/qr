mod services;

use std::{collections::HashMap, path::Path, sync::Arc};

use poem::{EndpointExt, Route, Server, listener::TcpListener};
use poem_openapi::OpenApiService;
use service::{QrCodeDatabase, QrCodeGenerator};
use tokio::sync::Mutex;

use crate::services::{HealthApi, QrCodeApi, RedirectApi, VersionApi};

pub static PACKAGE_VERSION: &str = env!("CARGO_PKG_VERSION");
pub static PACKAGE_NAME: &str = env!("CARGO_PKG_NAME");

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    if std::env::var_os("RUST_LOG").is_none() {
        unsafe { std::env::set_var("RUST_LOG", "poem=debug") };
    }
    tracing_subscriber::fmt::init();

    let links = Arc::new(Mutex::new(HashMap::new()));
    let qr_code_database = QrCodeDatabase {
        links: links.clone(),
    };
    let qr_generator = QrCodeGenerator {
        links,
        image_base_path: Path::new("./").to_path_buf(),
    };

    let api_service = OpenApiService::new(
        (HealthApi, RedirectApi, QrCodeApi, VersionApi),
        "qrcode",
        "1.0",
    )
    .server("/api");
    let ui = api_service.swagger_ui();

    Server::new(TcpListener::bind("localhost:3000"))
        .run(
            Route::new()
                .nest("/api", api_service)
                .nest("/docs", ui)
                .data(qr_generator)
                .data(qr_code_database),
        )
        .await
}
