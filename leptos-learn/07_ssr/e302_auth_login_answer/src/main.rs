 // ============================================================
 // 练习 e302: 登录认证 — 参考答案
 //
 // 核心知识点:
 //   - Server Function 验证用户身份
 //   - 创建 Session 并通过 Cookie 传递
 //   - Action 处理表单提交
 //   - 登录状态管理
 // ============================================================
 
 use leptos::prelude::*;
 use leptos::prelude::ServerFnError;
 
 #[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
 pub struct LoginData {
     pub username: String,
     pub password: String,
 }
 
 #[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
 pub struct UserInfo {
     pub id: i32,
     pub username: String,
 }
 
 #[server]
 async fn login(data: LoginData) -> Result<UserInfo, ServerFnError> {
     if data.username != "admin" || data.password != "password123" {
         return Err(ServerFnError::ServerError("用户名或密码错误".into()));
     }
     let session_id = format!("session_admin_{}", data.username);
     leptos::cookies::set_cookie(
         &leptos::cookies::Cookie::new("session_id", session_id)
     ).map_err(|e| ServerFnError::ServerError(e.to_string()))?;
     Ok(UserInfo { id: 1, username: "admin".into() })
 }
 
 #[server]
 async fn check_auth() -> Result<Option<UserInfo>, ServerFnError> {
     let cookies = leptos::cookies::Cookies::new();
     match cookies.get("session_id") {
         Some(cookie) if cookie.value().starts_with("session_admin_") => {
             let username = cookie.value().trim_start_matches("session_admin_");
             Ok(Some(UserInfo { id: 1, username: username.into() }))
         }
         _ => Ok(None),
     }
 }
 
 #[component]
 fn Exercise() -> impl IntoView {
     let login_action = Action::new(|input: &LoginData| {
         let input = input.clone();
         async move { login(input).await }
     });
 
     let auth_resource = Resource::new(|| (), |_| async move { check_auth().await.ok().flatten() });
 
     let (username, set_username) = signal(String::new());
     let (password, set_password) = signal(String::new());
 
     let logged_in_user = move || {
         login_action.value().get().and_then(|r| r.ok())
             .or_else(|| auth_resource.get().flatten())
     };
 
     let login_error = move || {
         login_action.value().get().and_then(|r| r.err())
     };
 
     view! {
         <div>
             <p>"练习 302 — 登录认证 (auth_login)"</p>
             {move || match logged_in_user() {
                 Some(user) => {
                     view! {
                         <div>
                             <p>"欢迎, " {user.username.clone()} "!"</p>
                             <p>"用户 ID: " {user.id.to_string()}</p>
                         </div>
                     }.into_any()
                 }
                 None => {
                     view! {
                         <div>
                             <input
                                 type="text"
                                 placeholder="用户名"
                                 prop:value={username}
                                 on:input=move |ev| set_username(event_target_value(&ev))
                             />
                             <input
                                 type="password"
                                 placeholder="密码"
                                 prop:value={password}
                                 on:input=move |ev| set_password(event_target_value(&ev))
                             />
                             <button
                                 on:click=move |_| {
                                     login_action.dispatch(LoginData {
                                         username: username.get(),
                                         password: password.get(),
                                     });
                                 }
                                 disabled=move || login_action.pending().get()
                             >
                                 {move || if login_action.pending().get() { "登录中..." } else { "登录" }}
                             </button>
                             {move || login_error().map(|e| view! { <p style="color:red">{e.to_string()}</p> })}
                         </div>
                     }.into_any()
                 }
             }}
         </div>
     }
 }
 
 fn main() {
     mount_to_body(Exercise);
 }
