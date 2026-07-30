// ============================================================
// 练习 e370: PDF 生成 — 通过浏览器打印/生成 PDF
//
// 核心知识点:
//   - 使用浏览器 window.print() 打印为 PDF
//   - 通过 #[wasm_bindgen(inline_js)] 调用浏览器 API
//   - 显示可打印的内容预览
//   - 提供"导出 PDF"按钮
//
// 难度: ⭐⭐ (关键位置有 TODO，补全约 50%)
// ============================================================

use leptos::prelude::*;
use wasm_bindgen::prelude::*;

// TODO: 定义 inline_js 导出 printPage 函数
// 使用 window.print() 触发浏览器打印对话框
// 用户可以在打印对话框中选择"另存为 PDF"

#[component]
fn Exercise() -> impl IntoView {
    // TODO: 实现打印功能
    // 点击按钮时调用 window.print()
    // 使用 web_sys::window() 或 inline_js

    view! {
        <div style="padding: 1rem; font-family: sans-serif; max-width: 600px; margin: 0 auto;">
            <h3>"练习 e370: PDF 生成"</h3>
            <p style="color: #666; font-size: 14px;">
                "点击下方按钮通过浏览器打印功能导出为 PDF"
            </p>

            // TODO: 添加"导出 PDF"按钮，点击触发打印

            // TODO: 显示打印内容预览区域
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
