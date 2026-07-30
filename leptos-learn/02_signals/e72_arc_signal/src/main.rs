// ============================================================
// 练习 e72: ArcSignal — 线程安全的信号共享
//
// 核心知识点:
//   - arc_signal(): 创建引用计数、线程安全的信号
//   - ArcReadSignal / ArcWriteSignal: Clone 而非 Copy
//   - 跨线程 / Web Worker 共享数据
//   - ArcSignal::derive(): 包装派生信号
//
// 难度: ⭐⭐⭐ (关键位置有 TODO — 补全约 50%)
// ============================================================

use leptos::prelude::*;

#[component]
fn Exercise() -> impl IntoView {
    // TODO: 使用 arc_signal(0) 创建线程安全的引用计数信号
    // 提示: arc_signal 返回 (ArcReadSignal, ArcWriteSignal)
    //       它们实现 Send + Sync，可在线程间共享
    let (count, set_count) = arc_signal(0);

    // TODO: ArcReadSignal 是 Clone 但不是 Copy
    // 需要在 move 闭包前显式 clone
    let count_clone = count.clone();

    // TODO: 使用 ArcSignal::derive() 包装派生信号
    let double = ArcSignal::derive(move || count_clone.get() * 2);
    let _ = double.get();

    // TODO: 读取和写入
    set_count.set(1);
    let _ = count.get();

    view! {
        <p>"ArcSignal 是引用计数、线程安全的信号类型。"</p>
        <p>"适用于需要在不同作用域或线程间共享的场景。"</p>
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
//     let (count, set_count) = arc_signal(0);
//     let count_clone = count.clone();
//     let double = ArcSignal::derive(move || count_clone.get() * 2);
//     let _ = double.get();
//     set_count.set(1);
//     let _ = count.get();
//     view! {
//         <p>"ArcSignal 是引用计数、线程安全的信号类型。"</p>
//         <p>"适用于需要在不同作用域或线程间共享的场景。"</p>
//     }
// }
//
// fn main() {
//     mount_to_body(Exercise);
// }
//
// ### 知识点
// - `arc_signal(value)` → `(ArcReadSignal<T>, ArcWriteSignal<T>)`
// - arena 信号 (`signal()`) 是 `Copy` 但受 Owner 生命周期约束
// - Arc 信号是 `Clone` 但不是 `Copy`，通过 Arc 引用计数管理生命周期
// - Arc 信号实现了 `Send + Sync`，可跨线程传递
// - `ArcSignal::derive(f)` 包装闭包为类型擦除的 Arc 信号
// - `ArcSignal::stored(v)` 将静态值包装为非响应式 Arc 信号
// - `ArcRwSignal::new(v)` 是线程安全的 RwSignal 版本
// </details>
