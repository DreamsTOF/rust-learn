// ============================================================
// 练习 e195: set_interval 轮询
//
// 核心知识点:
//   - set_interval 定时重复执行
//   - 定时刷新数据
//   - 与 Resource 或信号结合
//
// 难度: ⭐⭐
// ============================================================

use leptos::prelude::*;
use std::time::Duration;

#[component]
fn Exercise() -> impl IntoView {
    let (time, set_time) = signal(String::from("等待更新..."));
    let (count, set_count) = signal(0u32);

    // TODO: 使用 set_interval 每秒更新时间和计数
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
