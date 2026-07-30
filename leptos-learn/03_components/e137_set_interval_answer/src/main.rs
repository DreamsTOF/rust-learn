// ============================================================
// 练习 e137: set_interval - 答案
// ============================================================

use leptos::prelude::*;
use std::time::Duration;

#[component]
fn SetInterval() -> impl IntoView {
    let (count, set_count) = signal(0);

    set_interval(
        move || set_count.update(|n| *n += 1),
        Duration::from_secs(1),
    );

    view! {
        <div>
            <h2>"练习 e137: set_interval 定时器"</h2>
            <p>"每秒 +1"</p>
            <p style="font-size: 3rem; font-weight: bold;">{count}</p>
        </div>
    }
}

fn main() {
    mount_to_body(SetInterval);
}
