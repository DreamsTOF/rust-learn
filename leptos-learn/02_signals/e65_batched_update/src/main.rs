use leptos::prelude::*;

fn main() {
    mount_to_body(move || {
        let (a, set_a) = signal(0);
        let (b, set_b) = signal(0);
        let (run_count, set_run_count) = signal(0);

        Effect::new(move || {
            let _a = a();
            let _b = b();
            set_run_count.update(|n| *n += 1);
        });

        // TODO:
        // 1. 点「分开更新」→ Effect 执行 2 次（run_count +2）
        // 2. 点「批量更新」→ Effect 执行 1 次（run_count +1）

        view! {
            <p>"a: " {a} " | b: " {b}</p>
            <p>"Effect 执行次数: " {run_count}</p>
            <button on:click=move |_| {
                set_a(a() + 1);
                set_b(b() + 1);
            }>"🔸 分开更新（2 次 Effect）"</button>
            <button on:click=move |_| {
                batch(|| {
                    set_a(a() + 10);
                    set_b(b() + 10);
                });
            }>"🔹 批量更新（1 次 Effect）"</button>
            <details>
                <summary>"💡 答案与解释"</summary>
                <p>"batch() 将多次信号更新合并为一次通知，"</p>
                <p>"Effect 只重新执行一次，减少重复计算，提升性能。"</p>
            </details>
        }
    });
}
