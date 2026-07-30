// ============================================================
// 练习 247: route_tab_persist — 标签页持久化
//
// 目标: Tab 切换保持滚动位置和状态
//
// 难度: ⭐⭐⭐
// 核心知识点: 标签页持久化
//
// TODO: 按照注释提示补全代码
// ============================================================

use std::collections::HashMap;
use leptos::prelude::*;
use leptos::html::Div;
use leptos_router::components::*;
use leptos_router::hooks::*;
use leptos_router::NavigateOptions;
use leptos_router::path;

const TABS: &[(&str, &str)] = &[
    ("tab1", "Tab 1"),
    ("tab2", "Tab 2"),
    ("tab3", "Tab 3"),
];

/// 生成长列表内容用于测试滚动
fn long_list(prefix: &str) -> Vec<String> {
    (1..=50).map(|i| format!("{} — item #{}", prefix, i)).collect()
}

// --- 核心组件 ---

#[component]
fn TabPersistence() -> impl IntoView {
    let query = use_query_map();
    let navigate = use_navigate();

    // 当前激活的 tab id，默认 tab1
    let active = move || {
        query
            .read()
            .get("tab")
            .unwrap_or_default()
    };

    // 每个 tab 对应的滚动容器 ref
    let tab_refs = RwSignal::new(HashMap::<String, NodeRef<Div>>::new());
    {
        let mut refs = tab_refs.write();
        for (id, _) in TABS {
            refs.insert(id.to_string(), NodeRef::<Div>::new());
        }
    }

    // 持久化的滚动位置 { tab_id → scrollTop }
    let saved_scrolls = RwSignal::new(HashMap::<String, i32>::new());

    // --- 响应式恢复滚动位置 ---
    // 当 active tab 变化时，恢复其滚动位置
    Effect::new(move || {
        let current = active();
        // 延迟一帧等待 DOM 渲染
        request_animation_frame(move || {
            if let Some(node_ref) = tab_refs.read().get(&current) {
                if let Some(el) = node_ref.get() {
                    let saved = saved_scrolls.read().get(&current).copied().unwrap_or(0);
                    el.set_scroll_top(saved);
                }
            }
        });
    });

    // 保存当前 tab 滚动位置的辅助函数
    let save_current_scroll = move || {
        let current = active();
        if let Some(node_ref) = tab_refs.read().get(&current) {
            if let Some(el) = node_ref.get() {
                saved_scrolls.update(|m| {
                    m.insert(current, el.scroll_top());
                });
            }
        }
    };

    view! {
        <div style="font-family: system-ui, sans-serif; max-width: 700px; margin: 0 auto; padding: 1rem;">
            <h2>"Tab Persistence"</h2>

    // Tab 按钮栏
            <div style="display: flex; gap: 0;">
                {TABS
                    .iter()
                    .map(|(id, label)| {
                        let id_str = *id;
                        let is_active = move || active() == id_str;
                        let nav = navigate.clone();
                        view! {
                            <button
                                class="tab-btn"
                                class:active=is_active
                                on:click=move |_| {
                                    save_current_scroll();
                                    nav(&format!("/?tab={}", id_str), NavigateOptions::default());
                                }
                            >
                                {*label}
                            </button>
                        }
                    })
                    .collect_view()}
            </div>

            // Tab 内容面板（全部渲染，仅切换 display）
            {TABS
                    .iter()
                    .map(|(id, label)| {
                        let id_str = *id;
                        let items = long_list(label);
                        let node_ref = tab_refs.read().get(id_str).unwrap().clone();
                        let is_visible = move || active() == id_str;

                    view! {
                        <div
                            class="tab-content"
                            style:display=move || {
                                if is_visible() { "block" } else { "none" }
                            }
                            node_ref=node_ref
                        >
                            <h3>{*label}</h3>
                            {items
                                .into_iter()
                                .map(|item| {
                                    view! { <div class="item">{item}</div> }
                                })
                                .collect_view()}
                        </div>
                    }
                })
                .collect_view()}
        </div>
    }
}

#[component]
fn App() -> impl IntoView {
    view! {
        <Router>
            <Routes fallback=|| "Not found.">
                <Route path=path!("") view=TabPersistence />
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
// 1. 当前 tab 通过 URL query `?tab=tab1` 驱动，用 `use_query_map()` 读取
// 2. 所有 tab 内容同时渲染，用 `style:display` 切换可见性（DOM 不卸载）
// 3. 为每个 tab 分配一个 `NodeRef<Div>` 引用其滚动容器
// 4. 切换前保存当前 tab 的 `scroll_top()`，切换后用 `request_animation_frame`
//    延迟一帧调用 `set_scroll_top()` 恢复目标 tab 的滚动位置
// 5. 滚动位置存储在 `RwSignal<HashMap<String, i32>>` 中
//
// ### 知识点
// - `use_query_map()` 响应式读取 URL 查询参数
// - `use_navigate()` 更新 URL 切换 tab
// - `NodeRef::get()` 获取 DOM 元素
// - `Element::scroll_top()` / `set_scroll_top()` 读写滚动位置
// - `request_animation_frame()` 在下一次渲染帧执行回调
//
// </details>
