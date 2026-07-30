
 // ============================================================
 // 练习 e302: 登录认证 — 登录表单 + 服务端认证 + Session
 //
 // 核心知识点:
 //   - Server Function 验证用户身份
 //   - 创建 Session 并通过 Cookie 传递
 //   - Action 处理表单提交
 //   - 登录状态管理
 //
 // 难度: ⭐⭐⭐ (最低引导)
 //
 // Cookie / Session 数据流:
 //   客户端                         服务端
 //   ──────                         ──────
 //   POST /api/login ──credentials──→ 验证密码
 //   ←── Set-Cookie: session=abc ──  创建 Session
 //   GET /api/me ──Cookie: session→  校验 Session
 //   ←── { user: "Alice" } ───────  返回用户信息
 // ============================================================
 
 use leptos::prelude::*;
 use leptos::prelude::ServerFnError;
 
 /// 登录表单数据
 #[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
 pub struct LoginData {
     pub username: String,
     pub password: String,
 }
 
 /// 登录后返回的用户信息（不含密码）
 #[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
 pub struct UserInfo {
     pub id: i32,
     pub username: String,
 }
 
 // TODO: 编写 login server function
 // - 接收 LoginData 结构体作为参数
 // - 硬编码验证 "admin"/"password123"
 // - 成功时创建 session_id (例如 "session_admin_001")
 // - 使用 leptos::set_cookie(name, value) 设置 session cookie
 // - 返回 UserInfo { id: 1, username: "admin" }
 // - 失败时返回 Err(ServerFnError::ServerError("用户名或密码错误"))
 //
 // #[server]
 // async fn login(data: LoginData) -> Result<UserInfo, ServerFnError> {
 //     // 验证凭据
 //     // 创建 session
 //     // 设置 cookie
 //     // 返回用户信息
 // }
 
 // TODO: 编写 check_auth server function
 // - 从 cookie 读取 session_id (使用 leptos::cookies::Cookies)
 // - 验证 session 是否有效
 // - 返回 Option<UserInfo>
 //
 // #[server]
 // async fn check_auth() -> Result<Option<UserInfo>, ServerFnError> {
 //     // 从 cookie 中读取 session
 //     // 如果 session 有效，返回用户信息
 //     // 否则返回 Ok(None)
 // }
 
 #[component]
 fn Exercise() -> impl IntoView {
     // 目标:
     // 1. 创建 Action<LoginData, Result<UserInfo, ServerFnError>> 用于登录
     // 2. 创建 Resource 用于初始检查认证状态 (check_auth)
     // 3. 如果未登录: 显示用户名/密码输入框 + 登录按钮 + 错误信息
     // 4. 如果已登录: 显示 "欢迎, {username}" 和用户信息
     //
     // 提示: action.value() 返回 ReadSignal<Option<Result<UserInfo, ServerFnError>>>
     //       用 .map(|res| res.and_then(|r| r.ok())) 提取成功值
     //       用 resource.map(|r| r.flatten()) 提取 Option<UserInfo>
 
     view! {
         <div>
             <p>"练习 302 — 登录认证 (auth_login)"</p>
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
 // ### server function 实现
 // ```rust
 // #[server]
 // async fn login(data: LoginData) -> Result<UserInfo, ServerFnError> {
 //     if data.username != "admin" || data.password != "password123" {
 //         return Err(ServerFnError::ServerError("用户名或密码错误".into()));
 //     }
 //     let session_id = format!("session_admin_{}", data.username);
 //     leptos::cookies::set_cookie(
 //         &leptos::cookies::Cookie::new("session_id", session_id)
 //     ).map_err(|e| ServerFnError::ServerError(e.to_string()))?;
 //     Ok(UserInfo { id: 1, username: "admin".into() })
 // }
 //
 // #[server]
 // async fn check_auth() -> Result<Option<UserInfo>, ServerFnError> {
 //     let cookies = leptos::cookies::Cookies::new();
 //     let session_cookie = cookies.get("session_id")
 //         .ok_or_else(|| ServerFnError::ServerError("no cookies".into()))?;
 //     match session_cookie.value().as_str() {
 //         s if s.starts_with("session_admin_") => {
 //             let username = s.trim_start_matches("session_admin_");
 //             Ok(Some(UserInfo { id: 1, username: username.into() }))
 //         }
 //         _ => Ok(None)
 //     }
 // }
 // ```
 //
 // ### 组件实现
 // ```rust
 // let login_action = Action::new(|input: &LoginData| {
 //     let input = input.clone();
 //     async move { login(input).await }
 // });
 //
 // let auth_resource = Resource::new(|| (), |_| async move { check_auth().await.ok().flatten() });
 //
 // let logged_in_user = move || {
 //     login_action.value().get().and_then(|r| r.ok())
 //         .or_else(|| auth_resource.get().flatten())
 // };
 //
 // let (username, set_username) = signal(String::new());
 // let (password, set_password) = signal(String::new());
 // ```
 //
 // </details>
