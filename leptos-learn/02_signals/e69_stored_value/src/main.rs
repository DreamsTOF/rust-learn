// ============================================================
// 练习 e69: StoredValue — 不触发响应式更新的存储容器
//
// 核心知识点:
//   - StoredValue::new(): 创建非响应式存储（Copy handle）
//   - read_value() / write_value(): 不追踪/不通知的读写
//   - 适用场景：存储不需要触发 UI 更新的数据
//
// 难度: ⭐⭐⭐ (关键位置有 TODO — 补全约 50%)
// ============================================================

use leptos::prelude::*;

#[component]
fn Exercise() -> impl IntoView {
    // TODO: 使用 StoredValue::new(42) 创建非响应式存储
    // 提示: StoredValue 是 Copy 的，不会追踪读取或通知写入
    let stored = StoredValue::new(42);

    // TODO: 通过 read_value() 获取只读 guard
    // 提示: let guard = stored.read_value(); 然后通过解引用读取
    let guard = stored.read_value();
    let _value: &i32 = &guard;

    // TODO: 通过 write_value() 获取可变 guard 并修改值
    // 提示: *stored.write_value() = 100;
    *stored.write_value() = 100;

    view! {
        <p>"StoredValue 是 Copy 的但非响应式——读写都不会追踪或通知订阅者。"</p>
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
//     let stored = StoredValue::new(42);
//     let guard = stored.read_value();
//     let _value: &i32 = &guard;
//     *stored.write_value() = 100;
//     view! {
//         <p>"StoredValue 是 Copy 的但非响应式——读写都不会追踪或通知订阅者。"</p>
//     }
// }
//
// fn main() {
//     mount_to_body(Exercise);
// }
//
// ### 知识点
// - `StoredValue::new(value)` 在响应式 arena 中分配非响应式值
// - 返回 `StoredValue<T>` 是 `Copy` + `'static`
// - `.read_value()` 返回 `ReadGuard<T>`，通过解引用读取
// - `.write_value()` 返回 `UntrackedWriteGuard<T>`，通过解引用写入
// - 与信号不同：修改 StoredValue **不会** 通知订阅者或触发重渲染
// - 适合存储：定时器句柄、DOM 引用、无需响应式的配置数据
// </details>
