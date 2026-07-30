// ============================================================
// 练习 248: route_search_sync — 搜索同步 URL
//
// 目标: 搜索框内容 ↔ URL query 双向同步
//
// 难度: ⭐⭐⭐
// 核心知识点: 搜索同步 URL
//
// TODO: 按照注释提示补全代码
// ============================================================

use leptos::prelude::*;
use leptos_router::components::*;
use leptos_router::hooks::*;
use leptos_router::NavigateOptions;
use leptos_router::path;

/// 简易 URL 编码
fn urlencoding(s: &str) -> String {
    let mut result = String::new();
    for c in s.chars() {
        match c {
            'a'..='z' | 'A'..='Z' | '0'..='9' | '-' | '_' | '.' | '~' => result.push(c),
            ' ' => result.push('+'),
            _ => {
                let mut buf = [0u8; 4];
                let encoded_str = c.encode_utf8(&mut buf);
                for byte in encoded_str.as_bytes() {
                    result.push_str(&format!("%{:02X}", byte));
                }
            }
        }
    }
    result
}

/// 搜索页面组件
#[component]
fn SearchPage() -> impl IntoView {
    let query = use_query_map();
    let navigate = use_navigate();

    // 从 URL 读取初始搜索词
    let initial = query.read().get("q").unwrap_or_default();
    let (search_text, set_search_text) = signal(initial);

    // --- 方向①: URL → 输入框 ---
    // 当 query 变化时同步到输入框（排除自身导航导致的循环）
    Effect::new(move || {
        let url_value = query.read().get("q").unwrap_or_default();
        let current = search_text.get();
        if url_value != current {
            set_search_text.set(url_value);
        }
    });

    // --- 方向②: 输入框 → URL ---
    // 用户输入时更新 URL query
    let on_input = move |ev| {
        let new_val = event_target_value(&ev);
        set_search_text.set(new_val.clone());

        let encoded = urlencoding(&new_val);
        navigate(&format!("/?q={}", encoded), NavigateOptions::default());
    };

    // 模拟搜索结果
    let results = move || {
        let q = search_text.get();
        if q.trim().is_empty() {
            vec!["Start typing to search...".to_string()]
        } else {
            (1..=5)
                .map(|i| format!("Result {} for \"{}\"", i, q))
                .collect()
        }
    };

    view! {
        <div style="font-family: system-ui, sans-serif; max-width: 600px; margin: 0 auto; padding: 1rem;">
            <h2>"Search Sync"</h2>

            <div style="margin-bottom: 1rem;">
                <input
                    type="search"
                    placeholder="Type to search..."
                    prop:value=move || search_text.get()
                    on:input=on_input
                    style="width: 100%; padding: 0.75rem; font-size: 1rem; border: 2px solid #ddd; border-radius: 6px; box-sizing: border-box;"
                />
            </div>

            <div>
                <p style="color: #666; font-size: 0.9rem;">
                    <em>
                        "URL: " <code>{move || format!("/?q={}", search_text.get())}</code>
                    </em>
                </p>
                <ul>
                    {move || {
                        results()
                            .into_iter()
                            .map(|r| view! { <li style="padding: 0.25rem 0;">{r}</li> })
                            .collect_view()
                    }}
                </ul>
            </div>

            <hr />
            <h3>"Try it:"</h3>
            <ol>
                <li>"Type in the search box — URL updates."</li>
                <li>"Use browser back/forward — input updates."</li>
            </ol>
        </div>
    }
}

#[component]
fn App() -> impl IntoView {
    view! {
        <Router>
            <Routes fallback=|| "Not found.">
                <Route path=path!("") view=SearchPage />
            </Routes>
        </Router>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App);
}

// ============================================================
// 参考答案 (思考后再看!)
// ============================================================
// <details>
// <summary>点击展开答案</summary>
//
// ### 关键思路
// 1. **URL → 输入框**: 用 `Effect` 监听 `use_query_map()` 的变化，同步到本地信号
// 2. **输入框 → URL**: `on:input` 事件中调用 `use_navigate()` 更新 query
// 3. 通过比较新旧值避免双向更新循环
// 4. URL 编码确保特殊字符正确处理
//
// ### 知识点
// - `use_query_map()` 读取 URL 查询参数
// - `use_navigate()` 编程式导航更新 URL
// - `Effect::new()` 响应式同步
// - `event_target_value()` 获取输入框值
// - 双向数据绑定与循环防护
//
// </details>
