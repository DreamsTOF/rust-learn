use leptos::prelude::*;

#[component]
fn DisplayPanel(count: ReadSignal<i32>) -> impl IntoView {
    view! {
        <fieldset>
            <legend>"只读面板 (ReadSignal)"</legend>
            <p>"当前值: " {count}</p>
        </fieldset>
    }
}

#[component]
fn ControlPanel(set_count: WriteSignal<i32>) -> impl IntoView {
    view! {
        <fieldset>
            <legend>"只写面板 (WriteSignal)"</legend>
            <button on:click=move |_| set_count.update(|n| *n += 1)>"+1"</button>
            <button on:click=move |_| set_count.update(|n| *n -= 1)>"-1"</button>
            <button on:click=move |_| set_count.set(0)>"重置"</button>
        </fieldset>
    }
}

#[component]
fn Exercise() -> impl IntoView {
    let (count, set_count) = signal(0);

    view! {
        <div>
            <p>"ReadSignal 只允许读取，WriteSignal 只允许写入。"</p>
            <DisplayPanel count=count />
            <ControlPanel set_count=set_count />
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
