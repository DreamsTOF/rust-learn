// ============================================================
// 练习 e74: Effect Nested Scope — 子 Scope 中的 Effect
//
// 核心知识点:
//   - Effect 嵌套：父 Effect 创建子 Scope
//   - 子 Effect 在父 Effect 的 Scope 中独立追踪依赖
//   - 父 Effect 停止时，子 Scope 及其 Effect 也被清理
//   - on_cleanup 在各自 Scope 中独立执行
//
// 难度: ⭐⭐⭐ (关键位置有 TODO — 补全约 50%)
// ============================================================

use leptos::prelude::*;

#[component]
fn Exercise() -> impl IntoView {
    let (outer, set_outer) = signal(0);
    let (inner, set_inner) = signal(0);

    // TODO: 创建父 Effect，内部再创建子 Effect
    // 提示: Effect::new 内部可以再创建 Effect::new
    //       子 Effect 自动成为父 Effect 的子 Scope
    let parent_effect = Effect::new(move || {
        let _o = outer.get();

        // 子 Effect — 属于父 Effect 的子 Scope
        // TODO: 子 Effect 只追踪 inner，不追踪 outer
        Effect::new(move || {
            let _i = inner.get();
            // 子 Effect 的 cleanup — 在子 Effect 停止时执行
            on_cleanup(|| {
                leptos::logging::log!("e74: 子 Effect 已清理");
            });
        });

        // 父 Effect 的 cleanup — 在父 Effect 停止时执行
        on_cleanup(|| {
            leptos::logging::log!("e74: 父 Effect 已清理");
        });
    });

    // 修改 inner — 只触发子 Effect
    set_inner.set(1);

    // 修改 outer — 触发父 Effect，父 Effect 重新运行会销毁并重建子 Effect
    set_outer.set(1);

    // TODO: 停止父 Effect — 子 Effect 也会被级联清理
    parent_effect.stop();

    view! {
        <p>"Effect 嵌套形成父子 Scope 层级：父停止则子自动清理。"</p>
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
//     let (outer, set_outer) = signal(0);
//     let (inner, set_inner) = signal(0);
//
//     let parent_effect = Effect::new(move || {
//         let _o = outer.get();
//
//         Effect::new(move || {
//             let _i = inner.get();
//             on_cleanup(|| {
//                 leptos::logging::log!("e74: 子 Effect 已清理");
//             });
//         });
//
//         on_cleanup(|| {
//             leptos::logging::log!("e74: 父 Effect 已清理");
//         });
//     });
//
//     set_inner.set(1);
//     set_outer.set(1);
//     parent_effect.stop();
//
//     view! {
//         <p>"Effect 嵌套形成父子 Scope 层级：父停止则子自动清理。"</p>
//     }
// }
//
// fn main() {
//     mount_to_body(Exercise);
// }
//
// ### 知识点
// - Effect::new() 内部创建 Effect::new() → 子 Effect 自动成为子 Scope
// - 父 Effect 重新运行时：旧子 Scope 被销毁（触发子 Scope 的 cleanup），然后创建新子 Scope
// - 父 Effect 停止时：级联清理所有子 Scope
// - 每个 Effect 的 `on_cleanup` 在其自己的 Owner 清理时执行
// - 追踪独立：子 Effect 只追踪其闭包内读取的信号
// - 这种层级结构对应于组件树的响应式生命周期
// </details>
