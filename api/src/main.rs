mod services;
mod config;
mod pages;

use migration::sea_orm::Database;
use poem::{get, listener::TcpListener, middleware::Tracing, EndpointExt, Route, Server};
use poem_openapi::OpenApiService;
use service::{QrCodeDatabase, QrCodeGenerator};

use crate::{config::AppConfig, pages::*, services::{HealthApi, ImageApi, QrCodeApi, RedirectApi, VersionApi}};

pub static PACKAGE_VERSION: &str = env!("CARGO_PKG_VERSION");
pub static PACKAGE_NAME: &str = env!("CARGO_PKG_NAME");

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    if std::env::var_os("RUST_LOG").is_none() {
        unsafe { std::env::set_var("RUST_LOG", "poem=debug") };
    }
    tracing_subscriber::fmt::init();

    let app_config = AppConfig::load("../config.json").await.expect("App config file not present!");

    let conn = Database::connect(&app_config.connection_string).await.unwrap();

    let qr_code_database = QrCodeDatabase { db_conn: conn.clone() };
    let qr_generator = QrCodeGenerator { db_conn: conn.clone(), image_base_path: app_config.image_url };

    let api_service = OpenApiService::new(
        (HealthApi, RedirectApi, QrCodeApi, VersionApi, ImageApi),
        "qrcode",
        "1.0",
    )
    .server("/api");
    let ui = api_service.swagger_ui();

    Server::new(TcpListener::bind(&app_config.server_endpoint))
        .run(
            Route::new()
                .at("/", get(index_ui))
                .at("/new", get(new_ui))
                .at("/edit", get(edit_ui))
                .at("/delete", get(delete_ui))
                .at("/impressum", get(legal_notice_ui))
                .at("/privacy", get(privacy_ui))
                .nest("/api", api_service)
                .nest("/docs", ui)
                .with(Tracing)
                .data(qr_generator)
                .data(qr_code_database),
        )
        .await
}
