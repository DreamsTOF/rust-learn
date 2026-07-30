// ============================================================
// 练习 e70: StoredValue vs Signal — 对比两种存储方式
//
// 核心知识点:
//   - Signal: 响应式，读取追踪依赖，修改通知订阅者
//   - StoredValue: 非响应式，不追踪也不通知
//   - Effect::new() 仅对 Signal 变化作出反应
//
// 难度: ⭐⭐⭐ (关键位置有 TODO — 补全约 50%)
// ============================================================

use leptos::prelude::*;

#[component]
fn Exercise() -> impl IntoView {
    // TODO: 创建一个 Signal（响应式）和一个 StoredValue（非响应式）
    // 提示: let (count, set_count) = signal(0);
    //        let stored = StoredValue::new(0);
    let (count, set_count) = signal(0);
    let stored = StoredValue::new(0);

    // TODO: 创建一个 Effect，同时读取 signal 和 stored
    // 观察：effect 只会因 signal 变化而重新运行
    Effect::new(move || {
        // 读取 signal — 会被 effect 追踪
        let c = count.get();
        // 读取 stored — 不会被追踪
        let s = *stored.read_value();
        // 注意：修改 stored 不会重新运行这个 effect
        let _ = (c, s);
    });

    // TODO: 分别修改 signal 和 stored
    // signal.set(1) 会触发 effect 重新运行
    // *stored.write_value() = 1 不会触发 effect
    set_count.set(1);
    *stored.write_value() = 1;

    view! {
        <p>"Signal 是响应式的——修改会触发依赖它的 effect。"</p>
        <p>"StoredValue 是非响应式的——修改不触发任何 effect。"</p>
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
//     let (count, set_count) = signal(0);
//     let stored = StoredValue::new(0);
//
//     Effect::new(move || {
//         let c = count.get();
//         let s = *stored.read_value();
//         let _ = (c, s);
//     });
//
//     set_count.set(1);
//     *stored.write_value() = 1;
//
//     view! {
//         <p>"Signal 是响应式的——修改会触发依赖它的 effect。"</p>
//         <p>"StoredValue 是非响应式的——修改不触发任何 effect。"</p>
//     }
// }
//
// fn main() {
//     mount_to_body(Exercise);
// }
//
// ### 知识点
// | Signal | StoredValue |
// |--------|-------------|
// | `.get()` 在 effect 中会被追踪 | `.read_value()` 不会被追踪 |
// | `.set()` 通知所有订阅者 | `.write_value()` 不通知任何人 |
// | 用于驱动 UI 更新 | 用于存储无需响应式更新的数据 |
// | `ReadSignal` / `WriteSignal` 分离读写 | 单个 `StoredValue` handle |
// </details>
