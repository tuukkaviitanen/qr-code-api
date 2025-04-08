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

    let app = Router::new()
        .route("/{format}/{*content}", get(get_qr_code))
        .nest_service("/openapi.yaml", ServeFile::new("./api/openapi.yaml"))
        .fallback_service(ServeFile::new("./static/index.html"))
        .layer(CorsLayer::new().allow_origin(Any));

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{port}"))
        .await
        .unwrap_or_else(|_| panic!("Failed to bind to port \"{port}\""));

    log::info!("Listening on port {port}");
    axum::serve(listener, app).await.unwrap();
}

async fn get_qr_code(Path((format, content)): Path<(String, String)>) -> Result<Response, Error> {
    let parsed_format = match format.as_str() {
        "svg" => {
            log::info!("Generating SVG QR code");
            QrCodeFormat::Svg
        }
        "png" => {
            log::info!("Generating PNG QR code");
            QrCodeFormat::Png
        }
        _ => {
            return Err(Error::InvalidFormat(format));
        }
    };

    let qr_code_bytes = qr_code_generation::generate_qr_code(&content, &parsed_format).await?;

    Ok((
        axum::http::StatusCode::OK,
        [(header::CONTENT_TYPE, parsed_format.mime_type())],
        qr_code_bytes,
    )
        .into_response())
}
