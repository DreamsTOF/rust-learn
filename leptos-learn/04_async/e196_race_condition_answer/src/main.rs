// ============================================================
// 练习 196: race_condition — 竞态条件处理
//
// 目标: 使用版本号机制避免异步竞态条件
//
// 难度: ⭐⭐⭐⭐
// 核心知识点: 竞态条件处理
//
// TODO:
//   1. 实现 sleep 工具函数(futures::channel::oneshot + set_timeout)
//   2. mock_search: 模拟网络请求(偶数字符串延迟短、奇数字符串延迟长)
//   3. 使用版本号(RwSignal<u32>)标记每次请求
//   4. 异步回调返回后检查版本号是否仍匹配
//   5. 渲染搜索框和结果列表
// ============================================================

use std::time::Duration;
use futures::channel::oneshot;
use leptos::prelude::*;
use leptos::task::spawn_local;

/// 异步延迟工具函数
async fn sleep(ms: u64) {
    let (tx, rx) = oneshot::channel::<()>();
    set_timeout(move || { let _ = tx.send(()); }, Duration::from_millis(ms));
    let _ = rx.await;
}

#[derive(Debug, Clone, PartialEq)]
struct SearchItem {
    id: u32,
    label: String,
}

/// 模拟搜索: 偶数字符串 300ms, 奇数字符串 1500ms
async fn mock_search(query: &str) -> Vec<SearchItem> {
    let delay = if query.len() % 2 == 0 { 300 } else { 1500 };
    sleep(delay).await;
    (0..3)
        .map(|i| SearchItem {
            id: i,
            label: format!("{} - 结果 #{}", query, i + 1),
        })
        .collect()
}

#[component]
fn Exercise() -> impl IntoView {
    let query = RwSignal::new(String::new());
    // 版本号: 每次发起新请求时递增
    let version = RwSignal::new(0u32);
    let items = RwSignal::new(Vec::<SearchItem>::new());
    let loading = RwSignal::new(false);

    let on_search = move |ev| {
        let q = event_target_value(&ev);
        query.set(q.clone());

        if q.is_empty() {
            items.set(Vec::new());
            loading.set(false);
            return;
        }

        // 递增版本号 → 标记此请求为最新
        let ver = version.get() + 1;
        version.set(ver);
        loading.set(true);

        spawn_local(async move {
            let data = mock_search(&q).await;
            // 版本号检查: 只有最新版本的结果才更新 UI
            if version.get() == ver {
                items.set(data);
                loading.set(false);
            }
            // else: 结果已过期, 丢弃
        });
    };

    view! {
        <div>
            <h2>"e196: 竞态条件 —— 版本号防护"</h2>
            <p>"提示: 偶数长度查询 300ms, 奇数长度 1500ms。快速交替输入可触发竞态。"</p>
            <input
                type="text"
                placeholder="输入搜索词…"
                on:input=on_search
                prop:value=query
            />
            <p>
                {move || if loading.get() { "搜索中…" } else { "就绪" }}
            </p>
            <ul>
                {move || items.get().into_iter().map(|it| view! { <li>{it.label}</li> }).collect::<Vec<_>>()}
            </ul>
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
// 使用 `RwSignal<u32>` 作为版本号，每次发起请求前递增。
// 异步回调完成后检查当前版本号是否仍等于发起时的版本号——
// 若不等说明已有更新的请求，直接丢弃过期结果。
//
// ### 关键代码
// ```rust
// let ver = version.get() + 1;
// version.set(ver);
// spawn_local(async move {
//     let data = mock_search(&q).await;
//     if version.get() == ver {   // 仍是最新?
//         items.set(data);
//     }
// });
// ```
//
// ### 知识点
// - 竞态条件: 快请求(300ms)先返回但数据已过时, 被慢请求(1500ms)覆盖
// - 版本号模式: 轻量、无外部依赖、适合 Web 前端场景
// - 替代方案: AbortController / `AbortSignal` 中断旧请求
//
// </details>
