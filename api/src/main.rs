use poem::{listener::TcpListener, Route, Server};
use poem_openapi::{param::Query, payload::PlainText, OpenApi, OpenApiService, Tags};
use uuid::Uuid;

#[derive(Tags)]
enum ApiTags {
    Health,
    QrCode,
}

struct Api;

#[OpenApi]
impl Api {
    #[oai(path = "/health", method = "get", tag = "ApiTags::Health")] 
    async fn health_check(&self) -> PlainText<String> {
        PlainText("I am still alive!".to_string())
    }

    #[oai(path = "/qr/redirect", method = "get", tag = "ApiTags::QrCode")] 
    async fn index(&self, Query(id): Query<Uuid>) -> PlainText<String> {
        PlainText(format!("You requested: {}", id))
    }
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    if std::env::var_os("RUST_LOG").is_none() {
        unsafe { std::env::set_var("RUST_LOG", "poem=debug") };
    }
    tracing_subscriber::fmt::init();

    let api_service = OpenApiService::new(Api, "qrcode", "1.0").server("http://localhost:3000/api");
    let ui = api_service.swagger_ui();

    Server::new(TcpListener::bind("localhost:3000"))
        .run(Route::new().nest("/api", api_service).nest("/", ui))
        .await    
}