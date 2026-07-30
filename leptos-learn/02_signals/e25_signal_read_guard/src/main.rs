// ============================================================
// 练习 25: signal_read_guard
//
// 目标: 用 let guard = count.read() 读取值
//
// 难度: ⭐⭐
// 核心知识点: .read() guard 读取
// ============================================================
use leptos::prelude::*;

#[component]
fn Exercise() -> impl IntoView {
    let (count, _set_count) = signal(99);

    // === 使用 .read() 获取 guard，通过解引用读取值 ===
    let guard = count.read();
    let value = *guard;

    view! {
        <div>
            <p>"guard 读取的值: " {value}</p>
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
// ### 代码
// ```rust
// let guard = count.read();
// let value = *guard;
// ```
//
// ### 知识点
// - `.read()` 返回一个 RAII guard，实现 `Deref` 到 `T`
// - guard 持有信号的运行时借用，在其生命周期内阻止写入
// - 适合需要长时间引用信号内部值的场景
//
// </details>
