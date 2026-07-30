// ============================================================
// 练习 e347: 不可变状态 — 用 set() 替换整个值
//
// 核心知识点:
//   - 使用 signal() 创建包含结构体的状态
//   - 通过 set() 替换整个值实现"不可变"更新
//   - 对比可变更新与不可变替换的语义差异
//
// 难度: ⭐⭐ (关键位置有 TODO，补全约 50%)
// ============================================================

use leptos::prelude::*;

// TODO: 定义一个 User 结构体（包含 name: String, email: String, age: u32）
// 提示: 需要派生 Clone + Debug 以便在视图中克隆显示

#[component]
fn Exercise() -> impl IntoView {
    // TODO: 使用 signal() 创建一个 (ReadSignal, WriteSignal) 对
    // 初始化为 User { name: "张三", email: "zhangsan@example.com", age: 30 }

    // TODO: 显示当前用户信息（使用 .get() 读取）

    // TODO: 创建"更新为用户李四"按钮，用 set() 替换整个 User 对象
    // 提示: set_user.set(User { ... })

    // TODO: 创建"重置"按钮，用 set() 恢复默认值

    view! {
        <div>
            <h2>"用户信息（不可变更新）"</h2>
            // TODO: 显示 name、email、age
            // TODO: 添加"更新为用户李四"和"重置"两个按钮
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
// #[derive(Clone, Debug)]
// struct User {
//     name: String,
//     email: String,
//     age: u32,
// }
//
// #[component]
// fn Exercise() -> impl IntoView {
//     let default_user = User {
//         name: "张三".to_string(),
//         email: "zhangsan@example.com".to_string(),
//         age: 30,
//     };
//     let (user, set_user) = signal(default_user);
//
//     let update_user = move |_| {
//         set_user.set(User {
//             name: "李四".to_string(),
//             email: "lisi@example.com".to_string(),
//             age: 25,
//         });
//     };
//
//     let reset_user = move |_| {
//         set_user.set(User {
//             name: "张三".to_string(),
//             email: "zhangsan@example.com".to_string(),
//             age: 30,
//         });
//     };
//
//     view! {
//         <div>
//             <h2>"用户信息（不可变更新）"</h2>
//             <p>"姓名: " {move || user.get().name.clone()}</p>
//             <p>"邮箱: " {move || user.get().email.clone()}</p>
//             <p>"年龄: " {move || user.get().age.to_string()}</p>
//             <button on:click=update_user>"更新为用户李四"</button>
//             <button on:click=reset_user>"重置"</button>
//         </div>
//     }
// }
//
// fn main() {
//     mount_to_body(Exercise);
// }
//
// ### 知识点
// - `signal(value)` 返回 `(ReadSignal<T>, WriteSignal<T>)`，读写分离
// - 不可变更新：通过 `set()` 替换整个值，而非原地修改
// - 每次 `set()` 都创建新的结构体实例，旧值自动丢弃
// - 适合值较小、更新频率不高的场景
// - `ReadSignal::get()` 返回 `T` 的克隆（T: Clone）
// </details>
