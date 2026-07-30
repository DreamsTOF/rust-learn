// ============================================================
// 练习 e371 答案: CSV 导出 — 将表格数据导出为 CSV 文件下载
//
// 核心知识点:
//   - 使用 web_sys::Blob 和 URL.createObjectURL 生成文件下载
//   - CSV 格式的字符串拼接（header + 行数据）
//   - 通过 <a download> 触发浏览器文件下载
//   - wasm_bindgen 与 web_sys 的浏览器 API 调用
// ============================================================

use leptos::prelude::*;
use wasm_bindgen::JsCast;

#[derive(Clone, Debug)]
struct User {
    id: u32,
    name: String,
    email: String,
    role: String,
}

fn generate_csv(users: &[User]) -> String {
    let mut csv = String::from("ID,姓名,邮箱,角色\n");
    for user in users {
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

fn escape_csv_field(field: &str) -> String {
    if field.contains(',') || field.contains('"') || field.contains('\n') {
        format!("\"{}\"", field.replace('"', "\"\""))
    } else {
        field.to_string()
    }
}

#[component]
fn Exercise() -> impl IntoView {
    let users = vec![
        User { id: 1, name: "张三".to_string(), email: "zhangsan@example.com".to_string(), role: "管理员".to_string() },
        User { id: 2, name: "李四".to_string(), email: "lisi@example.com".to_string(), role: "编辑".to_string() },
        User { id: 3, name: "王五".to_string(), email: "wangwu@example.com".to_string(), role: "读者".to_string() },
        User { id: 4, name: "赵六".to_string(), email: "zhaoliu@example.com".to_string(), role: "读者".to_string() },
    ];

    let (download_msg, set_download_msg) = signal(String::new());

    let on_export = move |_| {
        let csv_content = generate_csv(&users);

        use leptos::web_sys::{Blob, BlobPropertyBag, Url};

        let mut options = BlobPropertyBag::new();
        options.type_("text/csv;charset=utf-8");
        let blob = Blob::new_with_str_options(&csv_content, &options)
            .expect("创建 Blob 失败");

        let url = Url::create_object_url_with_blob(&blob)
            .expect("创建 URL 失败");

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
                    {users.into_iter().map(|user| view! {
                        <tr>
                            <td style="border: 1px solid #ddd; padding: 8px;">{user.id}</td>
                            <td style="border: 1px solid #ddd; padding: 8px;">{user.name}</td>
                            <td style="border: 1px solid #ddd; padding: 8px;">{user.email}</td>
                            <td style="border: 1px solid #ddd; padding: 8px;">{user.role}</td>
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
                if !msg.is_empty() {
                    view! {
                        <p style="color: #27ae60; font-weight: bold;">{msg}</p>
                    }
                } else {
                    view! { <p>"点击按钮导出表格数据为 CSV 文件"</p> }
                }
            }}
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
