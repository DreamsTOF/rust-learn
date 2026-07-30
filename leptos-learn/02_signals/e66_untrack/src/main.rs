use leptos::prelude::*;

fn main() {
    mount_to_body(move || {
        let (count, set_count) = signal(0);
        let (trigger, set_trigger) = signal(());
        let (display, set_display) = signal("等待触发..".to_string());

        // Effect 只追踪 trigger，通过 untrack 读取 count
        Effect::new(move || {
            let _ = trigger();
            let n = untrack(move || count());
            set_display(format!("untrack 读取 count = {}", n));
        });

        // TODO:
        // 1. 点「修改 count」→ count 变化，但 Effect 不执行（display 不变）
        // 2. 点「触发 Effect」→ 读取 count 最新值并更新 display

        view! {
            <p>"count: " {count}</p>
            <p>"Effect: " {display}</p>
            <button on:click=move |_| set_count(count() + 1)>"修改 count（不触发 Effect）"</button>
            <button on:click=move |_| set_trigger(())>"触发 Effect（读取 count 最新值）"</button>
            <details>
                <summary>"💡 答案与解释"</summary>
                <p>"untrack() 读取信号值但不建立响应式依赖。"</p>
                <p>"Effect 不会因该信号变化而重新执行，适合读取"一次性"值。"</p>
            </details>
        }
    });
}
