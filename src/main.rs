use std::env;

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
    let app = Router::new().route("/{*content}", get(get_qr_code));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{port}"))
        .await
        .expect(&format!("Failed to bind to port \"{port}\""));

    log::info!("Listening on port {port}");
    axum::serve(listener, app).await.unwrap();
}

async fn get_qr_code(Path(content): Path<String>) -> Result<Response, Error> {
    let qr_code = qr_code_generation::generate_qr_code(&content).await?;
    Ok((
        axum::http::StatusCode::OK,
        [("content-type", "image/svg+xml")],
        qr_code,
    )
        .into_response())
}
