 // ============================================================
 // 练习 e297: 表单提交到服务端 (form_submit_to_server) — 参考答案
 //
 // 核心知识点:
 //   - 完整流程: 表单 → ActionForm → #[server] 函数 → 响应回显
 //   - 服务端函数可返回结构化数据
 //   - 客户端通过 Action 的 value() 获取服务端结果
 // ============================================================
 
 use leptos::prelude::*;
 use leptos::prelude::ServerFnError;
 
 #[server(SubmitFeedback)]
 pub async fn submit_feedback(name: String, message: String) -> Result<String, ServerFnError> {
     if name.trim().is_empty() || message.trim().is_empty() {
         return Err(ServerFnError::ServerError("姓名和留言不能为空".into()));
     }
     Ok(format!("感谢你的反馈，{}！你的留言「{}」已收到。", name, message))
 }
 
 #[component]
 fn Exercise() -> impl IntoView {
     let action = ServerAction::<SubmitFeedback>::new();
 
     view! {
         <div>
             <h2>"练习 297: 表单提交到服务端"</h2>
 
             <ActionForm action={action}>
                 <input type="text" name="name" placeholder="你的名字" />
                 <br/>
                 <textarea name="message" placeholder="你的留言"></textarea>
                 <br/>
                 <button type="submit">"提交反馈"</button>
             </ActionForm>
 
             <div>
                 {move || if action.pending().get() {
                     view! { <p>"提交中..."</p> }.into_any()
                 } else {
                     action.value().get().map(|result| match result {
                         Ok(msg) => view! { <p style="color: green">{msg}</p> }.into_any(),
                         Err(e) => view! { <p style="color: red">{format!("错误：{}", e)}</p> }.into_any(),
                     }).unwrap_or_else(|| view! { <p>"等待提交..."</p> }.into_any())
                 }}
             </div>
         </div>
     }
 }
 
 fn main() {
     mount_to_body(Exercise);
 }
