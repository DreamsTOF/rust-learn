use leptos::prelude::*;

#[component]
fn Panel(children: Children) -> impl IntoView {
    view! {
        <div style="border:2px solid #4a90d9;padding:20px;border-radius:10px;margin:10px 0;">
            <h3>"📦 Panel 组件"</h3>
            <hr/>
            {children()}
        </div>
    }
}

#[component]
fn Exercise() -> impl IntoView {
    view! {
        <Panel>
            <p>"这是 Panel 内部的内容"</p>
            <p>"所有子节点都会被渲染到 Panel 的 children 插槽中"</p>
        </Panel>
    }
}

fn main() {
    mount_to_body(Exercise);
}
