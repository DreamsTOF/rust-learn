// ============================================================
// 练习 e244: route_cache — 路由数据缓存
//
// 目标: 利用 Leptos 路由缓存机制保持组件状态，
//       导航离开再返回后数据不丢失
//
// 难度: ⭐⭐⭐
// 核心知识点: 路由缓存、Signal 持久性、RwSignal、StoredValue
// ============================================================

use leptos::prelude::*;
use leptos_router::components::{Router, Routes, Route, A};
use leptos_router::path;

// 计数器组件 — 带有可交互状态
// TODO: 使用 signal() 创建响应式计数器
//       在导航离开再返回后，观察计数是否保持
#[component]
fn Counter() -> impl IntoView {
    let (count, set_count) = signal(0);

    view! {
        <div style="border:2px solid #9C27B0;padding:16px;border-radius:8px;text-align:center;">
            <h2>"🔢 计数器"</h2>
            <p style="font-size:48px;font-weight:bold;margin:16px 0;">{count}</p>
            <div style="display:flex;gap:8px;justify-content:center;">
                <button on:click=move |_| *set_count.write() += 1
                    style="padding:8px 24px;font-size:18px;cursor:pointer;">
                    "➕ 增加"
                </button>
                <button on:click=move |_| *set_count.write() -= 1
                    style="padding:8px 24px;font-size:18px;cursor:pointer;">
                    "➖ 减少"
                </button>
                <button on:click=move |_| set_count.set(0)
                    style="padding:8px 24px;font-size:18px;cursor:pointer;">
                    "🔄 重置"
                </button>
            </div>
            <p style="margin-top:16px;color:#666;">
                "提示：导航到其他页面再返回，观察计数器状态"
            </p>
        </div>
    }
}

// 关于页面 — 说明缓存原理
#[component]
fn About() -> impl IntoView {
    view! {
        <div style="border:2px solid #2196F3;padding:16px;border-radius:8px;">
            <h2>"ℹ️ 关于页面"</h2>
            <p>"使用导航栏返回计数器页面，观察状态是否保持"</p>
        </div>
    }
}

// 设置页面 — 带有输入框，测试表单状态缓存
// TODO: 使用 signal 绑定输入框值，导航离开再回来验证状态
#[component]
fn Settings() -> impl IntoView {
    let (text, set_text) = signal(String::new());

    view! {
        <div style="border:2px solid #FF9800;padding:16px;border-radius:8px;">
            <h2>"⚙️ 设置页面"</h2>
            <div style="margin-top:12px;">
                <label>"输入框（测试状态保持）: "</label>
                <input
                    type="text"
                    value=text
                    on:input=move |ev| set_text.set(event_target_value(&ev))
                    style="padding:8px;width:200px;margin-left:8px;"
                />
            </div>
            <p style="margin-top:12px;color:#666;">
                "输入一些文字，切换到其他页面再回来"
            </p>
        </div>
    }
}

// ★ Leptos 默认缓存路由组件：
//   导航离开时，匹配的路由组件不会被销毁，
//   signal 和响应式系统保持活跃，
//   返回时组件重新渲染，但信号值保留
#[component]
fn Exercise() -> impl IntoView {
    view! {
        <Router>
            <nav>
                <A href="/">"🏠 首页（计数器） | "</A>
                <A href="/about">"ℹ️ 关于 | "</A>
                <A href="/settings">"⚙️ 设置"</A>
            </nav>
            <hr/>
            <Routes fallback=|| view! { <p>"404 页面未找到"</p> }>
                <Route path=path!("/") view=Counter/>
                <Route path=path!("/about") view=About/>
                <Route path=path!("/settings") view=Settings/>
            </Routes>
        </Router>
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
// ### 为什么状态会保持？
// Leptos 默认缓存路由组件：
// - 导航离开时，匹配的路由组件不会被销毁
// - signal 和响应式系统保持活跃
// - 返回时组件重新渲染，但信号值保留
//
// ### 高级缓存方案
// - `RwSignal<T>` 可在不同组件间共享缓存
// - `StoredValue<T>` 可用于存储不需要响应式的缓存数据
//
// ### 什么时候状态会丢失？
// - 组件被条件渲染完全卸载（如 <Show> 切换）
// - 强制刷新页面
//
// </details>
