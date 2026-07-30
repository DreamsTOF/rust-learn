// ============================================================
// 练习 e124: Context 在路由场景 (context_router)
//
// 核心知识点:
//   - Layout 组件提供 Context，Page 组件消费
//   - 模拟路由场景中的跨组件数据共享
//
// 难度: ⭐⭐
// ============================================================

use leptos::prelude::*;

// TODO: 定义 User 结构体，包含用户名和角色
#[derive(Clone)]
struct User {
    name: String,
    role: String,
}

// TODO: 使用 use_context 获取 User 并渲染
#[component]
fn DashboardPage() -> impl IntoView {
    let user = use_context::<User>()
        .expect("User should be provided by Layout");

    view! {
        <div style="border: 1px solid green; padding: 8px; margin: 8px 0;">
            <h3>"Dashboard Page"</h3>
            <p>"Welcome, " {user.name.clone()}</p>
            <p>"Role: " {user.role.clone()}</p>
        </div>
    }
}

// TODO: Layout 组件提供 User Context
#[component]
fn AppLayout() -> impl IntoView {
    provide_context(User {
        name: "Alice".to_string(),
        role: "Admin".to_string(),
    });

    view! {
        <div style="border: 1px solid gray; padding: 8px;">
            <h2>"App Layout"</h2>
            <p>"Layout 提供 User Context"</p>
            <DashboardPage/>
        </div>
    }
}

#[component]
fn Exercise() -> impl IntoView {
    view! {
        <div>
            <h1>"Context Router Demo"</h1>
            <p>"Layout 提供 Context，Page 消费 — 模拟路由场景"</p>
            <AppLayout/>
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
// use leptos::prelude::*;
//
// #[derive(Clone)]
// struct User { name: String, role: String }
//
// #[component]
// fn DashboardPage() -> impl IntoView {
//     let user = use_context::<User>().expect("...");
//     view! {
//         <div>
//             <p>"Welcome, " {user.name.clone()}</p>
//             <p>"Role: " {user.role.clone()}</p>
//         </div>
//     }
// }
//
// #[component]
// fn AppLayout() -> impl IntoView {
//     provide_context(User { name: "Alice".into(), role: "Admin".into() });
//     view! {
//         <div>
//             <h2>"App Layout"</h2>
//             <DashboardPage/>
//         </div>
//     }
// }
//
// #[component]
// fn Exercise() -> impl IntoView {
//     view! { <div><AppLayout/></div> }
// }
//
// fn main() { mount_to_body(Exercise); }
// ```
//
// ### 知识点
// - 路由场景中 Layout 负责公共数据（用户信息、主题等）
// - Page 组件通过 use_context 直接获取，无需 props 传递
// - 解耦：Layout 和 Page 可以独立开发，仅约定 Context 类型
//
// </details>
