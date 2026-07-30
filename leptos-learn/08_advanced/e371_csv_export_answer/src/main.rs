// ============================================================
// 练习 e371: CSV 导出 — 参考答案
//
// 核心知识点:
//   - CSV 格式生成（BOM + 表头 + 行数据 + 字段转义）
//   - Blob / URL.createObjectURL 创建下载链接
//   - 事件委托与数据驱动 UI
// ============================================================

use leptos::prelude::*;
use wasm_bindgen::prelude::*;

/// 用户数据结构
#[derive(Clone, Debug)]
struct User {
    id: u32,
    name: String,
    email: String,
    role: String,
}

/// 转义 CSV 字段（含逗号/引号/换行时用双引号包裹）
fn escape_csv_field(field: &str) -> String {
    if field.contains(',') || field.contains('"') || field.contains('\n') {
        format!("\"{}\"", field.replace('"', "\"\""))
    } else {
        field.to_string()
    }
}

/// 将用户列表转为 CSV 字符串（含 BOM 解决 Excel 中文乱码）
fn users_to_csv(users: &[User]) -> String {
    let mut csv = String::from("\u{feff}"); // BOM
    csv.push_str("ID,姓名,邮箱,角色\n");
    for u in users {
        csv.push_str(&format!(
            "{},{},{},{}\n",
            u.id,
            escape_csv_field(&u.name),
            escape_csv_field(&u.email),
            escape_csv_field(&u.role),
        ));
    }
    csv
}

/// 通过 Blob + URL.createObjectURL 触发 CSV 下载
#[wasm_bindgen(inline_js = r#"
export function download_csv(csv_content, filename) {
    const blob = new Blob([csv_content], { type: 'text/csv;charset=utf-8;' });
    const url = URL.createObjectURL(blob);
    const a = document.createElement('a');
    a.href = url;
    a.download = filename;
    document.body.appendChild(a);
    a.click();
    document.body.removeChild(a);
    URL.revokeObjectURL(url);
}
"#)]
extern "C" {
    fn download_csv(csv_content: &str, filename: &str);
}

#[component]
fn Exercise() -> impl IntoView {
    let users = RwSignal::new(vec![
        User { id: 1, name: "张三".into(), email: "zhangsan@example.com".into(), role: "管理员".into() },
        User { id: 2, name: "李四".into(), email: "lisi@example.com".into(), role: "编辑".into() },
        User { id: 3, name: "王五".into(), email: "wangwu@example.com".into(), role: "查看者".into() },
        User { id: 4, name: "赵六,测试".into(), email: "zhao_liu@test.com".into(), role: "编辑".into() },
    ]);

    let (status, set_status) = signal(String::new());

    let handle_export = move |_| {
        let csv = users_to_csv(&users.get());
        download_csv(&csv, "users_export.csv");
        set_status.set(format!("✅ 已导出 {} 条用户数据", users.get().len()));
    };

    view! {
        <div style="max-width: 600px; margin: 20px auto; font-family: sans-serif;">
            <h3>"📊 CSV 导出演示"</h3>

            <table style="width: 100%; border-collapse: collapse; margin: 12px 0;">
                <thead>
                    <tr style="background: #f5f5f5;">
                        <th style="padding: 8px; border: 1px solid #ddd; text-align: left;">"ID"</th>
                        <th style="padding: 8px; border: 1px solid #ddd; text-align: left;">"姓名"</th>
                        <th style="padding: 8px; border: 1px solid #ddd; text-align: left;">"邮箱"</th>
                        <th style="padding: 8px; border: 1px solid #ddd; text-align: left;">"角色"</th>
                    </tr>
                </thead>
                <tbody>
                    {move || users.get().iter().map(|u| view! {
                        <tr>
                            <td style="padding: 8px; border: 1px solid #ddd;">{u.id}</td>
                            <td style="padding: 8px; border: 1px solid #ddd;">{u.name.clone()}</td>
                            <td style="padding: 8px; border: 1px solid #ddd;">{u.email.clone()}</td>
                            <td style="padding: 8px; border: 1px solid #ddd;">{u.role.clone()}</td>
                        </tr>
                    }).collect_view()}
                </tbody>
            </table>

            <button on:click={handle_export}
                style="padding: 10px 20px; background: #27ae60; color: white;
                       border: none; border-radius: 6px; cursor: pointer; font-size: 1em;">
                "⬇ 导出 CSV"
            </button>

            <p style="margin-top: 12px; color: #666;">{move || status.get()}</p>
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
