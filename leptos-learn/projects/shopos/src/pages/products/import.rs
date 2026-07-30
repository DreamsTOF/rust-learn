use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos_router::components::A;
use thaw::*;

use crate::server::products::import_products;

#[derive(Debug, Clone)]
struct ParsedProduct {
    line: usize,
    name: String,
    price: f64,
    stock: i32,
    category: String,
    status: String,
}

#[component]
pub fn ImportPage() -> impl IntoView {
    let csv_text = RwSignal::new(String::new());
    let preview = RwSignal::new(Vec::<ParsedProduct>::new());
    let msg = RwSignal::new(String::new());
    let is_error = RwSignal::new(false);
    let importing = RwSignal::new(false);

    let parse_preview = move |_| {
        let text = csv_text.get();
        let mut parsed = Vec::new();
        for (i, line) in text.lines().enumerate() {
            if line.trim().is_empty() {
                continue;
            }
            let cols: Vec<&str> = line.split(',').collect();
            if cols.len() < 4 {
                continue;
            }
            let name = cols[0].trim().to_string();
            let price = cols[1].trim().parse::<f64>().unwrap_or(0.0);
            let stock = cols[2].trim().parse::<i32>().unwrap_or(0);
            let category = cols[3].trim().to_string();
            let status = cols.get(4).map(|s| s.trim().to_string()).unwrap_or_else(|| "active".into());
            parsed.push(ParsedProduct { line: i + 1, name, price, stock, category, status });
        }
        preview.set(parsed);
    };

    let do_import = move |_| {
        let data = csv_text.get();
        if data.trim().is_empty() {
            msg.set("请输入数据".into());
            is_error.set(true);
            return;
        }
        importing.set(true);
        msg.set(String::new());
        spawn_local(async move {
            match import_products(data).await {
                Ok(result) => {
                    msg.set(result);
                    is_error.set(false);
                    importing.set(false);
                }
                Err(e) => {
                    msg.set(format!("导入失败: {}", e));
                    is_error.set(true);
                    importing.set(false);
                }
            }
        });
    };

    // Pre-compute preview content as a closure
    let preview_content = move || {
        let items = preview.get();
        if items.is_empty() {
            view! { <span></span> }.into_any()
        } else {
            view! {
                <Card>
                    <h3>"预览"</h3>
                    <table class="thaw-table">
                        <thead>
                            <tr>
                                <th>"行号"</th>
                                <th>"名称"</th>
                                <th>"价格"</th>
                                <th>"库存"</th>
                                <th>"分类"</th>
                                <th>"状态"</th>
                            </tr>
                        </thead>
                        <tbody>
                            {items.iter().map(|p| {
                                view! {
                                    <tr>
                                        <td>{p.line}</td>
                                        <td>{p.name.to_string()}</td>
                                        <td>{format!("{:.2}", p.price)}</td>
                                        <td>{p.stock}</td>
                                        <td>{p.category.to_string()}</td>
                                        <td>{if p.status == "active" { "上架" } else { "下架" }}</td>
                                    </tr>
                                }
                            }).collect_view()}
                        </tbody>
                    </table>
                </Card>
            }.into_any()
        }
    };

    // Pre-compute msg content as a closure
    let msg_content = move || {
        let m = msg.get();
        if m.is_empty() {
            view! { <span></span> }.into_any()
        } else {
            view! {
                <div style="margin-top: 16px;"><Card>
                    {if is_error.get() {
                        view! { <Text>{m}</Text> }.into_any()
                    } else {
                        view! { <Text>"成功: " {m}</Text> }.into_any()
                    }}
                </Card>
            </div>
            }.into_any()
        }
    };

    view! {
        <div class="page-container">
            <div style="display: flex; margin-bottom: 16px;">
                <A href="/admin/products">
                    <Button>"返回列表"</Button>
                </A>
            </div>
            <h2>"批量导入商品"</h2>
            <p>"请按以下格式输入数据（每行一条，用逗号分隔）："</p>
            <p><code>"商品名称,价格,库存,分类,状态(可选)"</code></p>
            <div style="margin-top: 16px;">
                <textarea
                    placeholder="示例：&#10;测试商品A,99.00,100,电子数码,active&#10;测试商品B,199.00,50,家居用品,inactive"
                    prop:value=move || csv_text.get()
                    on:input=move |ev| csv_text.set(event_target_value(&ev))
                    style="min-height: 200px; width: 100%; padding: 8px; border: 1px solid #d9d9d9; border-radius: 4px;"
                ></textarea>
                <div style="display: flex; gap: 8px; margin-top: 8px;">
                    <Button on_click=parse_preview>"预览"</Button>
                    <Button
                        appearance=ButtonAppearance::Primary
                        on_click=do_import
                        disabled=move || importing.get()
                    >
                        {move || if importing.get() { "导入中..." } else { "开始导入" }}
                    </Button>
                </div>
            </div>

            <div>{preview_content}</div>

            <div>{msg_content}</div>
        </div>
    }
}
