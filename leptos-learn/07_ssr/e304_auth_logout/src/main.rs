
 // ============================================================
 // 练习 e304: 退出登录 — Session 销毁 + Cookie 清除 + UI 状态更新
 //
 // 核心知识点:
 //   - Server Function 销毁 session
 //   - 清除客户端 cookie (设置过期时间为过去)
 //   - 退出后 UI 立即响应 (回到登录界面)
 //
 // 难度: ⭐⭐ (关键 TODO)
 //
 // 数据流:
 //   客户端                           服务端
 //   ──────                           ──────
 //   POST /api/logout ──Cookie:sess→   清除 session
 //   ←── Set-Cookie: max-age=0 ────   清除 cookie
 //   ←── { ok: true }
 // ============================================================
 
 use leptos::prelude::*;
 use leptos::prelude::ServerFnError;
 
 // 假设用户已经通过 e302/e303 的方式登录
 // 当前用户信息由 check_auth 返回
 
 // TODO: 定义 UserInfo 结构体 (id, username)
 // #[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
 // pub struct UserInfo {
 //     pub id: i32,
 //     pub username: String,
 // }
 
 // TODO: 编写 logout server function
 // - 从 cookie 读取 session_id
 // - 使用 leptos::cookies::remove_cookie() 或设置过期时间为过去的 cookie 来清除
 // - 返回成功消息
 // #[server]
 // async fn logout() -> Result<String, ServerFnError> {
 //     // 读取当前 session
 //     // 清除 cookie
 //     // 返回 "已退出登录"
 // }
 
 // TODO: 编写 check_auth server function (与 e302/e303 相同)
 // #[server]
 // async fn check_auth() -> Result<Option<UserInfo>, ServerFnError> {
 //     // 从 cookie 读取 session
 //     // 验证有效性
 //     // 返回用户信息或 None
 // }
 
 #[component]
 fn Exercise() -> impl IntoView {
     // 目标:
     // 1. 创建两个 Action: login_action (登录), logout_action (退出)
     // 2. Resource 检查初始认证状态
     // 3. 未登录: 显示用户名/密码输入框 + 登录按钮
     // 4. 已登录: 显示 "欢迎, {username}" + 退出按钮
     // 5. 退出后: 自动回到登录表单
     // 6. 退出后清除 cookie 使服务端不再识别 session
     //
     // 提示:
     // - logout_action.dispatch(()) 触发退出
     // - 退出成功后 login_action.value() 应为 None
     // - logged_in_user 需要 combine login_action 和 auth_resource
     // - 退出后应 refetch auth_resource 或使用 action.value() 的 None 触发 UI 更新
 
     view! {
         <div>
             <p>"练习 304 — 退出登录 (auth_logout)"</p>
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
 // #[server]
 // async fn logout() -> Result<String, ServerFnError> {
 //     leptos::cookies::remove_cookie("session_id")
 //         .map_err(|e| ServerFnError::ServerError(e.to_string()))?;
 //     Ok("已退出登录".into())
 // }
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
 // ### 组件
 // ```rust
 // let login_action: Action<String, Result<UserInfo, ServerFnError>>
 //     = Action::new(|input: &String| {
 //         let input = input.clone();
 //         async move { /* 模拟登录 */ Ok(UserInfo { id: 1, username: input }) }
 //     });
 //
 // let logout_action = Action::new(|_: &()| async move { logout().await });
 //
 // let auth_resource = Resource::new(|| (), |_| async move { check_auth().await.ok().flatten() });
 //
 // let logged_in_user = move || {
 //     login_action.value().get().and_then(|r| r.ok())
 //         .or_else(|| auth_resource.get().flatten())
 // };
 // ```
 //
 // </details>
