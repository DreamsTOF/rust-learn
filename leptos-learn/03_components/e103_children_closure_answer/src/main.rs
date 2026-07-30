use leptos::prelude::*;

#[component]
fn FlexBox(children: ChildrenFn) -> impl IntoView {
    view! {
        <div style="display:flex;gap:10px;padding:10px;border:2px solid #e67e22;border-radius:8px;">
            <div style="flex:1;background:#fdf2e9;padding:8px;border-radius:4px;">
                <p><strong>"区域 A"</strong></p>
                {children()}
            </div>
            <div style="flex:1;background:#fef9e7;padding:8px;border-radius:4px;">
                <p><strong>"区域 B"</strong></p>
                {children()}
            </div>
        </div>
    }
}

#[component]
fn Exercise() -> impl IntoView {
    let (count, set_count) = signal(0);
    view! {
        <button on:click=move |_| set_count.update(|n| *n += 1)>
            "点击增加: " {count}
        </button>
        <FlexBox>
            <p>"计数 = " {move || count.get()}</p>
            <p>"同一 children 渲染到两个区域"</p>
        </FlexBox>
    }
}

fn main() {
    mount_to_body(Exercise);
}
