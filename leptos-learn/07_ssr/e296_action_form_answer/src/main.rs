 // ============================================================
 // 练习 e296: Action 表单 (action_form) — 参考答案
 //
 // 核心知识点:
 //   - <ActionForm action={...}/> 组件
 //   - Action::new 与 #[server] 函数结合
 //   - 表单数据自动序列化为服务端函数参数
 //   - 显示异步操作结果和加载状态
 // ============================================================
 
 use leptos::prelude::*;
 use leptos::prelude::ServerFnError;
 
 #[server(SubmitGreeting)]
 pub async fn submit_greeting(name: String) -> Result<String, ServerFnError> {
     Ok(format!("你好，{}！表单提交成功。", name))
 }
 
 #[component]
 fn Exercise() -> impl IntoView {
     let action = ServerAction::<SubmitGreeting>::new();
 
     view! {
         <div>
             <h2>"练习 296: Action 表单"</h2>
 
             <ActionForm action={action}>
                 <input type="text" name="name" placeholder="输入你的名字" />
                 <button type="submit">"提交"</button>
             </ActionForm>
 
             <p>{move || if action.pending().get() { "提交中..." } else { "等待提交" }}</p>
 
             <div>
                 {move || action.value().get().map(|result| match result {
                     Ok(msg) => view! { <p style="color: green">{msg}</p> }.into_any(),
                     Err(e) => view! { <p style="color: red">{format!("错误：{}", e)}</p> }.into_any(),
                 })}
             </div>
         </div>
     }
 }
 
 fn main() {
     mount_to_body(Exercise);
 }
