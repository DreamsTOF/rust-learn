// ============================================================
// 练习 e191: Action 与表单 — 简化版
//
// 核心知识点:
//   - Action 与 HTML 表单结合
//   - on:submit + ev.prevent_default()
//   - 表单数据收集与 dispatch
//   - action.pending() 控制按钮状态
//
// 难度: ⭐⭐
// ============================================================

use leptos::prelude::*;

#[component]
fn Exercise() -> impl IntoView {
    let (name, set_name) = signal(String::new());
    let (message, set_message) = signal(String::new());

    // TODO: 创建 Action，接收字符串输入并返回处理结果
    let action = Action::new(|input: &String| {
        let input = input.clone();
        async move {
            format!("你好，{}！消息已收到：{}", input, "处理成功")
        }
    });

    view! {
        <div>
            <p>"练习 191 — Action 与表单 (action_form)"</p>
            <form on:submit=move |ev| {
                // TODO: 阻止表单默认提交行为
                // 提示: ev.prevent_default()
                ev.prevent_default();
                // TODO: dispatch 表单数据
                action.dispatch(name.get());
            }>
                <div>
                    <label>"姓名: "
                        <input type="text"
                            prop:value=move || name.get()
                            on:input=move |ev| set_name(event_target_value(&ev))
                        />
                    </label>
                </div>
                <button type="submit" disabled=move || action.pending().get()>
                    {move || if action.pending().get() { "提交中..." } else { "提交" }}
                </button>
            </form>
            <hr />
            <div>
                {move || match action.value().get() {
                    None => view! { <p>"尚未提交"</p> }.into_any(),
                    Some(result) => view! { <pre>{result}</pre> }.into_any(),
                }}
            </div>
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
