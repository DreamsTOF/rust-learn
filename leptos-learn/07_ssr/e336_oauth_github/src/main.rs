// ============================================================
// 练习 e336: OAuth GitHub — OAuth 授权码流程
//
// 核心知识点:
//   - OAuth 2.0 authorization code flow 工作原理
//   - GitHub OAuth App 注册与回调 URL
//   - State 参数防止 CSRF 攻击
//   - Access token 交换与用户信息 API
//
// 难度: ⭐⭐⭐ (minimal guidance)
//
// 任务: 实现 GitHub OAuth 登录流程模拟
//   1. 定义 GitHubUser 结构体 (id, login, avatar_url, name)
//   2. 使用 signal / RwSignal 管理认证状态
//   3. "Login with GitHub" 按钮触发模拟授权
//   4. 显示用户头像、用户名、GitHub ID
//   5. "Logout" 按钮清除用户状态
// ============================================================

use leptos::prelude::*;

// TODO: 定义 GitHubUser 结构体

#[component]
fn Exercise() -> impl IntoView {
    // TODO: 管理 Option<GitHubUser> 状态
    // TODO: 实现 login / logout 回调

    view! {
        // TODO: 实现 OAuth UI
        //   - 未登录时显示 "Login with GitHub" 按钮
        //   - 已登录时显示用户信息卡片 + "Logout" 按钮
    }
}

fn main() {
    mount_to_body(Exercise);
}
