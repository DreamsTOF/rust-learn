// ============================================================
// 练习 e174: join_futures — 并行执行多个 future
//
// 目标: 使用 futures::join! 同时运行两个异步任务并合并结果
//
// 难度: ⭐⭐
// 核心知识点: join!、并行 future
// ============================================================

use futures::join;
use leptos::prelude::*;
use leptos::task::spawn_local;

// TODO: 定义两个 async 函数，代表并行的数据获取
async fn fetch_user() -> String {
    "用户数据".to_string()
}

async fn fetch_posts() -> String {
    "帖子数据".to_string()
}

#[component]
fn Exercise() -> impl IntoView {
    let (result, set_result) = signal(String::new());

    // TODO: 使用 join! 并行执行两个异步任务
    spawn_local(async move {
        let (user, posts) = join!(fetch_user(), fetch_posts());
        set_result.set(format!("{user} + {posts}"));
    });

    view! {
        <div>
            <h2>"e174: join_futures"</h2>
            <p>{result}</p>
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
// - `futures::join!` 并发执行多个 future，等待全部完成
// - `join!` 返回所有结果的元组
// - 与顺序 `.await` 不同，`join!` 不会阻塞其他 future 的执行
//
// </details>
