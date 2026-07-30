// ============================================================
// 练习 e138: request_animation_frame
//
// 目标: 使用 request_animation_frame 实现动画帧循环
//
// 难度: ⭐⭐⭐
// 核心知识点: request_animation_frame
//
// TODO: 利用 request_animation_frame 递归调用自身形成循环，
//       每次回调递增计数器
// ============================================================

use leptos::prelude::*;

#[component]
fn RafCounter() -> impl IntoView {
    let (count, set_count) = signal(0i32);

    // request_animation_frame 是 FnOnce 只执行一次，
    // 在回调中再次调用自身形成持续动画帧循环
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

// ============================================================
// 参考答案
// ============================================================
// <details>
// <summary>点击展开</summary>
//
// ```rust
// let (count, set_count) = signal(0i32);
// fn loop(c: WriteSignal<i32>) { c.update(|n| *n += 1);
//     request_animation_frame(move || loop(c)); }
// request_animation_frame(move || loop(set_count));
// ```
//
// request_animation_frame 是 FnOnce，递归调用形成持续循环。
// 与浏览器刷新同步，页面不可见时自动暂停。
//
// </details>
