 // ============================================================
 // 练习 e303: 用户注册 — 参考答案
 //
 // 核心知识点:
 //   - Server Function 处理注册逻辑
 //   - 密码哈希存储
 //   - 用户名唯一性验证
 //   - 注册成功后自动登录
 // ============================================================
 
 use leptos::prelude::*;
 use leptos::prelude::ServerFnError;
 
 #[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
 pub struct RegisterData {
     pub username: String,
     pub password: String,
     pub confirm_password: String,
 }
 
 #[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
 pub struct UserInfo {
     pub id: i32,
     pub username: String,
 }
 
 static USERS: std::sync::Mutex<Vec<(i32, String, String)>> = std::sync::Mutex::new(Vec::new());
 
 #[server]
 async fn register(data: RegisterData) -> Result<UserInfo, ServerFnError> {
     if data.password != data.confirm_password {
         return Err(ServerFnError::ServerError("两次密码不一致".into()));
     }
     let mut users = USERS.lock().map_err(|e| ServerFnError::ServerError(e.to_string()))?;
     if users.iter().any(|(_, u, _)| u == &data.username) {
         return Err(ServerFnError::ServerError("用户名已存在".into()));
     }
     // ponytail: use bcrypt::hash() in production
     let password_hash = format!("hashed_{}", data.password);
     let id = users.len() as i32 + 1;
     users.push((id, data.username.clone(), password_hash));
     let session_id = format!("session_user_{}", id);
     leptos::cookies::set_cookie(
         &leptos::cookies::Cookie::new("session_id", session_id)
     ).map_err(|e| ServerFnError::ServerError(e.to_string()))?;
     Ok(UserInfo { id, username: data.username })
 }
 
 #[server]
 async fn check_auth() -> Result<Option<UserInfo>, ServerFnError> {
     let cookies = leptos::cookies::Cookies::new();
     match cookies.get("session_id") {
         Some(cookie) if cookie.value().starts_with("session_user_") => {
             let id_str = cookie.value().trim_start_matches("session_user_");
             let id: i32 = id_str.parse().map_err(|_| ServerFnError::ServerError("invalid session".into()))?;
             let users = USERS.lock().map_err(|e| ServerFnError::ServerError(e.to_string()))?;
             match users.iter().find(|(uid, _, _)| *uid == id) {
                 Some((_, username, _)) => Ok(Some(UserInfo { id, username: username.clone() })),
                 None => Ok(None),
             }
         }
         _ => Ok(None),
     }
 }
 
 #[component]
 fn Exercise() -> impl IntoView {
     let register_action: Action<RegisterData, Result<UserInfo, ServerFnError>>
         = Action::new(|input: &RegisterData| {
             let input = input.clone();
             async move { register(input).await }
         });
 
     let auth_resource = Resource::new(|| (), |_| async move { check_auth().await.ok().flatten() });
 
     let (username, set_username) = signal(String::new());
     let (password, set_password) = signal(String::new());
     let (confirm_password, set_confirm_password) = signal(String::new());
 
     let logged_in_user = move || {
         register_action.value().get().and_then(|r| r.ok())
             .or_else(|| auth_resource.get().flatten())
     };
 
     let register_error = move || {
         register_action.value().get().and_then(|r| r.err())
     };
 
     view! {
         <div>
             <p>"练习 303 — 用户注册 (auth_register)"</p>
             {move || match logged_in_user() {
                 Some(user) => view! {
                     <div>
                         <p>"注册成功! 欢迎, " {user.username.clone()} "!"</p>
                         <p>"用户 ID: " {user.id.to_string()}</p>
                     </div>
                 }.into_any(),
                 None => view! {
                     <div>
                         <input type="text" placeholder="用户名"
                            prop:value={username}
                            on:input=move |ev| set_username(event_target_value(&ev)) />
                         <input type="password" placeholder="密码"
                            prop:value={password}
                            on:input=move |ev| set_password(event_target_value(&ev)) />
                         <input type="password" placeholder="确认密码"
                            prop:value={confirm_password}
                            on:input=move |ev| set_confirm_password(event_target_value(&ev)) />
                         <button
                            on:click=move |_| {
                                register_action.dispatch(RegisterData {
                                    username: username.get(),
                                    password: password.get(),
                                    confirm_password: confirm_password.get(),
                                });
                            }
                            disabled=move || register_action.pending().get()
                         >
                             {move || if register_action.pending().get() { "注册中..." } else { "注册" }}
                         </button>
                         {move || register_error().map(|e| view! { <p style="color:red">{e.to_string()}</p> })}
                     </div>
                 }.into_any(),
             }}
         </div>
     }
 }
 
 fn main() {
     mount_to_body(Exercise);
 }
