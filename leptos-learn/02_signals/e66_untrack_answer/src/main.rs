use leptos::prelude::*;

fn main() {
    mount_to_body(move || {
        let (count, set_count) = signal(0);
        let (trigger, set_trigger) = signal(());
        let (display, set_display) = signal("等待触发..".to_string());

        Effect::new(move || {
            let _ = trigger();
            let n = untrack(move || count());
            set_display(format!("untrack 读取 count = {}", n));
        });

        view! {
            <p>"count: " {count}</p>
            <p>"Effect: " {display}</p>
            <button on:click=move |_| set_count(count() + 1)>"修改 count（不触发 Effect）"</button>
            <button on:click=move |_| set_trigger(())>"触发 Effect（读取 count 最新值）"</button>
        }
    });
}
