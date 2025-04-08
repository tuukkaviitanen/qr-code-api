use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

pub enum Error {
    QrCodeGeneration(qrcode::types::QrError),
    ImageCreation(image::ImageError),
    InvalidFormat(String),
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        match self {
            Error::QrCodeGeneration(err) => match err {
                qrcode::types::QrError::DataTooLong => {
                    (StatusCode::BAD_REQUEST, "Error: Content too long").into_response()
                }
                qrcode::types::QrError::InvalidCharacter => (
                    StatusCode::BAD_REQUEST,
                    "Error: Content includes an invalid character",
                )
                    .into_response(),
                _ => {
                    log::error!("Unexpected QR code generation error: {:?}", err);

                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "Error: Internal server error",
                    )
                        .into_response()
                }
            },
            Error::ImageCreation(err) => {
                log::error!("Image creation error: {:?}", err);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Error: Internal server error",
                )
                    .into_response()
            }
            Error::InvalidFormat(format) => {
                log::error!("Invalid format: {}", format);
                (
                    StatusCode::BAD_REQUEST,
                    format!(
                        "Error: Invalid format '{format}'. Supported formats are 'svg' and 'png'"
                    ),
                )
                    .into_response()
            }
        }
    }
}
