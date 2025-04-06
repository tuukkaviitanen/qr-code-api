use std::io::Cursor;

use image::{Luma, Rgb};
use qrcode::render::svg;
use qrcode::{Color, QrCode};

use crate::errors::Error;

pub enum QrCodeFormat {
    Svg,
    Png,
}

impl QrCodeFormat {
    pub fn mime_type(&self) -> &str {
        match self {
            QrCodeFormat::Svg => "image/svg+xml",
            QrCodeFormat::Png => "image/png",
        }
    }
}

pub async fn generate_qr_code(data: &str, format: &QrCodeFormat) -> Result<Vec<u8>, Error> {
    // Create a QR code
    let code = QrCode::new(data).map_err(Error::QrCodeGenerationError)?;

    match format {
        QrCodeFormat::Svg => {
            // Render the QR code as SVG
            let svg_data = code
                .render()
                .dark_color(svg::Color("#142103"))
                .light_color(svg::Color("#cceba2"))
                .build();

            let bytes = svg_data.as_bytes().to_vec();
            Ok(bytes)
        }
        QrCodeFormat::Png => {
            // Render the QR code as PNG
            let png_data = code
                .render()
                .dark_color(Rgb([20u8, 33u8, 3u8]))
                .light_color(Rgb([204u8, 235u8, 162u8]))
                .build();

            let mut bytes: Vec<u8> = Vec::new();

            png_data
                .write_to(&mut Cursor::new(&mut bytes), image::ImageFormat::Png)
                .map_err(Error::ImageCreationError)?;

            Ok(bytes)
        }
    }
}
