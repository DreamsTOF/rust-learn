use leptos::prelude::*;

// 本题目标：创建一个表单，提交时阻止默认行为并显示输入数据。
//
// 步骤提示：
// 1. 创建响应式信号：name（String）、email（String）
// 2. 创建 submitted 信号：Option<(String, String)>，用于保存提交的数据
// 3. 编写 on_submit 处理函数，使用 ev::SubmitEvent::prevent_default()
// 4. 在 view 中绑定输入框的 prop:value 和 on:input
// 5. 条件渲染提交后的数据

#[component]
fn Exercise() -> impl IntoView {
    // TODO: 创建 name 信号（RwSignal::new(String::new())）
    // TODO: 创建 email 信号
    // TODO: 创建 submitted 信号，类型为 Option<(String, String)>

    // TODO: 编写 on_submit 回调
    // - 参数类型为 leptos::ev::SubmitEvent
    // - 调用 ev.prevent_default()
    // - 将 name 和 email 的当前值保存到 submitted

    view! {
        <div>
            <h2>"表单提交示例"</h2>
            <form on:submit=on_submit>
                <div>
                    <label>"姓名："</label>
                    {/*
                    TODO: 添加 <input type="text">
                    - prop:value 绑定到 name
                    - on:input 中使用 event_target_value(&ev) 更新 name
                    */}
                </div>
                <div>
                    <label>"邮箱："</label>
                    {/*
                    TODO: 添加 <input type="email">
                    - prop:value 绑定到 email
                    - on:input 中使用 event_target_value(&ev) 更新 email
                    */}
                </div>
                <button type="submit">"提交"</button>
            </form>
            {/*
            TODO: 当 submitted 有值时，显示提交的数据
            提示：submitted.get().map(|(n, e)| view! { ... })
            */}
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
