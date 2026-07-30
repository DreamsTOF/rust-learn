// ============================================================
// 练习 e73: Reactive Graph Drop — 响应式图析构与资源清理
//
// 核心知识点:
//   - on_cleanup(): 在当前 Owner 清理时执行回调
//   - 响应式节点在 Owner 释放时自动 dispose
//   - Effect 停止后其子节点也会被清理
//
// 难度: ⭐⭐⭐ (关键位置有 TODO — 补全约 50%)
// ============================================================

use leptos::prelude::*;

#[component]
fn Exercise() -> impl IntoView {
    // 创建一个信号
    let (count, set_count) = signal(0);

    // TODO: 使用 on_cleanup 注册资源清理回调
    // 提示: on_cleanup 在当前 Owner（组件的响应式作用域）清理时执行
    on_cleanup(|| {
        // 这里执行清理逻辑，例如取消定时器、关闭连接等
        leptos::logging::log!("e73: 组件作用域已清理");
    });

    // TODO: 创建一个 Effect，在其内部也注册 cleanup
    // 提示: Effect::new 创建了子 Owner，内部的 on_cleanup 在 effect 停止时执行
    let effect = Effect::new(move || {
        let _ = count.get();

        // 这个 cleanup 会在 effect 停止时执行
        on_cleanup(|| {
            leptos::logging::log!("e73: effect 已停止");
        });
    });

    // TODO: 修改 signal 触发 effect 运行
    set_count.set(1);

    // TODO: 主动停止 effect — 会触发其内部的 on_cleanup
    effect.stop();

    view! {
        <p>"on_cleanup 在当前 Owner 清理时执行——包括组件销毁和 Effect 停止。"</p>
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
//
//     on_cleanup(|| {
//         leptos::logging::log!("e73: 组件作用域已清理");
//     });
//
//     let effect = Effect::new(move || {
//         let _ = count.get();
//         on_cleanup(|| {
//             leptos::logging::log!("e73: effect 已停止");
//         });
//     });
//
//     set_count.set(1);
//     effect.stop();
//
//     view! {
//         <p>"on_cleanup 在当前 Owner 清理时执行——包括组件销毁和 Effect 停止。"</p>
//     }
// }
//
// fn main() {
//     mount_to_body(Exercise);
// }
//
// ### 知识点
// - 每个响应式节点（组件、Effect 等）都有一个 Owner
// - `on_cleanup(f)` 注册一个回调，在当前 Owner 清理时执行
// - 触发清理的场景：组件卸载、Effect.stop()、Owner.dispose()
// - Effect::new() 创建子 Owner，其内部的 on_cleanup 独立于父级
// - 清理是级联的：Owner 清理时会先清理所有子节点
// - 适合用来：取消定时器、关闭 WebSocket、释放外部资源
// </details>
