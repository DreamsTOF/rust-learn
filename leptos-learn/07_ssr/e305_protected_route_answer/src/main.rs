 // ============================================================
 // 练习 e305: 受保护路由 — 参考答案
 //
 // 核心知识点:
 //   - Auth Guard: 根据认证状态条件渲染
 //   - 未登录时显示登录提示
 //   - 使用 Resource 加载认证状态
 // ============================================================
 
 use leptos::prelude::*;
 use leptos::prelude::ServerFnError;
 
 #[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
 pub struct UserInfo {
     pub id: i32,
     pub username: String,
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
 fn ProtectedPage(children: Children) -> impl IntoView {
     // ponytail: in real SSR, also check at router level with leptos_router guards
     let auth = Resource::new(|| (), |_| async move { check_auth().await.ok().flatten() });
 
     move || match auth.get().flatten() {
         Some(user) => view! {
             <div>
                 <p>"欢迎, " {user.username} "!"</p>
                 {children()}
             </div>
         }.into_any(),
         None => view! {
             <div>
                 <p>"请先登录以访问受保护内容"</p>
                 <p>"请使用 e302-auth-login 练习中的登录表单进行登录。"</p>
             </div>
         }.into_any(),
     }
 }
 
 #[component]
 fn Exercise() -> impl IntoView {
     view! {
         <div>
             <p>"练习 305 — 受保护路由 (protected_route)"</p>
             <ProtectedPage>
                 <p>"这是受保护的内容，只有登录后才能看到。"</p>
                 <ul>
                     <li>"机密数据 #1"</li>
                     <li>"机密数据 #2"</li>
                     <li>"机密数据 #3"</li>
                 </ul>
             </ProtectedPage>
         </div>
     }
 }
 
 fn main() {
     mount_to_body(Exercise);
 }
