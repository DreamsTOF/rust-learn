 // ============================================================
 // 练习 e296: Action 表单 (action_form)
 //
 // 核心知识点:
 //   - <ActionForm action={...}/> 组件
 //   - Action::new 与 #[server] 函数结合
 //   - 表单数据自动序列化为服务端函数参数
 //   - 显示异步操作结果和加载状态
 //
 // 难度: ⭐⭐
 // ============================================================
 
 use leptos::prelude::*;
 use leptos::prelude::ServerFnError;
 
 // TODO: 使用 #[server] 定义 submit_greeting
 // 接受 name: String，返回 Result<String, ServerFnError>
 // 函数体: Ok(format!("你好，{}！表单提交成功。", name))
 // 提示:
 //   #[server(SubmitGreeting)]
 //   pub async fn submit_greeting(name: String) -> Result<String, ServerFnError> {
 //       Ok(format!("你好，{}！表单提交成功。", name))
 //   }
 
 #[component]
 fn Exercise() -> impl IntoView {
     // TODO: 创建 Action 包装 submit_greeting
     // 提示: Action::new(|input: &SubmitGreeting| {
     //     let input = input.clone();
     //     async move { submit_greeting(input.name).await }
     // })
 
     view! {
         <div>
             <h2>"练习 296: Action 表单"</h2>
 
             // TODO: 使用 <ActionForm action={action}>
             // 包含:
             //   - <input type="text" name="name" placeholder="输入你的名字" />
             //   - <button type="submit">"提交"</button>
 
             // TODO: 显示 action 的状态和结果
             // - pending 时显示 "提交中..."
             // - value 成功时显示绿色结果，失败时显示红色错误
         </div>
     }
 }
 
 fn main() {
     mount_to_body(Exercise);
 }
 
 // <details>
 // <summary>参考答案</summary>
 //
 // ```rust
 // #[server(SubmitGreeting)]
 // pub async fn submit_greeting(name: String) -> Result<String, ServerFnError> {
 //     Ok(format!("你好，{}！表单提交成功。", name))
 // }
 //
 // let action = Action::new(|input: &SubmitGreeting| {
 //     let input = input.clone();
 //     async move { submit_greeting(input.name).await }
 // });
 //
 // <ActionForm action={action}>
 //     <input type="text" name="name" placeholder="输入你的名字" />
 //     <button type="submit">"提交"</button>
 // </ActionForm>
 // <p>{move || if action.pending().get() { "提交中..." } else { "等待提交" }}</p>
 // <div>
 //     {move || action.value().get().map(|result| match result {
 //         Ok(msg) => view! { <p style="color: green">{msg}</p> }.into_any(),
 //         Err(e) => view! { <p style="color: red">{format!("错误：{}", e)}</p> }.into_any(),
 //     })}
 // </div>
 // ```
 // </details>
