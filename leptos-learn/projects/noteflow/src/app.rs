use crate::components::sidebar::Sidebar;
use crate::components::tab_bar::TabBar;
use crate::pages::activity::ActivityPage;
use crate::pages::auth::AuthPage;
use crate::pages::board::BoardPage;
use crate::pages::editor::EditorPage;
use crate::pages::members::MembersPage;
use crate::pages::share::SharePage;
use crate::pages::stats::StatsPage;
use crate::pages::workspace::WorkspacePage;
use crate::state::AppState;
use leptos::prelude::*;
use leptos_meta::provide_meta_context;
use leptos_router::components::{Route, Router, Routes};
use leptos_router::StaticSegment;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    let state = AppState::new();
    provide_context(state);

    view! {
        <Router>
            <Routes fallback=|| view! { <h1>"404 - 页面未找到"</h1> }>
                // TODO: 练习 - 添加更多路由配置
                // 提示: 需要添加 /doc/:id, /activity, /board, /stats, /auth, /workspace,
                //       /members, /share/:id 等路由，注意哪些需要用 Layout 包裹
                <Route path=StaticSegment("") view=move || view! { <Layout><EditorPage/></Layout> }/>
                <Route path=StaticSegment("doc/:id") view=move || view! { <Layout><EditorPage/></Layout> }/>
                <Route path=StaticSegment("activity") view=move || view! { <Layout><ActivityPage/></Layout> }/>
                <Route path=StaticSegment("board") view=move || view! { <Layout><BoardPage/></Layout> }/>
                <Route path=StaticSegment("stats") view=move || view! { <Layout><StatsPage/></Layout> }/>
                <Route path=StaticSegment("auth") view=AuthPage/>
                <Route path=StaticSegment("workspace") view=WorkspacePage/>
                <Route path=StaticSegment("members") view=MembersPage/>
                <Route path=StaticSegment("share/:id") view=SharePage/>
            </Routes>
        </Router>
    }
}

#[component]
fn Layout(children: Children) -> impl IntoView {
    // TODO: 练习 - 实现页面布局组件
    // 提示: 使用 use_context 获取 AppState，读取 theme，渲染侧边栏、标签栏和内容区
    let state = use_context::<AppState>().expect("AppState not provided");
    let theme = state.theme;

    view! {
        <div class="app-container" data-theme={move || theme.get()}>
            <Sidebar/>
            <div class="main-area">
                <TabBar/>
                <div class="content-area">
                    {children()}
                </div>
            </div>
        </div>
    }
}
