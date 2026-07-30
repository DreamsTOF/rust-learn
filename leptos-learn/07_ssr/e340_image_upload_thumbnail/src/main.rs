// ============================================================
// 练习 e340: Image Upload & Thumbnail — 图片上传与缩略图生成
//
// 核心知识点:
//   - Multipart 文件上传流程
//   - image crate: 图片解码与缩放
//   - 缩略图生成: resize + 裁剪
//
// 难度: ⭐⭐⭐ (少量 TODO)
// ============================================================

use leptos::prelude::*;
use leptos::prelude::ServerFnError;

// TODO: 定义缩略图配置常量
// ⭐⭐⭐ 提示: 最大宽度 150px, 质量 85
const THUMBNAIL_MAX_WIDTH: u32 = 150;
const THUMBNAIL_QUALITY: u8 = 85;

// TODO: 实现图片缩放函数
// ⭐⭐⭐ 使用 image crate 对 DynamicImage 进行 resize，
// 保持宽高比，返回 Vec<u8> (JPEG 编码)
// 提示: img.resize(max_width, max_height, FilterType::Lanczos3)
fn resize_image(image_data: &[u8], max_width: u32) -> Result<Vec<u8>, String> {
    // 示意实现: 返回原始数据占位
    // 实际代码:
    // use image::{load_from_memory, DynamicImage, FilterType, ImageFormat};
    //
    // let img = load_from_memory(image_data)
    //     .map_err(|e| format!("Image decode error: {}", e))?;
    //
    // let (w, h) = (img.width(), img.height());
    // if w <= max_width {
    //     let mut buf = Vec::new();
    //     img.write_to(&mut std::io::Cursor::new(&mut buf), ImageFormat::Jpeg)
    //         .map_err(|e| format!("Encode error: {}", e))?;
    //     return Ok(buf);
    // }
    //
    // let ratio = max_width as f64 / w as f64;
    // let new_h = (h as f64 * ratio).round() as u32;
    // let thumbnail = img.resize_exact(max_width, new_h.max(1), FilterType::Lanczos3);
    //
    // let mut buf = Vec::new();
    // thumbnail.write_to(&mut std::io::Cursor::new(&mut buf), ImageFormat::Jpeg)
    //     .map_err(|e| format!("Encode error: {}", e))?;
    // Ok(buf)
    Ok(image_data.to_vec())
}

// TODO: 实现 #[server] 上传端点
// ⭐⭐⭐ 使用 #[server(ImageUpload, "/api/upload")]
// 接收 multipart 表单数据，返回缩略图 URL 或错误
#[server(ImageUpload, "/api/upload")]
pub async fn upload_image(image_data: Vec<u8>) -> Result<String, ServerFnError> {
    // 1. 生成缩略图
    let thumbnail_bytes = resize_image(&image_data, THUMBNAIL_MAX_WIDTH)
        .map_err(|e| ServerFnError::new(e))?;

    // 2. 保存缩略图（示意）
    let thumbnail_url = format!("/thumbnails/{}.jpg", uuid::Uuid::new_v4());

    Ok(thumbnail_url)
}

#[component]
fn Exercise() -> impl IntoView {
    const IMAGE_CODE: &str = "\
// 服务端: Multipart 图片上传 + 缩略图生成
use image::{load_from_memory, DynamicImage, FilterType, ImageFormat};

const MAX_WIDTH: u32 = 150;

#[server(ImageUpload, \"/api/upload\")]
pub async fn upload_image(
    image_data: Vec<u8>,
) -> Result<String, ServerFnError> {
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
    thumbnail.write_to(
        &mut std::io::Cursor::new(&mut buf),
        ImageFormat::Jpeg,
    ).map_err(|e| ServerFnError::new(e.to_string()))?;

    let path = format!(\"/uploads/thumbnails/{}.jpg\", uuid::Uuid::new_v4());
    Ok(path)
}";

    view! {
        <div>
            <h1>"Image Upload & Thumbnail Generation"</h1>

            <section>
                <h2>"服务端缩略图生成"</h2>
                <pre>{IMAGE_CODE}</pre>
            </section>

            <section>
                <h2>"处理流程"</h2>
                <ol>
                    <li>"客户端选择图片 → multipart/form-data 上传"</li>
                    <li>"服务端接收: #[server] 函数解析 Vec<u8>"</li>
                    <li>"解码: image::load_from_memory"</li>
                    <li>"缩放: resize_exact(MAX_WIDTH, new_h, Lanczos3)"</li>
                    <li>"编码: write_to(JPEG, quality=85)"</li>
                    <li>"存储: 文件系统 / 对象存储"</li>
                    <li>"返回 URL → 客户端显示"</li>
                </ol>
            </section>

            <section>
                <h2>"配置参数"</h2>
                <ul>
                    <li>"最大宽度: 150px"</li>
                    <li>"JPEG 质量: 85"</li>
                    <li>"滤镜: Lanczos3 (高质量抗锯齿)"</li>
                    <li>"输出格式: JPEG"</li>
                </ul>
            </section>
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
