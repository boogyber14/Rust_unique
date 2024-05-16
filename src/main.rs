use qrcode::QrCode;
use qrcode::render::svg;

fn main() {
    // Generate a QR code for a URL
    let code = QrCode::new("https://example.com").unwrap();

    // Render the QR code as SVG
    let svg = code.render::<svg::Color>().build();

    // Save the SVG as a file
    std::fs::write("qr_code.svg", svg).expect("Failed to save QR code");
}
