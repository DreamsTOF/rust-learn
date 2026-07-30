 // ============================================================
 // 练习 e301: 连接池 — 参考答案
 //
 // 核心知识点:
 //   - 使用 sqlx::SqlitePool 创建数据库连接池
 //   - 通过 provide_context / use_context 在 server fn 间共享池
 //   - 在 axum State 中注册连接池的典型模式
 // ============================================================
 
 use leptos::prelude::*;
 use leptos::prelude::ServerFnError;
 
 /// 包装数据库连接池，使其可通过 provide_context 共享
 #[derive(Clone)]
 struct DbPool(String);
 
 #[server]
 async fn get_todo_count() -> Result<i32, ServerFnError> {
     let _pool = use_context::<DbPool>()
         .ok_or_else(|| ServerFnError::ServerError("DbPool not provided".into()))?;
     // 真实场景: sqlx::query_scalar("SELECT COUNT(*) FROM todos")
     //     .fetch_one(&pool.0).await
     //     .map_err(|e| ServerFnError::ServerError(e.to_string()))?;
     Ok(42)
 }
 
 #[component]
 fn Exercise() -> impl IntoView {
     provide_context(DbPool("sqlite::memory:".to_string()));
 
     let count = Resource::new(|| (), |_| async move { get_todo_count().await.ok() });
 
     view! {
         <div>
             <p>"练习 301 — 连接池 (connection_pool)"</p>
             <p>
                 "待办事项数量: "
                 {move || count.map(|c| c.map(|v| v.to_string()).unwrap_or_else(|| "加载中...".to_string()))}
             </p>
         </div>
     }
 }
 
 fn main() {
     mount_to_body(Exercise);
 }
