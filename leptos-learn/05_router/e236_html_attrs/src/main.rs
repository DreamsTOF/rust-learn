use leptos::prelude::*;
use leptos_meta::Html;

fn main() {
    mount_to_body(Exercise);
}

#[component]
fn Exercise() -> impl IntoView {
    view! {
        <Html {..} lang="en" class="my-app" />
        <div>
            <p>"练习 236 (html_attrs) - 使用 leptos_meta::Html 设置 html 属性"</p>
        </div>
    }
}
