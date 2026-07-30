// 练习 251 — on:input 事件
//
// 目标: 创建一个受控输入框，在输入时实时显示当前输入文本。
//
// 学习要点:
// - on:input 事件在每次输入时触发
// - event_target_value(&ev) 从事件对象中提取输入框的当前值
// - RwSignal 创建可读写的响应式信号

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
            <h2>"练习 251: on:input 事件"</h2>

            {/*
                添加受控输入框:
                1. type="text"
                2. 绑定 on:input 事件
                3. 使用 event_target_value(&ev) 获取输入值
                4. 通过 value.set() 更新信号
            */}
            <input type="text" on:input=move |ev| todo!() />

            {/*
                实时显示当前输入的内容
                直接在视图中使用 {value}
            */}
            <p>"你输入的是: " {todo!()}</p>
        </div>
    }
}
