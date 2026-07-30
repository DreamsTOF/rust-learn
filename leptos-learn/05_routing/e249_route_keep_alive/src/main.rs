// ============================================================
// 练习 249: route_keep_alive — 路由 KeepAlive
//
// 目标: 离开路由时保存组件状态，回来时恢复
//
// 难度: ⭐⭐⭐⭐
// 核心知识点: 路由 KeepAlive
//
// TODO: 按照注释提示补全代码
// ============================================================

use std::collections::HashMap;
use leptos::prelude::*;
use leptos_router::components::*;
use leptos_router::hooks::{use_location, use_navigate};
use leptos_router::NavigateOptions;
use leptos_router::path;

// --- KeepAlive 存储定义 ---

/// 以路由 path 为 key 的状态存储
#[derive(Clone, Copy)]
struct KeepAliveStore(RwSignal<HashMap<String, String>>);

impl KeepAliveStore {
    fn new() -> Self {
        Self(RwSignal::new(HashMap::new()))
    }

    fn save(&self, key: &str, value: &str) {
        self.0.update(|m| {
            m.insert(key.to_string(), value.to_string());
        });
    }

    fn restore(&self, key: &str) -> Option<String> {
        self.0.with(|m| m.get(key).cloned())
    }
}

/// 为组件提供 KeepAlive 能力
fn use_keep_alive(key: &str, default: &str) -> (ReadSignal<String>, WriteSignal<String>) {
    let store = use_context::<KeepAliveStore>().expect("KeepAliveStore not provided");
    let initial = store.restore(key).unwrap_or_else(|| default.to_string());
    let (value, set_value) = signal(initial);

    // 状态变化时自动保存
    let key_owned = key.to_string();
    Effect::new(move || {
        store.save(&key_owned, &value.get());
    });

    // 组件卸载时保存最终状态
    let key_owned2 = key.to_string();
    on_cleanup(move || {
        store.save(&key_owned2, &value.get());
    });

    (value, set_value)
}

// --- 页面组件 ---

/// 表单页面 A
#[component]
fn PageA() -> impl IntoView {
    let (text, set_text) = use_keep_alive("pageA", "Hello Page A");
    let (textarea, set_textarea) = use_keep_alive("pageA-note", "Notes here...");
    let location = use_location();

    let on_input = move |ev| {
        set_text.set(event_target_value(&ev));
    };
    let on_textarea = move |ev| {
        set_textarea.set(event_target_value(&ev));
    };

    view! {
        <div style="padding: 1rem 0;">
            <h3>"Page A"</h3>
            <p style="color: #666;">"Current path: " {move || location.pathname.get()}</p>
            <div style="margin-bottom: 1rem;">
                <label>"Name:"</label>
                <input
                    type="text"
                    prop:value=move || text.get()
                    on:input=on_input
                    style="display: block; width: 100%; padding: 0.5rem; margin: 0.25rem 0; box-sizing: border-box;"
                />
            </div>
            <div style="margin-bottom: 1rem;">
                <label>"Notes:"</label>
                <textarea
                    prop:value=move || textarea.get()
                    on:input=on_textarea
                    style="display: block; width: 100%; height: 100px; padding: 0.5rem; margin: 0.25rem 0; box-sizing: border-box;"
                ></textarea>
            </div>
            <p><em>"Try navigating away and back — your input is preserved."</em></p>
            <A href="/page-b">"Go to Page B →"</A>
        </div>
    }
}

/// 表单页面 B
#[component]
fn PageB() -> impl IntoView {
    let (checked, set_checked) = use_keep_alive("pageB-check", "false");
    let (slider, set_slider) = use_keep_alive("pageB-slider", "50");
    let location = use_location();

    let is_checked = move || checked.get() == "true";

    let on_check = move |ev| {
        let val = event_target_checked(&ev);
        set_checked.set(if val { "true".to_string() } else { "false".to_string() });
    };
    let on_slider = move |ev| {
        set_slider.set(event_target_value(&ev));
    };

    view! {
        <div style="padding: 1rem 0;">
            <h3>"Page B"</h3>
            <p style="color: #666;">"Current path: " {move || location.pathname.get()}</p>
            <div style="margin-bottom: 1rem;">
                <label>
                    <input
                        type="checkbox"
                        prop:checked=is_checked
                        on:change=on_check
                    />
                    " Enable feature"
                </label>
            </div>
            <div style="margin-bottom: 1rem;">
                <label>"Volume: " {move || slider.get()}</label>
                <input
                    type="range"
                    min="0"
                    max="100"
                    prop:value=move || slider.get()
                    on:input=on_slider
                    style="display: block; width: 100%;"
                />
            </div>
            <p><em>"Your checkbox and slider values are preserved across navigations."</em></p>
            <A href="/page-a">"← Go to Page A"</A>
        </div>
    }
}

/// 布局组件
#[component]
fn Layout() -> impl IntoView {
    view! {
        <div style="max-width: 600px; margin: 0 auto; padding: 1rem; font-family: system-ui, sans-serif;">
            <h2>"Route KeepAlive"</h2>
            <nav style="display: flex; gap: 1rem; margin-bottom: 1rem;">
                <A href="/page-a">"Page A"</A>
                <A href="/page-b">"Page B"</A>
            </nav>
            <hr />
            <Outlet />
        </div>
    }
}

/// 重定向组件（在 setup 阶段执行导航）
#[component]
fn RedirectTo(path: &'static str) -> impl IntoView {
    let navigate = use_navigate();
    Effect::new(move || {
        navigate(path, NavigateOptions { replace: true, ..Default::default() });
    });
}

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| {
        let store = KeepAliveStore::new();
        provide_context(store);

        view! {
            <Router>
                <Routes fallback=|| "Not found.">
                    <ParentRoute path=path!("/") view=Layout>
                        <Route path=path!("page-a") view=PageA />
                        <Route path=path!("page-b") view=PageB />
                        <Route path=path!("") view=|| view! { <RedirectTo path="/page-a" /> } />
                    </ParentRoute>
                </Routes>
            </Router>
        }
    });
}

// ============================================================
// 参考答案 (思考后再看!)
// ============================================================
// <details>
// <summary>点击展开答案</summary>
//
// ### 关键思路
// 1. `KeepAliveStore` 是 `RwSignal<HashMap<String, String>>` 的封装，以路由 path 为 key 存储状态
// 2. `use_keep_alive()` 自定义 hook：
//    - 挂载时从 store 恢复状态（如无则使用默认值）
//    - 状态变化时自动保存到 store
//    - `on_cleanup()` 确保组件卸载时保存最终状态
// 3. 在 `<Router>` 内通过 `provide_context()` 注入 store，确保所有子路由可访问
// 4. 用 key 区分不同组件的状态（如 "pageA"、"pageB-check"），避免冲突
//
// ### 知识点
// - `provide_context()` / `use_context()` 跨路由共享状态
// - `RwSignal` 可变状态管理
// - `on_cleanup()` 组件卸载回调
// - `Effect::new()` 响应式保存
// - 自定义 hook `use_keep_alive()` 封装复用逻辑
//
// </details>
