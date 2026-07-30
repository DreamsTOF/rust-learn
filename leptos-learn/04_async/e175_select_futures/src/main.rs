// ============================================================
// 练习 e175: select_futures — 竞速执行多个 future
//
// 目标: 使用 futures::select! 获取最先完成的 future 结果
//
// 难度: ⭐⭐⭐
// 核心知识点: select!、FutureExt::fuse、竞速
// ============================================================

use futures::future::FutureExt;
use leptos::prelude::*;
use leptos::task::spawn_local;

// TODO: 定义两个 async 函数，模拟不同速度的任务
async fn fast_task() -> String {
    "快速任务完成".to_string()
}

async fn slow_task() -> String {
    "慢速任务完成".to_string()
}

#[component]
fn Exercise() -> impl IntoView {
    let (winner, set_winner) = signal("等待竞速结果...".to_string());

    // TODO: 使用 select! 获取最先完成的 future
    spawn_local(async move {
        let fut1 = fast_task().fuse();
        let fut2 = slow_task().fuse();
        futures::pin_mut!(fut1, fut2);

        futures::select! {
            result = fut1 => { set_winner.set(result); },
            result = fut2 => { set_winner.set(result); },
        }
    });

    view! {
        <div>
            <h2>"e175: select_futures"</h2>
            <p>{winner}</p>
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}

// ============================================================
// 参考答案 (思考后再看!)
// ============================================================
// <details>
// <summary>点击展开答案</summary>
//
// ### 知识点
// - `futures::select!` 轮询多个 future，返回最先完成的一个
// - `.fuse()` 将 future 转换为 `FusedFuture`（调用后不再轮询）
// - `pin_mut!` 在栈上固定 future（select! 要求 Unpin）
//
// </details>
