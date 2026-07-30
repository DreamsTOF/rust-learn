// ============================================================
// 练习 e138: request_animation_frame - 答案
// ============================================================

use leptos::prelude::*;

#[component]
fn RafCounter() -> impl IntoView {
    let (count, set_count) = signal(0i32);

    fn frame_loop(c: WriteSignal<i32>) {
        c.update(|n| *n += 1);
        request_animation_frame(move || frame_loop(c));
    }

    request_animation_frame(move || frame_loop(set_count));

    view! {
        <div>
            <h2>"练习 e138: requestAnimationFrame"</h2>
            <p>"帧计数 (约 60fps 快速递增)"</p>
            <p style="font-size: 3rem; font-weight: bold;">{count}</p>
        </div>
    }
}

fn main() {
    mount_to_body(RafCounter);
}
