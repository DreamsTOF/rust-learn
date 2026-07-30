use leptos::prelude::*;
use leptos_meta::*;

#[component]
fn Card(title: &'static str, children: Children) -> impl IntoView {
    view! {
        <>
            <Style>
                ".card { border: 1px solid #ddd; border-radius: 8px; padding: 16px; margin: 8px 0; box-shadow: 0 2px 4px rgba(0,0,0,0.1); }"
                ".card h3 { margin: 0 0 8px 0; color: #333; }"
                ".card p { color: #666; }"
            </Style>
            <div class="card">
                <h3>{title}</h3>
                {children()}
            </div>
        </>
    }
}

#[component]
fn PrimaryButton(label: &'static str) -> impl IntoView {
    view! {
        <>
            <Style>
                ".primary-btn { background: #2196F3; color: white; border: none; padding: 8px 16px; border-radius: 4px; cursor: pointer; font-size: 14px; }"
                ".primary-btn:hover { background: #1976D2; }"
            </Style>
            <button class="primary-btn">{label}</button>
        </>
    }
}

#[component]
fn Exercise() -> impl IntoView {
    view! {
        <div>
            <h1>"e234: Style 组件"</h1>
            <p>"每个组件通过 <Style> 定义自己的 CSS"</p>
            <Card title="卡片标题">
                <p>"这是卡片内容，样式由组件内的 <Style> 定义"</p>
                <PrimaryButton label="确认"/>
            </Card>
            <Card title="另一张卡片">
                <p>"多个卡片实例共享组件定义的样式"</p>
                <PrimaryButton label="取消"/>
            </Card>
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
