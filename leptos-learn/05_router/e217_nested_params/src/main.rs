// ============================================================
// 练习 217: nested_params — 嵌套路由中的路径参数
//
// 目标: 在嵌套路由的不同层级读取路径参数
//       - 父路由参数 `:section`
//       - 子路由参数 `:id`
//       - 使用 use_params_map() 在不同层级读取
//
// 难度: ⭐⭐⭐
// 核心知识点: 嵌套路由参数, use_params_map()
//
// TODO: 按照注释提示补全代码
// ============================================================

use leptos::prelude::*;
use leptos_router::components::*;
use leptos_router::hooks::use_params_map;
use leptos_router::path;

// === 步骤 1 ——————————————————————————————————————————
// TODO: 创建 SectionLayout 布局组件
// 提示: 从 use_params_map() 读取 :section 参数并显示
//       包含导航链接和 <Outlet/>
#[component]
fn SectionLayout() -> impl IntoView {
    let params = use_params_map();
    let section = move || params.read().get("section").unwrap_or_default();

    view! {
        <nav>
            <a href={move || format!("/{}", section())}>"版块首页"</a>
            <a href={move || format!("/{}/1", section())}>"第 1 项"</a>
            <a href={move || format!("/{}/2", section())}>"第 2 项"</a>
            <a href={move || format!("/{}/3", section())}>"第 3 项"</a>
        </nav>
        <h2>{move || format!("版块: {}", section())}</h2>
        // TODO: 在这里放置 Outlet 组件，用来渲染匹配的子路由
        <Outlet/>
    }
}

// TODO: 创建 SectionHome 组件
// 提示: 使用 use_params_map() 读取 :section 并显示 "欢迎来到 {section} 版块"
#[component]
fn SectionHome() -> impl IntoView {
    let params = use_params_map();
    let section = move || params.read().get("section").unwrap_or_default();
    view! { <p>"欢迎来到 " {move || section()} " 版块"</p> }
}

// TODO: 创建 ItemDetail 组件
// 提示: 使用 use_params_map() 读取 :section 和 :id 参数
#[component]
fn ItemDetail() -> impl IntoView {
    let params = use_params_map();
    let section = move || params.read().get("section").unwrap_or_default();
    let id = move || params.read().get("id").unwrap_or_default();
    view! { <p>"正在查看 " {move || section()} " 版块的第 " {move || id()} " 项"</p> }
}

fn main() {
    mount_to_body(|| view! {
        // TODO: 配置路由器，使用 ParentRoute 定义嵌套路由结构
        <Router>
            <main>
                <Routes fallback=|| "页面未找到">
                    <ParentRoute path=path!("/:section") view=SectionLayout>
                        <Route path=path!("/:section") view=SectionHome/>
                        <Route path=path!("/:section/:id") view=ItemDetail/>
                    </ParentRoute>
                </Routes>
            </main>
        </Router>
    });
}
