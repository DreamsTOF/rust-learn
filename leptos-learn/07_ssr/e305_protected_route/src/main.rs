
 // ============================================================
 // 练习 e305: 受保护路由 — Auth Guard + 条件渲染 + 跳转
 //
 // 核心知识点:
 //   - 根据认证状态条件渲染页面内容
 //   - 未登录时自动跳转到登录表单
 //   - 使用 Resource 加载认证状态
 //   - 条件渲染受保护内容
 //
 // 难度: ⭐⭐ (关键 TODO)
 //
 // 常见 SSR 路由保护模式:
 //   路由层: leptos_router 的 <Route> guard (服务端检查)
 //   组件层: #[component] 内部检查 auth → 转发 (客户端检查)
 //   本练习: 组件层模式（CSR 友好）
 // ============================================================
 
 use leptos::prelude::*;
 use leptos::prelude::ServerFnError;
 
 // TODO: 定义 UserInfo 结构体
 // #[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
 // pub struct UserInfo {
 //     pub id: i32,
 //     pub username: String,
 // }
 
 // TODO: 编写 check_auth server function
 // 从 cookie 读取 session_id，验证并返回 UserInfo
 // 未认证时返回 Ok(None)
 
 // TODO: 编写 ProtectedPage 组件
 // - 接收 children: 受保护内容
 // - 使用 Resource 调用 check_auth
 // - 如果 auth 为 None: 显示 "请先登录" 消息
 // - 如果 auth 为 Some: 渲染 children
 // - 同时显示一个 "转到登录" 的链接/按钮
 // #[component]
 // fn ProtectedPage(children: Children) -> impl IntoView {
 //     // 检查认证状态
 //     // 根据状态渲染
 // }
 
 // TODO: 编写 LoginForm 组件 (简化版)
 // - 包含用户名输入框和登录按钮
 // - 使用 Action 调用 login server function
 // - 登录成功后更新 UI
 
 #[component]
 fn Exercise() -> impl IntoView {
     // 目标:
     // 1. 初始加载时检查认证状态 (use Resource + check_auth)
     // 2. 如果已登录: 显示 "受保护内容" + 欢迎消息 + 用户信息
     // 3. 如果未登录: 显示登录提示 + 登录表单
     // 4. 登录后: 自动显示受保护内容
     //
     // 挑战点:
     // - 使用 ProtectedPage 组件或直接在 Exercise 中实现守卫逻辑
     // - 登录后 refetch auth_resource 而不是依赖 action.value()
 
     view! {
         <div>
             <p>"练习 305 — 受保护路由 (protected_route)"</p>
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
 // ### server function
 // ```rust
 // #[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq)]
 // pub struct UserInfo { pub id: i32, pub username: String }
 //
 // #[server]
 // async fn check_auth() -> Result<Option<UserInfo>, ServerFnError> {
 //     let cookies = leptos::cookies::Cookies::new();
 //     match cookies.get("session_id") {
 //         Some(cookie) if cookie.value().starts_with("session_admin_") => {
 //             let username = cookie.value().trim_start_matches("session_admin_");
 //             Ok(Some(UserInfo { id: 1, username: username.into() }))
 //         }
 //         _ => Ok(None),
 //     }
 // }
 // ```
 //
 // ### ProtectedPage 组件 & Exercise
 // ```rust
 // #[component]
 // fn ProtectedPage(children: Children) -> impl IntoView {
 //     let auth = Resource::new(|| (), |_| async move { check_auth().await.ok().flatten() });
 //     move || match auth.get().flatten() {
 //         Some(user) => view! {
 //             <div>
 //                 <p>"欢迎, " {user.username} "!"</p>
 //                 {children()}
 //             </div>
 //         }.into_any(),
 //         None => view! {
 //             <p>"请先登录以访问受保护内容"</p>
 //         }.into_any(),
 //     }
 // }
 //
 // #[component]
 // fn Exercise() -> impl IntoView {
 //     view! {
 //         <ProtectedPage>
 //             <p>"这是受保护的内容，只有登录后才能看到。"</p>
 //         </ProtectedPage>
 //     }
 // }
 // ```
 //
 // </details>
