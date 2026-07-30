// ============================================================
// 练习 e347: 不可变状态 — 用 set() 替换整个值
//
// 核心知识点:
//   - 使用 signal() 创建包含结构体的状态
//   - 通过 set() 替换整个值实现"不可变"更新
//   - 对比可变更新与不可变替换的语义差异
//
// 难度: ⭐⭐
// ============================================================

use leptos::prelude::*;

#[derive(Clone, Debug)]
struct User {
    name: String,
    email: String,
    age: u32,
}

#[component]
fn Exercise() -> impl IntoView {
    let default_user = User {
        name: "张三".to_string(),
        email: "zhangsan@example.com".to_string(),
        age: 30,
    };
    let (user, set_user) = signal(default_user);

    let update_user = move |_| {
        set_user.set(User {
            name: "李四".to_string(),
            email: "lisi@example.com".to_string(),
            age: 25,
        });
    };

    let reset_user = move |_| {
        set_user.set(User {
            name: "张三".to_string(),
            email: "zhangsan@example.com".to_string(),
            age: 30,
        });
    };

    view! {
        <div>
            <h2>"用户信息（不可变更新）"</h2>
            <p>"姓名: " {move || user.get().name.clone()}</p>
            <p>"邮箱: " {move || user.get().email.clone()}</p>
            <p>"年龄: " {move || user.get().age.to_string()}</p>
            <button on:click=update_user>"更新为用户李四"</button>
            <button on:click=reset_user>"重置"</button>
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
