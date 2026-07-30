use leptos::prelude::*;
use leptos_router::components::*;
use leptos_router::hooks::query_signal;
use leptos_router::path;

const TOTAL_PAGES: i32 = 10;

fn page_content(page: i32) -> Vec<String> {
    let start = (page - 1) * 3 + 1;
    (start..start + 3).map(|i| format!("项目 {}", i)).collect()
}

#[component]
fn Exercise() -> impl IntoView {
    let (page, set_page) = query_signal::<i32>("page");
    let current = move || {
        let p = page().unwrap_or(1);
        p.clamp(1, TOTAL_PAGES)
    };
    let items = move || page_content(current());
    let go_to = move |p: i32| {
        let p = p.clamp(1, TOTAL_PAGES);
        set_page.set(Some(p));
    };

    view! {
        <Router>
            <h1>"e234: 分页路由"</h1>

            <div>
                <ul>
                    {move || items().into_iter().map(|item| view! { <li>{item}</li> }).collect_view()}
                </ul>
            </div>

            <div style="margin-top: 12px;">
                <button on:click=move |_| go_to(current() - 1) disabled=move || current() == 1>
                    "上一页"
                </button>

                {move || {
                    let cur = current();
                    (1..=TOTAL_PAGES).map(|p| {
                        let style = if p == cur { "font-weight: bold; margin: 0 4px;" } else { "margin: 0 4px;" };
                        view! {
                            <button style=style on:click=move |_| go_to(p)>
                                {p}
                            </button>
                        }
                    }).collect_view()
                }}

                <button on:click=move |_| go_to(current() + 1) disabled=move || current() == TOTAL_PAGES>
                    "下一页"
                </button>
            </div>

            <p>"当前页: " {move || current()} " / " {TOTAL_PAGES}</p>
            <p>"(页码已同步到 URL: ?page=N)"</p>

            <Routes fallback=|| "页面未找到">
                <Route path=path!("") view=|| view! { <p>"使用分页控件浏览"</p> }/>
            </Routes>
        </Router>
    }
}

fn main() {
    mount_to_body(Exercise);
}
