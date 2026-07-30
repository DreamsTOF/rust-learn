// ============================================================
// Exercise 214 - Answer: params_parse — use_params() 自动解析
// ============================================================

use leptos::prelude::*;
use leptos_router::components::*;
use leptos_router::hooks::use_params;
use leptos_router::params::Params;
use leptos_router::path;

#[derive(Params, Debug, Clone, PartialEq)]
struct UserParams {
    id: Option<u32>,
    tab: Option<String>,
}

#[component]
fn Home() -> impl IntoView {
    view! {
        <h2>"首页"</h2>
        <p><a href="/user/42">"用户 42"</a></p>
        <p><a href="/user/99/posts">"用户 99 的帖子"</a></p>
    }
}

#[component]
fn User() -> impl IntoView {
    let params = use_params::<UserParams>();

    view! {
        <h2>"用户页面"</h2>
        <p>
            {move || match params.get().as_ref() {
                Ok(p) => format!("用户 ID: {:?}, 标签: {:?}", p.id, p.tab),
                Err(e) => format!("参数解析失败: {}", e),
            }}
        </p>
        <p><a href="/">"返回首页"</a></p>
    }
}

#[component]
fn Exercise() -> impl IntoView {
    view! {
        <Router>
            <nav>
                <a href="/">"首页"</a>
            </nav>
            <main>
                <Routes fallback=|| "页面未找到">
                    <Route path=path!("/") view=Home/>
                    <Route path=path!("/user/:id/:tab?") view=User/>
                </Routes>
            </main>
        </Router>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(Exercise);
}
