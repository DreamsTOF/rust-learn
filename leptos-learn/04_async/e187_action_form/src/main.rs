// ============================================================
// 练习 e187: Action + 表单 (action_form)
//
// 核心知识点:
//   - Action 与 HTML 表单结合
//   - on:submit + ev.prevent_default() 阻止默认提交
//   - 从表单收集数据并 dispatch 给 Action
//   - action.pending() 控制提交按钮状态
//
// 难度: ⭐⭐⭐
// ============================================================
//
// 说明: leptos 0.8 nightly 中 <ActionForm> 仅支持 ServerAction（SSR）。
// 客户端场景使用标准 <form> + on:submit 手动 dispatch。

use leptos::prelude::*;

#[component]
fn Exercise() -> impl IntoView {
    // 表单数据模型
    let (name, set_name) = signal(String::new());
    let (rating, set_rating) = signal(String::from("5"));
    let (comment, set_comment) = signal(String::new());

    // TODO: 创建 Action，处理表单提交
    let action = Action::new(|input: &(String, String, String)| {
        let (name, rating, comment) = input.clone();
        async move {
            format!(
                "感谢 {} 的反馈！\n评分: {} 星\n留言: {}",
                name, rating, comment
            )
        }
    });

    view! {
        <div>
            <p>"练习 187 — Action + 表单 (action_form)"</p>

            <form on:submit=move |ev| {
                ev.prevent_default();
                ev.stop_propagation();
                action.dispatch((
                    name.get(),
                    rating.get(),
                    comment.get(),
                ));
            }>
                <div>
                    <label>"姓名: "
                        <input
                            type="text"
                            name="name"
                            prop:value=move || name.get()
                            on:input=move |ev| set_name(event_target_value(&ev))
                        />
                    </label>
                </div>
                <div>
                    <label>"评分: "
                        <select
                            name="rating"
                            prop:value=move || rating.get()
                            on:change=move |ev| set_rating(event_target_value(&ev))
                        >
                            <option value="5">"⭐⭐⭐⭐⭐"</option>
                            <option value="4">"⭐⭐⭐⭐"</option>
                            <option value="3">"⭐⭐⭐"</option>
                            <option value="2">"⭐⭐"</option>
                            <option value="1">"⭐"</option>
                        </select>
                    </label>
                </div>
                <div>
                    <label>"留言: "
                        <textarea
                            name="comment"
                            prop:value=move || comment.get()
                            on:input=move |ev| set_comment(event_target_value(&ev))
                        ></textarea>
                    </label>
                </div>
                <button type="submit" disabled=move || action.pending().get()>
                    {move || if action.pending().get() { "提交中..." } else { "提交反馈" }}
                </button>
            </form>

            <hr />
            <div>
                <h3>"提交结果:"</h3>
                {move || match action.value().get() {
                    None => view! { <p>"还没有提交反馈。"</p> }.into_any(),
                    Some(result) => view! { <pre>{result}</pre> }.into_any(),
                }}
            </div>
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
// ### 核心代码
// ```rust
// let action = Action::new(|input: &(String, String, String)| {
//     let (name, rating, comment) = input.clone();
//     async move {
//         format!("感谢 {} 的反馈！评分: {}", name, rating)
//     }
// });
//
// <form on:submit=move |ev| {
//     ev.prevent_default();
//     action.dispatch((name.get(), rating.get(), comment.get()));
// }>
// ```
//
// ### 知识点
// - 使用 `on:submit` + `ev.prevent_default()` 拦截表单提交
// - 手动收集表单信号值，调用 `action.dispatch(input)` 触发异步处理
// - 结合 `action.pending()` 在提交时禁用按钮
// - 在 SSR 场景中可使用 `<ActionForm>` 组件自动处理表单序列化
// - 表单字段使用受控组件模式（信号绑定 prop:value + on:input）
//
// </details>
