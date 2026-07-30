// 练习 252 — on:change 事件
//
// 目标: 创建一个输入框，在失焦（失去焦点）时显示输入的内容。
//
// 学习要点:
// - on:change 事件在输入框失焦且值发生变化时触发
// - 与 on:input（每输入一次都触发）不同，on:change 只在失焦时触发
// - event_target_value(&ev) 提取输入框的值

use leptos::prelude::*;

fn main() {
    // 使用 mount_to_body 将 Exercise 组件挂载到页面
    // 提示: mount_to_body(|| view! { <Exercise/> })
    todo!();
}

#[component]
fn Exercise() -> impl IntoView {
    // 创建 RwSignal<String> 信号，初始值为空字符串
    // 提示: RwSignal::new(String::new())
    let value = todo!();

    view! {
        <div>
            <h2>"练习 252: on:change 事件"</h2>

            {/*
                添加输入框:
                1. type="text"
                2. 绑定 on:change 事件
                3. 使用 event_target_value(&ev) 获取值
                4. 通过 value.set() 更新信号
            */}
            <input type="text" on:change=move |ev| todo!() />

            {/*
                显示失焦时提交的内容
            */}
            <p>"你提交的是: " {todo!()}</p>
        </div>
    }
}
