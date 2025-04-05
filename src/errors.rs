use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use log::error;

pub enum Error {
    QrCodeGenerationError(qrcode::types::QrError),
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        match self {
            Error::QrCodeGenerationError(err) => match err {
                qrcode::types::QrError::DataTooLong => {
                    (StatusCode::BAD_REQUEST, "Error: Content too long").into_response()
                }
                qrcode::types::QrError::InvalidCharacter => (
                    StatusCode::BAD_REQUEST,
                    "Error: Content includes an invalid character",
                )
                    .into_response(),
                _ => {
                    error!("Unexpected QR code generation error: {:?}", err);

                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "Error: Internal server error",
                    )
                        .into_response()
                }
            },
        }
    }
}
