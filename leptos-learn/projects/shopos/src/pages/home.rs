use leptos::prelude::*;

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <div class="page-container">
            <h1>"欢迎使用 ShopOS 电商管理后台"</h1>
            <p>"请通过左侧菜单导航访问各项功能。"</p>
        </div>
    }
}
