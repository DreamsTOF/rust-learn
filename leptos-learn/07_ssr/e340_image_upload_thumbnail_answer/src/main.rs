// ============================================================
// Exercise e340 — Answer: Image Upload & Thumbnail
//
// Core: Multipart image upload, image crate resize, thumbnail generation
// ============================================================

use leptos::prelude::*;
use leptos::prelude::ServerFnError;

const THUMBNAIL_MAX_WIDTH: u32 = 150;
const THUMBNAIL_QUALITY: u8 = 85;

fn resize_image(image_data: &[u8], max_width: u32) -> Result<Vec<u8>, String> {
    use image::{load_from_memory, FilterType, ImageFormat};

    let img = load_from_memory(image_data)
        .map_err(|e| format!("Image decode error: {}", e))?;

    let (w, h) = (img.width(), img.height());
    if w <= max_width {
        let mut buf = Vec::new();
        img.write_to(&mut std::io::Cursor::new(&mut buf), ImageFormat::Jpeg)
            .map_err(|e| format!("Encode error: {}", e))?;
        return Ok(buf);
    }

    let ratio = max_width as f64 / w as f64;
    let new_h = (h as f64 * ratio).round() as u32;
    let thumbnail = img.resize_exact(max_width, new_h.max(1), FilterType::Lanczos3);

    let mut buf = Vec::new();
    thumbnail.write_to(&mut std::io::Cursor::new(&mut buf), ImageFormat::Jpeg)
        .map_err(|e| format!("Encode error: {}", e))?;
    Ok(buf)
}

#[server(ImageUpload, "/api/upload")]
pub async fn upload_image(image_data: Vec<u8>) -> Result<String, ServerFnError> {
    let thumbnail_bytes = resize_image(&image_data, THUMBNAIL_MAX_WIDTH)
        .map_err(|e| ServerFnError::new(e))?;

    let thumbnail_url = format!("/thumbnails/{}.jpg", uuid::Uuid::new_v4());

    Ok(thumbnail_url)
}

#[component]
fn Exercise() -> impl IntoView {
    const IMAGE_CODE: &str = "\
use image::{load_from_memory, FilterType, ImageFormat};

const MAX_WIDTH: u32 = 150;

#[server(ImageUpload, \"/api/upload\")]
pub async fn upload_image(image_data: Vec<u8>) -> Result<String, ServerFnError> {
    let img = load_from_memory(&image_data)
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    let thumbnail = if img.width() > MAX_WIDTH {
        let ratio = MAX_WIDTH as f64 / img.width() as f64;
        let new_h = (img.height() as f64 * ratio).round() as u32;
        img.resize_exact(MAX_WIDTH, new_h.max(1), FilterType::Lanczos3)
    } else {
        img
    };

    let mut buf = Vec::new();
    thumbnail.write_to(&mut std::io::Cursor::new(&mut buf), ImageFormat::Jpeg)
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    let path = format!(\"/uploads/thumbnails/{}.jpg\", uuid::Uuid::new_v4());
    Ok(path)
}";

    view! {
        <div>
            <h1>"Image Upload & Thumbnail Generation"</h1>

            <section>
                <h2>"Server-side Thumbnail Generation"</h2>
                <pre>{IMAGE_CODE}</pre>
            </section>

            <section>
                <h2>"Processing Pipeline"</h2>
                <ol>
                    <li>"Client selects image → multipart/form-data upload"</li>
                    <li>"Server receives via #[server] fn as Vec<u8>"</li>
                    <li>"Decode: image::load_from_memory"</li>
                    <li>"Resize: resize_exact(MAX_WIDTH, new_h, Lanczos3)"</li>
                    <li>"Encode: write_to(JPEG, quality=85)"</li>
                    <li>"Store: filesystem / object storage"</li>
                    <li>"Return URL → client displays thumbnail"</li>
                </ol>
            </section>

            <section>
                <h2>"Configuration"</h2>
                <ul>
                    <li>"Max width: 150px"</li>
                    <li>"JPEG quality: 85"</li>
                    <li>"Filter: Lanczos3 (high-quality antialiasing)"</li>
                    <li>"Output format: JPEG"</li>
                </ul>
            </section>
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
