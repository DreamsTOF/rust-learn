// ============================================================
// 练习 e308: 文件上传 — 参考答案
//
// 核心知识点:
//   - 文件输入控件与表单处理
//   - 多部分表单 (multipart/form-data) 解析
//   - 服务端文件接收与存储
//   - Axum 中的 Multipart 提取器
// ============================================================

use leptos::prelude::*;

#[component]
fn Exercise() -> impl IntoView {
    let (selected_file, _set_selected_file) = signal::<Option<String>>(None);
    let (upload_status, set_upload_status) = signal(String::new());

    view! {
        <div style="padding: 1.5rem; font-family: system-ui, sans-serif; max-width: 48rem; margin: 0 auto;">
            <h1 style="border-bottom: 2px solid #e2e8f0; padding-bottom: 0.5rem;">
                "📁 文件上传 (File Upload)"
            </h1>

            <section style="margin: 1.5rem 0;">
                <h2>"文件上传流程"</h2>
                <p style="line-height: 1.6; color: #334155;">
                    "文件上传通常使用 multipart/form-data 编码类型，通过 HTTP POST 请求"
                    "将文件二进制数据与服务端接收处理。在 Axum 中，使用 Multipart 提取器"
                    "来解析多部分表单数据。"
                </p>
                <ol style="line-height: 1.8; color: #334155;">
                    <li>"📤 前端：input type=\"file\" 选择文件"</li>
                    <li>"📦 编码：FormData + multipart/form-data"</li>
                    <li>"🌐 传输：POST 请求发送到服务器"</li>
                    <li>"💾 服务端：Multipart 提取器逐字段处理"</li>
                </ol>
            </section>

            <section style="margin: 1.5rem 0; background: #1e293b; color: #e2e8f0; border-radius: 8px; padding: 1.5rem;">
                <h3 style="color: #38bdf8;">"📝 前端 HTML 表单"</h3>
                <pre style="font-size: 0.875rem; overflow-x: auto;">
"<form action=\"/api/upload\" method=\"post\" enctype=\"multipart/form-data\">
    <input type=\"file\" name=\"file\" accept=\"image/*,application/pdf\">
    <input type=\"text\" name=\"description\" placeholder=\"文件描述\">
    <button type=\"submit\">上传文件</button>
</form>

// JavaScript fetch 上传
const formData = new FormData();
formData.append('file', fileInput.files[0]);
formData.append('description', description);
fetch('/api/upload', {
    method: 'POST',
    body: formData,
});"
                </pre>
            </section>

            <section style="margin: 1.5rem 0; background: #1e293b; color: #e2e8f0; border-radius: 8px; padding: 1.5rem;">
                <h3 style="color: #38bdf8;">"🖥️ 服务端接收代码 (Axum)"</h3>
                <pre style="font-size: 0.875rem; overflow-x: auto;">
"use axum::extract::Multipart;

async fn upload_file(mut multipart: Multipart) -> impl IntoResponse {
    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap_or(\"\").to_string();
        let file_name = field.file_name().unwrap_or(\"unknown\").to_string();
        let content_type = field.content_type().unwrap_or(\"\").to_string();
        let data = field.bytes().await.unwrap();
        println!(\"收到文件: name={name}, file_name={file_name}, size={} bytes\", data.len());
    }
    \"文件上传成功\"
}"
                </pre>
            </section>

            <section style="margin: 1.5rem 0; background: #f8fafc; border: 1px solid #e2e8f0; border-radius: 8px; padding: 1.5rem;">
                <h3>"📋 模拟上传状态"</h3>
                <div style="border: 2px dashed #cbd5e1; border-radius: 8px; padding: 2rem; text-align: center;">
                    <p style="color: #64748b;">
                        {move || match selected_file.get() {
                            Some(name) => format!("已选择文件: {name}"),
                            None => "尚未选择文件".to_string(),
                        }}
                    </p>
                    <button
                        style="background: #2563eb; color: white; border: none; padding: 0.5rem 1.5rem; border-radius: 4px; cursor: pointer; margin-top: 1rem;"
                        on:click=move |_| set_upload_status("文件已上传 (模拟)".to_string())
                    >
                        "模拟上传"
                    </button>
                    <p style="color: #059669; margin-top: 1rem;">
                        {move || upload_status.get()}
                    </p>
                </div>
            </section>

            <section style="margin: 1.5rem 0; padding: 1rem; background: #f0f9ff; border-left: 4px solid #38bdf8; border-radius: 4px;">
                <p style="margin: 0; font-size: 0.9rem;">
                    <strong>"🔑 关键概念："</strong>
                    "enctype=\"multipart/form-data\" 是文件上传必需的编码类型；"
                    "Axum 的 Multipart 提取器自动解析多部分数据；"
                    "bytes() 方法提取文件二进制数据；大文件建议使用 stream() 流式处理。"
                </p>
            </section>

            <details style="margin: 1.5rem 0;">
                <summary style="cursor: pointer; font-weight: 600; color: #2563eb;">
                    "📖 点击展开完整代码参考"
                </summary>
                <pre style="background: #f1f5f9; padding: 1rem; border-radius: 4px; font-size: 0.8rem; overflow-x: auto; margin-top: 0.5rem;">
"use axum::{Router, extract::Multipart, response::IntoResponse, routing::post};
use tokio::fs;

async fn upload_file(mut multipart: Multipart) -> impl IntoResponse {
    while let Some(field) = multipart.next_field().await.unwrap() {
        let file_name = field.file_name().unwrap_or(\"unknown\").to_string();
        let data = field.bytes().await.unwrap();
        let save_path = format!(\"./uploads/{file_name}\");
        fs::write(&save_path, &data).await.unwrap();
        println!(\"保存文件: {save_path} ({} bytes)\", data.len());
    }
    \"文件上传成功\"
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route(\"/api/upload\", post(upload_file));
    let listener = tokio::net::TcpListener::bind(\"0.0.0.0:3000\").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
"
                </pre>
            </details>
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
