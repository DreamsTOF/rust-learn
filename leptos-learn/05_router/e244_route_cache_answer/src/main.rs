// ============================================================
// Exercise 244 - Answer: route_cache
// ============================================================

use leptos::prelude::*;
use leptos_router::components::{Router, Routes, Route, A};
use leptos_router::path;

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

#[component]
fn About() -> impl IntoView {
    view! {
        <div style="border:2px solid #2196F3;padding:16px;border-radius:8px;">
            <h2>"ℹ️ 关于页面"</h2>
            <p>"使用导航栏返回计数器页面，观察状态是否保持"</p>
        </div>
    }
}

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
