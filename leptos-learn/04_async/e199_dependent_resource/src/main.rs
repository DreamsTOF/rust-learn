// ============================================================
// 练习 199: dependent_resource — Resource 链式依赖
//
// 目标: 第二个 Resource 依赖第一个 Resource 的选择结果
//
// 难度: ⭐⭐⭐
// 核心知识点: Resource 链式依赖
//
// TODO:
//   1. 实现 sleep 工具函数
//   2. fetch_categories: 模拟加载分类列表
//   3. fetch_items(cat): 根据分类 ID 加载项目列表
//   4. 第一个 Resource 加载分类, 选择后触发第二个 Resource
// ============================================================

use serde::{Deserialize, Serialize};
use std::time::Duration;
use futures::channel::oneshot;
use leptos::prelude::*;

async fn sleep(ms: u64) {
    let (tx, rx) = oneshot::channel::<()>();
    set_timeout(move || { let _ = tx.send(()); }, Duration::from_millis(ms));
    let _ = rx.await;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Category {
    id: u32,
    name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Item {
    id: u32,
    title: String,
}

/// 加载分类列表
async fn fetch_categories() -> Vec<Category> {
    sleep(500).await;
    vec![
        Category { id: 1, name: "前端框架".into() },
        Category { id: 2, name: "后端技术".into() },
        Category { id: 3, name: "数据库".into() },
        Category { id: 4, name: "DevOps".into() },
    ]
}

/// 根据分类 ID 加载项目列表
async fn fetch_items(category_id: u32) -> Vec<Item> {
    let items = match category_id {
        1 => vec![
            Item { id: 1, title: "Leptos — Rust 响应式框架".into() },
            Item { id: 2, title: "Solid.js — 细粒度响应式".into() },
            Item { id: 3, title: "React — 虚拟 DOM".into() },
        ],
        2 => vec![
            Item { id: 4, title: "Axum — Rust 异步运行时".into() },
            Item { id: 5, title: "Actix — Actor 模型".into() },
        ],
        3 => vec![
            Item { id: 6, title: "SQLite — 嵌入式数据库".into() },
            Item { id: 7, title: "PostgreSQL — 关系型数据库".into() },
            Item { id: 8, title: "Redis — 缓存系统".into() },
        ],
        _ => vec![
            Item { id: 9, title: "Docker — 容器化".into() },
            Item { id: 10, title: "Kubernetes — 编排".into() },
        ],
    };
    sleep(400).await;
    items
}

#[component]
fn Exercise() -> impl IntoView {
    // 第一步: 加载分类列表 (只加载一次)
    let categories = Resource::new(move || (), |_| async { fetch_categories().await });

    let selected_id = RwSignal::new(None::<u32>);
    let selected_name = RwSignal::new(String::new());

    // 第二步: 根据 selected_id 加载项目 (依赖链)
    let items = Resource::new(
        move || selected_id.get(),
        |id| async move {
            match id {
                Some(cat_id) => fetch_items(cat_id).await,
                None => Vec::new(),
            }
        },
    );

    let on_select = move |ev| {
        let val = event_target_value(&ev);
        if val.is_empty() {
            selected_id.set(None);
            selected_name.set(String::new());
        } else if let Ok(id) = val.parse::<u32>() {
            selected_id.set(Some(id));
            // 查找选中分类的名称
            if let Some(cats) = categories.get() {
                if let Some(cat) = cats.iter().find(|c| c.id == id) {
                    selected_name.set(cat.name.clone());
                }
            }
        }
    };

    view! {
        <div>
            <h2>"e199: Resource 链式依赖"</h2>

            <h3>"分类"</h3>
            <select on:change=on_select>
                <option value="">"-- 请选择分类 --"</option>
                {move || categories.get().map(|cats| {
                    cats.into_iter().map(|cat| view! {
                        <option value=cat.id>{cat.name}</option>
                    }).collect::<Vec<_>>()
                })}
            </select>

            <h3>
                "项目列表 "
                {move || if !selected_name.get().is_empty() {
                    format!("— {}", selected_name.get())
                } else { String::new() }}
            </h3>

            <Transition
                fallback=|| view! { <p>"⏳ 加载中…"</p> }
            >
                {move || items.get().map(|item_list| {
                    if item_list.is_empty() {
                        view! { <p>"请选择一个分类"</p> }.into_any()
                    } else {
                        item_list.into_iter().map(|item| view! {
                            <p>"📄 " {item.title}</p>
                        }).collect::<Vec<_>>().into_any()
                    }
                })}
            </Transition>
        </div>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(Exercise);
}

// ============================================================
// 参考答案 (思考后再看!)
// ============================================================
// <details>
// <summary>点击展开答案</summary>
//
// ### 核心思路
// 第一个 `Resource` 加载分类列表(无依赖, 启动即加载)。
// 选中分类后 `selected_id` 信号变化, 第二个 `Resource` 的 deps
// 函数返回新的 ID, 自动触发重新 fetch。
//
// ### 关键代码
// ```rust
// let categories = Resource::new(move || (), |_| fetch_categories());
// let items = Resource::new(
//     move || selected_id.get(),
//     |id| async move { fetch_items(id.unwrap_or(0)).await },
// );
// ```
// `<Transition fallback=|| view! { "加载中…" }>` 在切换分类时保留旧内容。
//
// ### 知识点
// - Resource 的 deps 函数返回新值时自动重新执行 fetcher
// - Transition 在重新加载时保持上次内容(比 Suspense 体验更平滑)
// - 链式依赖模式: 数据 → 选择 → 关联数据
//
// </details>
