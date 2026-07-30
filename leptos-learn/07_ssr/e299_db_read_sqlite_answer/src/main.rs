// ============================================================
// Exercise 299 - Answer
// ============================================================

use leptos::prelude::*;
use leptos::prelude::ServerFnError;

#[derive(Debug, Clone)]
pub struct Item {
    pub id: i32,
    pub name: String,
    pub description: String,
}

#[server(GetItems)]
pub async fn get_items() -> Result<Vec<Item>, ServerFnError> {
    // 在真实 SSR 应用中会使用 sqlx 查询数据库:
    //   let pool = /* 获取连接池 */;
    //   let items = sqlx::query_as::<_, Item>("SELECT id, name, description FROM items")
    //       .fetch_all(pool)
    //       .await
    //       .map_err(|e| ServerFnError::ServerError(e.to_string()))?;
    //   Ok(items)
    //
    Ok(vec![
        Item { id: 1, name: "学习 Leptos".into(), description: "掌握 Leptos 0.8 nightly API".into() },
        Item { id: 2, name: "编写练习".into(), description: "完成 SSR 相关练习".into() },
        Item { id: 3, name: "部署上线".into(), description: "配置 VPS 环境".into() },
    ])
}

#[component]
fn Exercise() -> impl IntoView {
    let load_action = Action::new(|_: &()| async move {
        get_items().await.unwrap_or_default()
    });

    view! {
        <div style="padding: 1.5rem; font-family: system-ui, sans-serif; max-width: 40rem; margin: 0 auto;">
            <h2 style="border-bottom: 2px solid #e2e8f0; padding-bottom: 0.5rem;">
                "练习 299: SQLite 数据读取"
            </h2>
            <p style="color: #475569;">"点击按钮从服务端获取项目列表。"</p>

            <button
                on:click=move |_| { load_action.dispatch(()); }
                disabled=move || load_action.pending().get()
                style="padding: 0.5rem 1rem; cursor: pointer; background: #3b82f6; color: white; border: none; border-radius: 4px;"
            >
                {move || if load_action.pending().get() { "加载中..." } else { "获取项目列表" }}
            </button>

            <div style="margin-top: 1rem;">
                {move || load_action.value().get().map(|items| {
                    view! {
                        <ul style="list-style: none; padding: 0;">
                            {items.iter().map(|item| {
                                view! {
                                    <li style="padding: 0.5rem; border-bottom: 1px solid #e2e8f0;">
                                        <strong>{&item.name}</strong>
                                        " — "
                                        {&item.description}
                                    </li>
                                }
                            }).collect::<Vec<_>>()}
                        </ul>
                    }
                })}
            </div>
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
