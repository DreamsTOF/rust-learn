// ============================================================
// Exercise 109 - Answer: pass_through_props
// ============================================================

use leptos::prelude::*;

#[component]
fn Panel(
    #[prop(into)]
    title: String,
    #[prop(into)]
    class: String,
    #[prop(into)]
    style: String,
    children: Children,
) -> impl IntoView {
    view! {
        <div class={class} style={style}>
            <h3>{title}</h3>
            <div class="panel-body">
                {children()}
            </div>
        </div>
    }
}

#[component]
fn App() -> impl IntoView {
    view! {
        <h3>"练习 109: pass_through_props"</h3>
        <Panel
            title="第一个面板"
            class="primary"
            style="border: 2px solid #4CAF50; padding: 10px;"
        >
            <p>"这个面板有 class 和 style"</p>
        </Panel>
    }
}

fn main() {
    mount_to_body(App);
}
