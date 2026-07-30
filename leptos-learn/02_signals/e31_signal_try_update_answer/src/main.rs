use leptos::prelude::*;

#[component]
fn Exercise() -> impl IntoView {
    let (count, set_count) = signal(0);
    let (last_old, set_last_old) = signal(Option::<i32>::None);

    let do_try_update = move |_| {
        let old = set_count.try_update(|n| {
            let prev = *n;
            *n = *n + 10;
            prev
        });
        set_last_old.set(old);
    };

    view! {
        <div>
            <p>"当前值: " {count}</p>
            <p>
                "上一次旧值: "
                {move || match last_old.get() {
                    Some(v) => format!("{}", v),
                    None => "暂无".to_string(),
                }}
            </p>
            <button on:click=do_try_update>"try_update (加 10)"</button>
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
