use qrcode::QrCode;
use qrcode::render::svg;

use crate::errors::Error;

pub async fn generate_qr_code(data: &str) -> Result<String, Error> {
    // Create a QR code
    let code = QrCode::new(data).map_err(Error::QrCodeGenerationError)?;
    let image = code
        .render()
        .min_dimensions(200, 200)
        .dark_color(svg::Color("#142103"))
        .light_color(svg::Color("#cceba2"))
        .build();

    Ok(image)
}
