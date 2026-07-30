// ============================================================
// 练习 42: memo_get
//
// 目标: 使用 .get() 方法读取 Memo 的缓存值
//
// 难度: ⭐
// 核心知识点: Memo 的 .get()
//
// TODO: 按照注释提示补全代码
// ============================================================
use leptos::prelude::*;

#[component]
fn Exercise() -> impl IntoView {
    // === 步骤 1 ——————————————————————————————————————————
    // TODO: 创建信号 `count`，i32 初始值为 5

    // === 步骤 2 ——————————————————————————————————————————
    // TODO: 创建 Memo `double` 派生值为 count * 2

    // === 步骤 3 ——————————————————————————————————————————
    // TODO: 使用 `double.get()` 读取 Memo 的值到局部变量
    //   在视图中同时显示 Memo 信号直接读取和 .get() 读取的结果

    view! {
        <div>
            <p>"练习 42: memo_get"</p>
            // TODO: 显示 count 的值
            // TODO: 显示 double (直接读取，使用 memo 信号名)
            // TODO: 显示 double.get() (调用方法获取值)
            // TODO: 按钮 "count += 1"
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
// #[component]
// fn Exercise() -> impl IntoView {
//     let (count, set_count) = signal(5);
//
//     let double = Memo::new(move |_| count() * 2);
//
//     // .get() 克隆 Memo 的缓存值到局部变量
//     let double_via_get = double.get();
//
//     view! {
//         <div>
//             <p>"练习 42: memo_get"</p>
//             <p>"count = " {count}</p>
//             <p>"double (memo 信号) = " {double}</p>
//             <p>"double.get() = " {double_via_get}</p>
//             <button on:click=move |_| set_count.update(|v| *v += 1)>"count += 1"</button>
//         </div>
//     }
// }
// ```
//
// ### 知识点
// - `.get()` 返回 Memo 当前缓存值的克隆副本
// - 与 `ReadSignal::get()` 类似，Memo 的 `.get()` 是显式读取
// - 使用 `.get()` 获取到的值是普通 Rust 值，不保留响应式追踪
// - 在视图中直接使用 Memo 对象（如 `{double}`）会自动追踪其变化
//
// </details>
