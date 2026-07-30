use leptos::prelude::*;

fn main() {
    mount_to_body(move || {
        let (count, set_count) = signal(0);

        Effect::new(move || {
            let n = count();
            tracing::info!("[Effect Debug] count = {}. Effect 重新执行！", n);
        });

        view! {
            <p>"count: " {count}</p>
            <button on:click=move |_| set_count(count() + 1)>"+1"</button>
            <button on:click=move |_| set_count(count() + 5)>"+5"</button>
            <p>"📋 打开浏览器控制台 (F12)，观察 tracing 日志输出"</p>
        }
    });
}
