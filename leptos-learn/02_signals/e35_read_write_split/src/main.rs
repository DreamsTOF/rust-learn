// ============================================================
// 练习 e35: read_write_split — ReadSignal / WriteSignal 分离
//
// 核心知识点:
//   - signal() 返回 (ReadSignal<T>, WriteSignal<T>)
//   - 只读信号 (ReadSignal) 不可写入
//   - 只写信号 (WriteSignal) 不可读取
//   - 利用类型系统实现权限分离
//
// 难度: ⭐⭐
// ============================================================

use leptos::prelude::*;

// 只读子组件 — 只能读取，不可写入
#[component]
fn DisplayPanel(count: ReadSignal<i32>) -> impl IntoView {
    view! {
        <fieldset>
            <legend>"只读面板 (ReadSignal)"</legend>
            <p>"当前值: " {count}</p>
            // ❌ count 没有 set/update/write 方法 — 编译时报错
        </fieldset>
    }
}

// 只写子组件 — 只能写入，不可读取
#[component]
fn ControlPanel(set_count: WriteSignal<i32>) -> impl IntoView {
    view! {
        <fieldset>
            <legend>"只写面板 (WriteSignal)"</legend>
            <button on:click=move |_| set_count.update(|n| *n += 1)>"+1"</button>
            <button on:click=move |_| set_count.update(|n| *n -= 1)>"-1"</button>
            <button on:click=move |_| set_count.set(0)>"重置"</button>
            // ❌ set_count 没有 get/with 方法 — 编译时报错
        </fieldset>
    }
}

#[component]
fn Exercise() -> impl IntoView {
    // signal() 解构后得到两个独立的类型
    let (count, set_count) = signal(0);

    view! {
        <div>
            <p>"ReadSignal 只允许读取，WriteSignal 只允许写入。"</p>
            <DisplayPanel count=count />
            <ControlPanel set_count=set_count />
        </div>
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
// fn DisplayPanel(count: ReadSignal<i32>) -> impl IntoView {
//     view! {
//         <fieldset>
//             <legend>"只读面板 (ReadSignal)"</legend>
//             <p>"当前值: " {count}</p>
//         </fieldset>
//     }
// }
//
// #[component]
// fn ControlPanel(set_count: WriteSignal<i32>) -> impl IntoView {
//     view! {
//         <fieldset>
//             <legend>"只写面板 (WriteSignal)"</legend>
//             <button on:click=move |_| set_count.update(|n| *n += 1)>"+1"</button>
//             <button on:click=move |_| set_count.update(|n| *n -= 1)>"-1"</button>
//             <button on:click=move |_| set_count.set(0)>"重置"</button>
//         </fieldset>
//     }
// }
//
// #[component]
// fn Exercise() -> impl IntoView {
//     let (count, set_count) = signal(0);
//
//     view! {
//         <div>
//             <p>"ReadSignal 只允许读取，WriteSignal 只允许写入。"</p>
//             <DisplayPanel count=count />
//             <ControlPanel set_count=set_count />
//         </div>
//     }
// }
//
// fn main() {
//     mount_to_body(Exercise);
// }
//
// 知识点:
// 1. signal() 解构后得到 ReadSignal 和 WriteSignal 两个独立类型
// 2. ReadSignal 只提供 get / with / track 等读取方法
// 3. WriteSignal 只提供 set / update / try_update / write 写入方法
// 4. 利用类型系统实现权限分离，编译期保证安全
// 5. 实际开发中常将 ReadSignal 传给纯展示组件，WriteSignal 传给表单/控制组件
// </details>
