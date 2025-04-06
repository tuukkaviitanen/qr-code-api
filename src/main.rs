use axum::http::HeaderMap;
use axum::http::header;
use qr_code_generation::QrCodeFormat;
use std::env;
use tower_http::cors::Any;
use tower_http::cors::CorsLayer;
use tower_http::services::ServeFile;

use axum::{
    Router,
    extract::Path,
    response::{IntoResponse, Response},
    routing::get,
};

use errors::Error;

mod errors;
mod qr_code_generation;

#[tokio::main]
async fn main() {
    env_logger::init();

    let port = env::var("PORT").unwrap_or_else(|_| "3000".to_string());

    // build our application with a single route
    let app = Router::new()
        .route("/qr/{*content}", get(get_qr_code))
        .nest_service("/openapi.yaml", ServeFile::new("./api/openapi.yaml"))
        .fallback_service(ServeFile::new("./static/index.html"))
        .layer(CorsLayer::new().allow_origin(Any));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{port}"))
        .await
        .unwrap_or_else(|_| panic!("Failed to bind to port \"{port}\""));

    log::info!("Listening on port {port}");
    axum::serve(listener, app).await.unwrap();
}

async fn get_qr_code(Path(content): Path<String>, headers: HeaderMap) -> Result<Response, Error> {
    let accept_header = headers
        .get("accept")
        .and_then(|value| value.to_str().ok())
        .unwrap_or(QrCodeFormat::Svg.mime_type());

    let format = match accept_header {
        header if header.contains(QrCodeFormat::Svg.mime_type()) => QrCodeFormat::Svg,
        header if header.contains(QrCodeFormat::Png.mime_type()) => QrCodeFormat::Png,
        _ => QrCodeFormat::Svg,
    };

    let qr_code_bytes = qr_code_generation::generate_qr_code(&content, &format).await?;

    Ok((
        axum::http::StatusCode::OK,
        [(header::CONTENT_TYPE, format.mime_type())],
        qr_code_bytes,
    )
        .into_response())
}
