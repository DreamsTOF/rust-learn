use leptos::prelude::*;

fn main() {
    mount_to_body(move || {
        let (count, set_count) = signal(0);

        // Effect 每次重新执行时通过 tracing 记录日志
        Effect::new(move || {
            let n = count();
            tracing::info!("[Effect Debug] count = {}. Effect 重新执行！", n);
        });

        // TODO:
        // 1. 多次点击按钮，观察浏览器控制台中 tracing::info! 的输出
        // 2. 每次 set_count 都会触发 Effect 重新执行，日志逐条增加

        view! {
            <p>"count: " {count}</p>
            <button on:click=move |_| set_count(count() + 1)>"+1"</button>
            <button on:click=move |_| set_count(count() + 5)>"+5"</button>
            <p>"📋 打开浏览器控制台 (F12)，观察 tracing 日志输出"</p>
            <details>
                <summary>"💡 答案与解释"</summary>
                <p>"在 Effect 中添加 tracing::info! 可以精确观察其重新执行的次数和时机，"</p>
                <p>"是调试响应式性能、诊断无限循环的有效手段。"</p>
            </details>
        }
    });
}
