
 // ============================================================
 // 练习 e303: 用户注册 — 注册表单 + 密码哈希 + 唯一性验证
 //
 // 核心知识点:
 //   - Server Function 处理注册逻辑
 //   - 密码哈希存储 (bcrypt)
 //   - 用户名唯一性验证
 //   - 注册成功后自动登录 (设置 session cookie)
 //
 // 难度: ⭐⭐⭐ (最低引导，仅描述目标)
 //
 // 数据流:
 //   客户端                             服务端
 //   ──────                             ──────
 //   POST /api/register ──credentials──→  验证输入
 //                                      检查用户名唯一
 //                                      哈希密码
 //                                      创建用户
 //                                      设置 session cookie
 //   ←── { user: "Alice" } ──────────  返回用户信息
 // ============================================================
 
 use leptos::prelude::*;
 use leptos::prelude::ServerFnError;
 
 // TODO: 定义 RegisterData 结构体 (username, password, confirm_password)
 // #[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
 // pub struct RegisterData {
 //     pub username: String,
 //     pub password: String,
 //     pub confirm_password: String,
 // }
 
 // TODO: 定义 UserInfo 结构体 (id, username) — 与 e302 相同
 // #[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
 // pub struct UserInfo {
 //     pub id: i32,
 //     pub username: String,
 // }
 
 // TODO: 定义 "数据库" — 使用 RwSignal<Vec<...>> 存储已注册用户
 // 注意: RwSignal 在 SSR 中需要特别处理，这里用静态 Mutex 模拟
 // static USERS: std::sync::Mutex<Vec<(i32, String, String)>> = std::sync::Mutex::new(Vec::new());
 // 元组: (id, username, password_hash)
 
 // TODO: 编写 register server function
 // - 验证 password == confirm_password
 // - 检查用户名唯一性 (遍历 USERS)
 // - 使用 bcrypt::hash() 哈希密码 (或简单用 format!("hashed_{}", password) 模拟)
 // - 创建新用户记录，分配 id
 // - 设置 session cookie (格式: "session_user_{id}")
 // - 返回 UserInfo
 // #[server]
 // async fn register(data: RegisterData) -> Result<UserInfo, ServerFnError> {
 //     // 验证密码一致性
 //     // 检查用户名唯一
 //     // 哈希密码
 //     // 存储用户
 //     // 设置 session cookie
 //     // 返回 UserInfo
 // }
 
 // TODO: 编写 check_auth server function (与 e302 相同逻辑)
 // #[server]
 // async fn check_auth() -> Result<Option<UserInfo>, ServerFnError> {
 //     // 从 cookie 读取 session
 //     // 验证有效性
 //     // 返回用户信息或 None
 // }
 
 #[component]
 fn Exercise() -> impl IntoView {
     // 目标:
     // 1. Action 处理注册 (action.dispatch(RegisterData))
     // 2. Resource 检查初始认证状态
     // 3. 未注册/未登录: 显示注册表单 (username, password, confirm_password)
     // 4. 已注册/已登录: 显示 "欢迎, {username}" + 注册成功消息
     // 5. 显示错误信息 (密码不匹配，用户名已存在等)
     //
     // 提示: action.value() → ReadSignal<Option<Result<UserInfo, ServerFnError>>>
 
     view! {
         <div>
             <p>"练习 303 — 用户注册 (auth_register)"</p>
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
 // ### 结构体 & 静态存储
 // ```rust
 // #[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
 // pub struct RegisterData {
 //     pub username: String,
 //     pub password: String,
 //     pub confirm_password: String,
 // }
 //
 // #[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
 // pub struct UserInfo {
 //     pub id: i32,
 //     pub username: String,
 // }
 //
 // static USERS: std::sync::Mutex<Vec<(i32, String, String)>> = std::sync::Mutex::new(Vec::new());
 // ```
 //
 // ### server function
 // ```rust
 // #[server]
 // async fn register(data: RegisterData) -> Result<UserInfo, ServerFnError> {
 //     if data.password != data.confirm_password {
 //         return Err(ServerFnError::ServerError("两次密码不一致".into()));
 //     }
 //     let mut users = USERS.lock().map_err(|e| ServerFnError::ServerError(e.to_string()))?;
 //     if users.iter().any(|(_, u, _)| u == &data.username) {
 //         return Err(ServerFnError::ServerError("用户名已存在".into()));
 //     }
 //     // 实际项目使用 bcrypt::hash(&data.password, DEFAULT_COST)?
 //     let password_hash = format!("hashed_{}", data.password);
 //     let id = users.len() as i32 + 1;
 //     users.push((id, data.username.clone(), password_hash));
 //     let session_id = format!("session_user_{}", id);
 //     leptos::cookies::set_cookie(
 //         &leptos::cookies::Cookie::new("session_id", session_id)
 //     ).map_err(|e| ServerFnError::ServerError(e.to_string()))?;
 //     Ok(UserInfo { id, username: data.username })
 // }
 // ```
 //
 // ### 组件
 // ```rust
 // let register_action: Action<RegisterData, Result<UserInfo, ServerFnError>>
 //     = Action::new(|input: &RegisterData| {
 //         let input = input.clone();
 //         async move { register(input).await }
 //     });
 //
 // let auth_resource = Resource::new(|| (), |_| async move { check_auth().await.ok().flatten() });
 //
 // let (username, set_username) = signal(String::new());
 // let (password, set_password) = signal(String::new());
 // let (confirm_password, set_confirm_password) = signal(String::new());
 // ```
 // 视图逻辑与 e302 类似，使用 match logged_in_user() 切换表单/欢迎页
 //
 // </details>
