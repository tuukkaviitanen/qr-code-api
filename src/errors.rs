use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

pub enum Error {
    QrCodeGenerationError(qrcode::types::QrError),
    ImageCreationError(image::ImageError),
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
                    log::error!("Unexpected QR code generation error: {:?}", err);

                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "Error: Internal server error",
                    )
                        .into_response()
                }
            },
            Error::ImageCreationError(err) => {
                log::error!("Image creation error: {:?}", err);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Error: Internal server error",
                )
                    .into_response()
            }
        }
    }
}
