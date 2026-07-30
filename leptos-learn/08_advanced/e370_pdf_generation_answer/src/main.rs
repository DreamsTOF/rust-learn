// ============================================================
// 参考答案 e370: PDF 生成 — 通过浏览器打印/生成 PDF
//
// 核心知识点:
//   - 使用浏览器 window.print() 打印为 PDF
//   - 通过 #[wasm_bindgen(inline_js)] 调用浏览器 API
//   - 显示可打印的内容预览
//   - 提供"导出 PDF"按钮
// ============================================================

use leptos::prelude::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(inline_js = r#"
export function printPage() {
    window.print();
}
"#)]
extern "C" {
    fn printPage();
}

#[component]
fn Exercise() -> impl IntoView {
    let handle_print = move |_| {
        printPage();
    };

    view! {
        <div style="padding: 1rem; font-family: sans-serif; max-width: 600px; margin: 0 auto;">
            <h3>"练习 e370: PDF 生成"</h3>
            <p style="color: #666; font-size: 14px;">
                "点击下方按钮通过浏览器打印功能导出为 PDF。在打印对话框中选择「另存为 PDF」即可。"
            </p>

            <button
                on:click=handle_print
                style="background: #4caf50; color: white; border: none; padding: 12px 24px;
                       border-radius: 6px; cursor: pointer; font-size: 16px; margin-bottom: 16px;"
            >
                "📄 导出 PDF"
            </button>

            <div
                class="print-content"
                style="border: 2px dashed #ddd; border-radius: 8px; padding: 24px;
                       background: #fafafa;"
            >
                <h2 style="border-bottom: 2px solid #333; padding-bottom: 8px; margin-top: 0;">
                    "Leptos 学习报告"
                </h2>

                <div style="line-height: 1.8;">
                    <p><strong>"学员:"</strong> "Rust 开发者"</p>
                    <p><strong>"课程:"</strong> "Leptos 高级 WebAssembly 开发"</p>
                    <p><strong>"日期:"</strong> "2026 年 7 月 28 日"</p>
                </div>

                <h3>"学习进度"</h3>
                <table style="width: 100%; border-collapse: collapse; margin-bottom: 16px;">
                    <thead>
                        <tr style="background: #e0e0e0;">
                            <th style="padding: 8px; border: 1px solid #ccc; text-align: left;">"章节"</th>
                            <th style="padding: 8px; border: 1px solid #ccc; text-align: left;">"状态"</th>
                            <th style="padding: 8px; border: 1px solid #ccc; text-align: left;">"备注"</th>
                        </tr>
                    </thead>
                    <tbody>
                        <tr>
                            <td style="padding: 8px; border: 1px solid #ccc;">"第一章: 基础"</td>
                            <td style="padding: 8px; border: 1px solid #ccc; color: #4caf50;">"✅ 已完成"</td>
                            <td style="padding: 8px; border: 1px solid #ccc;">"20 个练习"</td>
                        </tr>
                        <tr>
                            <td style="padding: 8px; border: 1px solid #ccc;">"第二章: 信号"</td>
                            <td style="padding: 8px; border: 1px solid #ccc; color: #4caf50;">"✅ 已完成"</td>
                            <td style="padding: 8px; border: 1px solid #ccc;">"75 个练习"</td>
                        </tr>
                        <tr>
                            <td style="padding: 8px; border: 1px solid #ccc;">"第三章: 组件"</td>
                            <td style="padding: 8px; border: 1px solid #ccc; color: #ff9800;">"🔄 进行中"</td>
                            <td style="padding: 8px; border: 1px solid #ccc;">"60 个练习"</td>
                        </tr>
                        <tr>
                            <td style="padding: 8px; border: 1px solid #ccc;">"第八章: 高级"</td>
                            <td style="padding: 8px; border: 1px solid #ccc; color: #2196f3;">"📝 学习中"</td>
                            <td style="padding: 8px; border: 1px solid #ccc;">"PDF 生成练习"</td>
                        </tr>
                    </tbody>
                </table>

                <p style="color: #666; font-style: italic; font-size: 13px;">
                    "此报告由 Leptos WebAssembly 应用生成。此内容将出现在 PDF 中。"
                </p>
            </div>

            <style>
                "@media print {
                    body * { visibility: hidden; }
                    .print-content, .print-content * { visibility: visible; }
                    .print-content { position: absolute; left: 0; top: 0; border: none !important; }
                }"
            </style>
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
