 // ============================================================
 // 练习 e304: 退出登录 — 参考答案
 //
 // 核心知识点:
 //   - Server Function 销毁 session
 //   - 清除客户端 cookie
 //   - 退出后 UI 立即响应
 // ============================================================
 
 use leptos::prelude::*;
 use leptos::prelude::ServerFnError;
 
 #[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
 pub struct UserInfo {
     pub id: i32,
     pub username: String,
 }
 
 #[server]
 async fn logout() -> Result<String, ServerFnError> {
     leptos::cookies::remove_cookie("session_id")
         .map_err(|e| ServerFnError::ServerError(e.to_string()))?;
     Ok("已退出登录".into())
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
     let login_action: Action<String, Result<UserInfo, ServerFnError>>
         = Action::new(|input: &String| {
             let input = input.clone();
             async move {
                 // 模拟登录 — 真实场景调用 login server function
                 if input.is_empty() {
                     Err(ServerFnError::ServerError("用户名不能为空".into()))
                 } else {
                     Ok(UserInfo { id: 1, username: input })
                 }
             }
         });
 
     let logout_action = Action::new(|_: &()| async move { logout().await });
 
     let auth_resource = Resource::new(|| (), |_| async move { check_auth().await.ok().flatten() });
 
     let logged_in_user = move || {
         login_action.value().get().and_then(|r| r.ok())
             .or_else(|| auth_resource.get().flatten())
     };
 
     let (username, set_username) = signal(String::new());
 
     view! {
         <div>
             <p>"练习 304 — 退出登录 (auth_logout)"</p>
             {move || match logged_in_user() {
                 Some(user) => view! {
                     <div>
                         <p>"欢迎, " {user.username.clone()} "!"</p>
                         <button
                             on:click=move |_| {
                                 logout_action.dispatch(());
                                 auth_resource.refetch();
                             }
                             disabled=move || logout_action.pending().get()
                         >
                             {move || if logout_action.pending().get() { "退出中..." } else { "退出登录" }}
                         </button>
                         {move || logout_action.value().get().and_then(|r| r.ok()).map(|msg| {
                             view! { <p>{msg}</p> }
                         })}
                     </div>
                 }.into_any(),
                 None => view! {
                     <div>
                         <input type="text" placeholder="用户名"
                            prop:value={username}
                            on:input=move |ev| set_username(event_target_value(&ev)) />
                         <button
                             on:click=move |_| {
                                 login_action.dispatch(username.get());
                             }
                             disabled=move || login_action.pending().get()
                         >
                             {move || if login_action.pending().get() { "登录中..." } else { "登录" }}
                         </button>
                         {move || login_action.value().get().and_then(|r| r.err()).map(|e| {
                             view! { <p style="color:red">{e.to_string()}</p> }
                         })}
                     </div>
                 }.into_any(),
             }}
         </div>
     }
 }
 
 fn main() {
     mount_to_body(Exercise);
 }
