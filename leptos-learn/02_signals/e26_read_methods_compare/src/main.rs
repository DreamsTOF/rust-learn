// ============================================================
// 练习 26: read_methods_compare — 三种读取方式对比
//
// 目标: 用同一信号展示 .get() vs .with() vs .read() 的区别
//
// 难度: ⭐⭐
// 核心知识点:
//   - .get() 返回所有权值（克隆）
//   - .with() 通过闭包借用引用
//   - .read() 返回守卫，解引用后获得引用
// ============================================================

use leptos::prelude::*;

#[component]
fn Exercise() -> impl IntoView {
    // === 步骤 1 ——————————————————————————————————————————
    // TODO: 使用 signal(42) 创建一个信号 count
    let (count, _set_count) = signal(42);

    // === 步骤 2 ——————————————————————————————————————————
    // TODO: 用三种方式读取信号值

    // 方式一: .get() — 返回克隆的 i32
    let by_get = count.get();

    // 方式二: .with() — 闭包内接收 &i32
    let by_with = count.with(|val| format!("with: {}", val));

    // 方式三: .read() — 返回 ReadGuard，解引用得 &i32
    let by_read = format!("read: {}", *count.read());

    view! {
        <div>
            <h1>"练习 26: 三种读取方式对比"</h1>
            <p>"count.get() = " {by_get}</p>
            <p>"count.with(|v| ...) = " {by_with}</p>
            <p>"*count.read() = " {by_read}</p>
        </div>
    }
}

fn main() {
    console_error_panic_hook::set_once();
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
// let (count, _set_count) = signal(42);
// let by_get = count.get();
// let by_with = count.with(|val| format!("with: {}", val));
// let by_read = format!("read: {}", *count.read());
// ```
//
// ### 知识点
// - `.get()` 最直接，但会克隆值。适合简单场景。
// - `.with()` 避免克隆，通过闭包借用。适合需要引用做计算的场景。
// - `.read()` 返回 `ReadGuard<T>`，实现了 `Deref<Target=T>`。
//   守卫借用了信号的内部值，在守卫生命周期内保持借入状态。
//   适合需要多次引用且不想反复调用的场景。
//
// </details>
