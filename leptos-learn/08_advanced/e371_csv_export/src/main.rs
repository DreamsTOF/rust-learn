// ============================================================
// 练习 e371: CSV 导出 — 将表格数据导出为 CSV 文件下载
//
// 核心知识点:
//   - 使用 web_sys::Blob 和 URL.createObjectURL 生成文件下载
//   - CSV 格式的字符串拼接（header + 行数据）
//   - 通过 <a download> 触发浏览器文件下载
//   - wasm_bindgen 与 web_sys 的浏览器 API 调用
//
// 难度: ⭐⭐ (关键位置有 TODO，补全约 50%)
// ============================================================

use leptos::prelude::*;
use std::rc::Rc;
use wasm_bindgen::JsCast;

/// 用户数据模型
#[derive(Clone, Debug)]
struct User {
    id: u32,
    name: String,
    email: String,
    role: String,
}

/// 生成 CSV 字符串（header + 行数据）
fn generate_csv(users: &[User]) -> String {
    let mut csv = String::from("ID,姓名,邮箱,角色\n");
    for user in users {
        // TODO: 将每个用户的数据拼接为 CSV 行
        csv.push_str(&format!(
            "{},{},{},{}\n",
            user.id,
            escape_csv_field(&user.name),
            user.email,
            user.role,
        ));
    }
    csv
}

/// 转义 CSV 字段（如果包含逗号、引号或换行，用双引号包裹）
fn escape_csv_field(field: &str) -> String {
    if field.contains(',') || field.contains('"') || field.contains('\n') {
        format!("\"{}\"", field.replace('"', "\"\""))
    } else {
        field.to_string()
    }
}

#[component]
fn Exercise() -> impl IntoView {
    let users = Rc::new(vec![
        User { id: 1, name: "张三".to_string(), email: "zhangsan@example.com".to_string(), role: "管理员".to_string() },
        User { id: 2, name: "李四".to_string(), email: "lisi@example.com".to_string(), role: "编辑".to_string() },
        User { id: 3, name: "王五".to_string(), email: "wangwu@example.com".to_string(), role: "读者".to_string() },
        User { id: 4, name: "赵六".to_string(), email: "zhaoliu@example.com".to_string(), role: "读者".to_string() },
    ]);
    let users_for_view = users.clone();

    let (download_msg, set_download_msg) = signal(String::new());

    let on_export = move |_| {
        // TODO: 1. 生成 CSV 字符串
        let csv_content = generate_csv(&users);

        // TODO: 2. 创建 Blob 对象 (MIME: text/csv;charset=utf-8)
        // 提示: 使用 web_sys::Blob::new_with_str_options 和 BlobPropertyBag
        use web_sys::{Blob, BlobPropertyBag};
        let options = BlobPropertyBag::new();
        options.set_type("text/csv;charset=utf-8");
        let parts = js_sys::Array::new();
        parts.push(&wasm_bindgen::JsValue::from_str(&csv_content));
        let blob = Blob::new_with_str_sequence_and_options(&parts, &options)
            .expect("创建 Blob 失败");

        // TODO: 3. 创建对象 URL 用于下载
        use web_sys::Url;
        let url = Url::create_object_url_with_blob(&blob)
            .expect("创建 URL 失败");

        // TODO: 4. 创建 <a> 元素并触发点击下载
        let document = web_sys::window()
            .expect("window 不存在")
            .document()
            .expect("document 不存在");

        let anchor = document
            .create_element("a")
            .expect("创建 a 元素失败")
            .dyn_into::<web_sys::HtmlAnchorElement>()
            .expect("转换为 HtmlAnchorElement 失败");

        anchor.set_href(&url);
        anchor.set_download("用户数据.csv");
        anchor.click();

        Url::revoke_object_url(&url).expect("释放 URL 失败");

        set_download_msg.set(format!(
            "✓ 已导出 {} 条用户数据（CSV 文件已下载）",
            users.len()
        ));
    };

    view! {
        <div style="padding: 20px; max-width: 800px; margin: 0 auto;">
            <h2>"CSV 数据导出"</h2>

            <table style="width: 100%; border-collapse: collapse; margin: 16px 0;">
                <thead>
                    <tr style="background: #f5f5f5;">
                        <th style="border: 1px solid #ddd; padding: 8px; text-align: left;">"ID"</th>
                        <th style="border: 1px solid #ddd; padding: 8px; text-align: left;">"姓名"</th>
                        <th style="border: 1px solid #ddd; padding: 8px; text-align: left;">"邮箱"</th>
                        <th style="border: 1px solid #ddd; padding: 8px; text-align: left;">"角色"</th>
                    </tr>
                </thead>
                <tbody>
                    {users_for_view.iter().map(|user| view! {
                        <tr>
                            <td style="border: 1px solid #ddd; padding: 8px;">{user.id.to_string()}</td>
                            <td style="border: 1px solid #ddd; padding: 8px;">{user.name.clone()}</td>
                            <td style="border: 1px solid #ddd; padding: 8px;">{user.email.clone()}</td>
                            <td style="border: 1px solid #ddd; padding: 8px;">{user.role.clone()}</td>
                        </tr>
                    }).collect_view()}
                </tbody>
            </table>

            <div style="margin: 16px 0;">
                <button
                    on:click=on_export
                    style="padding: 10px 24px; background: #3498db; color: white;
                           border: none; border-radius: 6px; cursor: pointer; font-size: 16px;"
                >
                    "📥 导出 CSV"
                </button>
            </div>

            {move || {
                let msg = download_msg.get();
                let display = if msg.is_empty() {
                    "点击按钮导出表格数据为 CSV 文件".to_string()
                } else {
                    msg
                };
                view! {
                    <p style="color: #27ae60; font-weight: bold;">{display}</p>
                }
            }}
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
