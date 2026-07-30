use leptos::prelude::*;

fn main() {
    mount_to_body(move || {
        let (count, set_count) = signal(0);
        let (trigger, set_trigger) = signal(());
        let (display, set_display) = signal("等待触发..".to_string());

        // Effect 只追踪 trigger，通过 with_untracked 读取 count
        Effect::new(move || {
            let _ = trigger();
            let n = count.with_untracked(|&n| n);
            set_display(format!("with_untracked 读取 count = {}", n));
        });

        // TODO:
        // 1. 点「修改 count」→ count 变化，但 Effect 不执行
        // 2. 点「触发 Effect」→ 读取 count 最新值

        view! {
            <p>"count: " {count}</p>
            <p>"Effect: " {display}</p>
            <button on:click=move |_| set_count(count() + 1)>"修改 count（不触发 Effect）"</button>
            <button on:click=move |_| set_trigger(())>"触发 Effect（读取 count 最新值）"</button>
            <details>
                <summary>"💡 答案与解释"</summary>
                <p>"with_untracked() 方法（来自 WithUntracked trait）与 untrack() 类似，"</p>
                <p>"读取信号值但不建立响应式依赖。"</p>
                <p>"适合在 Effect 中读取"快照"值而不被其变化反复触发。"</p>
            </details>
        }
    });
}
