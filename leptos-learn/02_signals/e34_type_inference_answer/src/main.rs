use leptos::prelude::*;

#[component]
fn Exercise() -> impl IntoView {
    let (count, set_count) = signal(0);
    let (_count2, _set2) = signal::<i32>(0);
    let (count_anno, set_count_anno): (ReadSignal<i32>, WriteSignal<i32>) = signal(42);
    let (text, set_text) = signal("hello");

    let do_update = move |_| {
        set_count.update(|n| *n += 1);
        set_count_anno.update(|n| *n += 1);
        set_text.set("world");
    };

    view! {
        <div>
            <p>"隐式 count: " {count}</p>
            <p>"注解 count_anno: " {count_anno}</p>
            <p>"文字 text: " {text}</p>
            <button on:click=do_update>"全部更新"</button>
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
