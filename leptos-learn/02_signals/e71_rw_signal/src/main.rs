// ============================================================
// 练习 e71: RwSignal — 同一个 handle 读写信号
//
// 核心知识点:
//   - RwSignal::new(): 创建可读可写的信号
//   - .get(): 读取值（追踪依赖）
//   - .set(): 设置值（通知订阅者）
//   - .update(): 通过闭包原地修改
//
// 难度: ⭐⭐⭐ (关键位置有 TODO — 补全约 50%)
// ============================================================

use leptos::prelude::*;

#[component]
fn Exercise() -> impl IntoView {
    // TODO: 使用 RwSignal::new(0) 创建一个可读可写的信号
    // 提示: 与 signal() 返回 (ReadSignal, WriteSignal) 不同，
    //       RwSignal 通过同一个 handle 同时支持读写
    let count = RwSignal::new(0);

    // TODO: 使用 .get() 读取值
    let _ = count.get();

    // TODO: 使用 .set() 设置值
    count.set(1);

    // TODO: 使用 .update() 通过闭包原地修改
    count.update(|n| *n += 1);

    // TODO: .split() 可以将 RwSignal 拆分为读写两部分
    let (read, write) = count.split();
    let _ = read.get();
    write.set(3);

    view! {
        <p>"RwSignal 通过同一个 handle 同时支持读写操作。"</p>
    }
}

fn main() {
    mount_to_body(Exercise);
}

// <details>
// 参考答案（去除注释后的纯净版本）:
//
// use leptos::prelude::*;
//
// #[component]
// fn Exercise() -> impl IntoView {
//     let count = RwSignal::new(0);
//     let _ = count.get();
//     count.set(1);
//     count.update(|n| *n += 1);
//     let (read, write) = count.split();
//     let _ = read.get();
//     write.set(3);
//     view! {
//         <p>"RwSignal 通过同一个 handle 同时支持读写操作。"</p>
//     }
// }
//
// fn main() {
//     mount_to_body(Exercise);
// }
//
// ### 知识点
// - `RwSignal::new(value)` 在 arena 中分配一个可读可写的信号
// - 与 `signal()` 的读写分离不同，RwSignal 是统一的 handle
// - `.get()` → 克隆当前值（追踪依赖）
// - `.set(val)` → 替换值（通知订阅者）
// - `.update(f)` → 通过 `&mut T` 闭包原地修改
// - `.split()` → 拆分为 `(ReadSignal, WriteSignal)`
// - `.read_only()` / `.write_only()` → 转为只读/只写 handle
// - RwSignal 是 `Copy` + `'static`，在 Owner 清理时自动释放
// </details>
