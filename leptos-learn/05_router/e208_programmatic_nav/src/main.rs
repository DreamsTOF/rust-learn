// ============================================================
// 练习 e208: 编程式导航 (programmatic_nav)
//
// 目标: 使用 use_navigate() hook 在事件处理中
//       实现编程式导航，不依赖 <A> 链接。
//
// 难度: ⭐⭐
// 核心知识点: use_navigate(), 按钮点击触发导航
//
// TODO: 按照注释提示补全代码
// ============================================================

use leptos::prelude::*;
use leptos_router::components::*;
use leptos_router::hooks::*;

// ============================================================
// 步骤 1 — 定义页面组件
// ============================================================

// TODO: 创建 Home 和 About 组件
#[component]
fn Home() -> impl IntoView {
    view! {
        <h2>"首页"</h2>
        <p>"欢迎来到首页！"</p>
    }
}

// TODO: 创建 About 组件
#[component]
fn About() -> impl IntoView {
    view! {
        <h2>"关于"</h2>
        <p>"这是关于页面。"</p>
    }
}

// ============================================================
// 步骤 2 — 创建导航按钮组件
// ============================================================
// 注意: use_navigate() 必须在 <Router> 的子组件中调用

#[component]
fn NavButtons() -> impl IntoView {
    // TODO: 调用 use_navigate() 获取 navigate 函数
    // let navigate = use_navigate();
    //
    // TODO: 添加两个按钮，分别导航到 "/home" 和 "/about"
    //       使用 on:click 事件 + move |_| navigate("/path", Default::default())
    //
    // 提示: navigate 接受两个参数 — 路径字符串和 NavigateOptions
    //       使用 Default::default() 作为选项参数

    view! {
        <nav>
            // TODO: 添加 "Go Home" 和 "Go About" 两个按钮
        </nav>
    }
}

// ============================================================
// 步骤 3 — 组装根组件
// ============================================================

#[component]
fn Exercise() -> impl IntoView {
    view! {
        <Router>
            // TODO: 将 <NavButtons/> 放在此处
            <main>
                <Routes fallback=|| "页面未找到">
                    // TODO: 添加 /home 和 /about 两条路由
                </Routes>
            </main>
        </Router>
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
// use leptos_router::components::*;
use leptos_router::hooks::*;
//
// #[component]
// fn Home() -> impl IntoView {
//     view! {
//         <h2>"首页"</h2>
//         <p>"欢迎来到首页！"</p>
//     }
// }
//
// #[component]
// fn About() -> impl IntoView {
//     view! {
//         <h2>"关于"</h2>
//         <p>"这是关于页面。"</p>
//     }
// }
//
// #[component]
// fn NavButtons() -> impl IntoView {
//     let navigate = use_navigate();
//     view! {
//         <nav>
//             <button on:click=move |_| navigate("/home", Default::default())>
//                 "Go Home"
//             </button>
//             " "
//             <button on:click=move |_| navigate("/about", Default::default())>
//                 "Go About"
//             </button>
//         </nav>
//     }
// }
//
// #[component]
// fn Exercise() -> impl IntoView {
//     view! {
//         <Router>
//             <NavButtons/>
//             <main>
//                 <Routes fallback=|| "页面未找到">
//                     <Route path="/home" view=Home/>
//                     <Route path="/about" view=About/>
//                 </Routes>
//             </main>
//         </Router>
//     }
// }
//
// fn main() {
//     mount_to_body(Exercise);
// }
// ```
//
// ### 知识点
// - use_navigate() 返回一个 navigate 函数，可在事件处理中调用
// - navigate(path, options) 接受路径和 NavigateOptions 参数
// - 调用 navigate() 必须在 <Router> 作用域内（子组件中调用）
// - 编程式导航适用于按钮点击、表单提交等场景
//
// </details>
