 // ============================================================
 // 练习 e297: 表单提交到服务端 (form_submit_to_server)
 //
 // 核心知识点:
 //   - 完整流程: 表单 → ActionForm → #[server] 函数 → 响应回显
 //   - 服务端函数可返回结构化数据
 //   - 客户端通过 Action 的 value() 获取服务端结果
 //
 // 难度: ⭐⭐
 // ============================================================
 
 use leptos::prelude::*;
 use leptos::prelude::ServerFnError;
 
 // TODO: 使用 #[server(SubmitFeedback)] 定义 submit_feedback
 // 接受 name: String 和 message: String
 // 返回 Result<String, ServerFnError>
 // 函数体: 验证输入非空，返回确认消息
 // 提示: 使用 serde 自动反序列化表单字段到函数参数
 
 #[component]
 fn Exercise() -> impl IntoView {
     // TODO: 创建 Action 包装 submit_feedback
     // 提示: Action::new(|input: &SubmitFeedback| {
     //     let input = input.clone();
     //     async move { submit_feedback(input.name.clone(), input.message.clone()).await }
     // })
 
     view! {
         <div>
             <h2>"练习 297: 表单提交到服务端"</h2>
 
             // TODO: 使用 <ActionForm action={action}>
             // 包含:
             //   - 名字输入框: <input type="text" name="name" placeholder="你的名字" />
             //   - 留言输入框: <textarea name="message" placeholder="你的留言"></textarea>
             //   - <button type="submit">"提交反馈"</button>
 
             // TODO: 显示服务端返回的消息
             // - pending 时显示 "提交中..."
             // - 有结果时显示服务端返回的确认信息
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
 // #[server(SubmitFeedback)]
 // pub async fn submit_feedback(name: String, message: String) -> Result<String, ServerFnError> {
 //     if name.trim().is_empty() || message.trim().is_empty() {
 //         return Err(ServerFnError::ServerError("姓名和留言不能为空".into()));
 //     }
 //     Ok(format!("感谢你的反馈，{}！你的留言「{}」已收到。", name, message))
 // }
 //
 // let action = Action::new(|input: &SubmitFeedback| {
 //     let input = input.clone();
 //     async move { submit_feedback(input.name.clone(), input.message.clone()).await }
 // });
 //
 // <ActionForm action={action}>
 //     <input type="text" name="name" placeholder="你的名字" />
 //     <br/>
 //     <textarea name="message" placeholder="你的留言"></textarea>
 //     <br/>
 //     <button type="submit">"提交反馈"</button>
 // </ActionForm>
 // <div>
 //     {move || if action.pending().get() {
 //         view! { <p>"提交中..."</p> }.into_any()
 //     } else {
 //         action.value().get().map(|result| match result {
 //             Ok(msg) => view! { <p style="color: green">{msg}</p> }.into_any(),
 //             Err(e) => view! { <p style="color: red">{format!("错误：{}", e)}</p> }.into_any(),
 //         }).unwrap_or_else(|| view! { <p>"等待提交..."</p> }.into_any())
 //     }}
 // </div>
 // ```
 // </details>
