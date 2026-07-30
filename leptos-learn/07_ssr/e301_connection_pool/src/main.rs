
 // ============================================================
 // 练习 e301: 连接池 — 创建并共享数据库连接池
 //
 // 核心知识点:
 //   - 使用 sqlx::SqlitePool 创建数据库连接池
 //   - 通过 provide_context / use_context 在 server fn 间共享池
 //   - 在 axum State 中注册连接池的典型模式
 //
 // 难度: ⭐⭐ (关键 TODO)
 // ============================================================
 
 use leptos::prelude::*;
 use leptos::prelude::ServerFnError;
 
 /// 包装数据库连接池，使其可通过 provide_context 共享
 /// 在实际 SSR 应用中，此处应使用 sqlx::SqlitePool
 #[derive(Clone)]
 struct DbPool(String);
 
 // TODO: 使用 #[server] 定义一个 server function
 // 该函数通过 use_context::<DbPool>() 获取连接池并返回待办事项数量
 // 提示: use_context::<T>() 返回 Option<T>
 #[server]
 async fn get_todo_count() -> Result<i32, ServerFnError> {
     // 第 1 步: 用 use_context 获取 DbPool
     // 第 2 步: 模拟数据库查询 (返回 Ok(42))
     // 真实查询: sqlx::query_scalar("SELECT COUNT(*) FROM todos")
     //     .fetch_one(&pool.0).await
     //     .map_err(|e| ServerFnError::ServerError(e.to_string()))?;
     Ok(42)
 }
 
 #[component]
 fn Exercise() -> impl IntoView {
     // TODO: 调用 provide_context 将 DbPool 注入到组件树
     // provide_context(DbPool("sqlite::memory:".to_string()));
 
     // TODO: 创建 Resource，调用 get_todo_count server function
     // let count = Resource::new(|| (), |_| async move { get_todo_count().await.ok() });
 
     view! {
         <div>
             <p>"练习 301 — 连接池 (connection_pool)"</p>
             // TODO: 显示待办事项数量
             // <p>"待办事项数量: " {move || count.map(|c| c.map(|v| v.to_string()).unwrap_or_else(|| "加载中...".to_string()))}</p>
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
 // ```rust
 // #[server]
 // async fn get_todo_count() -> Result<i32, ServerFnError> {
 //     let _pool = use_context::<DbPool>()
 //         .ok_or_else(|| ServerFnError::ServerError("DbPool not provided".into()))?;
 //     // let count: (i32,) = sqlx::query_scalar("SELECT COUNT(*) FROM todos")
 //     //     .fetch_one(&pool.0).await
 //     //     .map_err(|e| ServerFnError::ServerError(e.to_string()))?;
 //     Ok(42)
 // }
 //
 // // 在组件中:
 // provide_context(DbPool("sqlite::memory:".to_string()));
 // let count = Resource::new(|| (), |_| async move { get_todo_count().await.ok() });
 // ```
 //
 // ### 关键知识点
 // - `use_context::<T>()` 从上下文中获取类型 T 的实例，返回 Option<T>
 // - `provide_context(value)` 将值注入当前组件及子组件的上下文
 // - `#[server]` 宏将函数标记为 server function，CSR 时自动生成 fetch 请求
 // - 真实 SSR 流程: axum main() 创建 Pool → 注入 State → server fn 通过 use_context 获取
 //
 // </details>
