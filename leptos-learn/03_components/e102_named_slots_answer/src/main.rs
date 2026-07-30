use leptos::prelude::*;

#[component]
fn BlogPost<T, M>(
    title: T,
    meta: M,
    children: Children,
) -> impl IntoView
where
    T: Fn() -> AnyView + Send + Sync + 'static,
    M: Fn() -> AnyView + Send + Sync + 'static,
{
    view! {
        <article style="border:1px solid #ddd;padding:16px;margin:10px 0;border-radius:8px;">
            <header style="border-bottom:1px solid #eee;padding-bottom:8px;margin-bottom:8px;">
                {title()}
            </header>
            <div style="color:#888;font-size:0.9em;margin:8px 0;">
                {meta()}
            </div>
            <section>{children()}</section>
        </article>
    }
}

#[component]
fn Exercise() -> impl IntoView {
    view! {
        <BlogPost
            title=move || view! { <h1>"Rust 学习笔记"</h1> }.into_any()
            meta=move || view! { <span>"发布于 2024-01-15"</span> }.into_any()
        >
            <p>"所有权系统是 Rust 最独特的特性之一。"</p>
            <p>"它让 Rust 在不使用垃圾回收器的前提下保证内存安全。"</p>
        </BlogPost>
    }
}

fn main() {
    mount_to_body(Exercise);
}
