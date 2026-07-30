use leptos::prelude::*;

#[component]
fn Exercise() -> impl IntoView {
    let (count, set_count) = signal(0);

    let do_set = move |_| set_count.set(100);
    let do_fn_call = move |_| set_count(200);
    let do_update = move |_| set_count.update(|n| *n += 1);
    let do_write = move |_| {
        let mut guard = set_count.write();
        *guard += 10;
    };

    view! {
        <div>
            <p>"当前值: " {count}</p>
            <button on:click=do_set>"set(100)"</button>
            <button on:click=do_fn_call>"fn_call(200)"</button>
            <button on:click=do_update>"update(+1)"</button>
            <button on:click=do_write>"write(+10)"</button>
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
