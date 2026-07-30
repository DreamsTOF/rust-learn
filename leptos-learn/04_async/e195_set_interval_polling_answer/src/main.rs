// ============================================================
// Exercise 195 - set_interval Polling
// ============================================================

use leptos::prelude::*;
use std::time::Duration;

#[component]
fn Exercise() -> impl IntoView {
    let (time, set_time) = signal(String::from("等待更新..."));
    let (count, set_count) = signal(0u32);

    set_interval(move || {
        set_time.set(format!("当前时间: {:?}", std::time::SystemTime::now()));
        set_count.update(|v| *v += 1);
    }, Duration::from_secs(1));

    view! {
        <div>
            <p>"练习 195 — set_interval 轮询 (set_interval_polling)"</p>
            <p>{move || time.get()}</p>
            <p>"已更新 " {move || count.get()} " 次"</p>
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
