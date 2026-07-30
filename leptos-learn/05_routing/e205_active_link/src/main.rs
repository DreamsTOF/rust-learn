// ============================================================
// 练习 e205: 活动链接高亮 (active_link)
//
// 目标: 利用 <A> 组件的自动 active 类，实现当前页面
//       导航链接的高亮效果。
//
// 难度: ⭐⭐
// 核心知识点: class="active" 自动高亮
//
// TODO: 按照注释提示补全代码
// ============================================================

use leptos::prelude::*;
use leptos_router::components::Router;

// === 步骤 1 ——————————————————————————————————————————
// TODO: 定义 Dashboard、Settings、Profile 三个页面组件
// 提示: 需要时添加 Routes, Route, A 等组件导入
//       每个组件显示对应的页面标题和内容

// TODO: Dashboard 组件
// TODO: Settings 组件
// TODO: Profile 组件

#[component]
fn Exercise() -> impl IntoView {
    // === 步骤 2 ——————————————————————————————————————————
    // TODO: 使用 <A> 组件创建导航栏
    // 注意：<A> 组件会自动为当前活动页面的链接添加
    //       class="active"，无需手动判断
    // 提示：在 index.html 中已预置 .active 样式

    view! {
        <Router>
            <nav>
                // TODO: 添加三个 <A> 导航链接
                //       - 仪表盘 (/dashboard)
                //       - 设置 (/settings)
                //       - 个人资料 (/profile)
            </nav>
            <main>
                // TODO: 添加 <Routes fallback=|| "页面未找到"> 和三条 <Route>
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
// use leptos_router::components::{Router, Routes, Route, A};
//
// #[component]
// fn Dashboard() -> impl IntoView {
//     view! { <h2>"仪表盘"</h2><p>"欢迎回来！这是你的控制面板。"</p> }
// }
//
// #[component]
// fn Settings() -> impl IntoView {
//     view! { <h2>"设置"</h2><p>"在这里管理你的偏好设置。"</p> }
// }
//
// #[component]
// fn Profile() -> impl IntoView {
//     view! { <h2>"个人资料"</h2><p>"查看和编辑你的个人信息。"</p> }
// }
//
// #[component]
// fn Exercise() -> impl IntoView {
//     view! {
//         <Router>
//             <nav>
//                 <A href="/dashboard">"仪表盘"</A>
//                 <A href="/settings">"设置"</A>
//                 <A href="/profile">"个人资料"</A>
//             </nav>
//             <main>
//                 <Routes fallback=|| "页面未找到">
//                     <Route path="/dashboard" view=Dashboard/>
//                     <Route path="/settings" view=Settings/>
//                     <Route path="/profile" view=Profile/>
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
// - <A> 组件自动检测当前 URL 是否匹配其 href
// - 匹配时自动添加 class="active"，无需手动管理
// - 通过 CSS 为 .active 类设置样式即可实现高亮
// - 这是声明式路由导航的核心优势之一
//
// </details>
