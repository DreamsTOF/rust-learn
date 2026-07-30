// 练习 255 — 受控文本域 (textarea)
//
// 目标: 创建受控 textarea，并在下方显示当前字符计数。
//
// 学习要点:
// - <textarea> 的受控模式与 <input> 类似
// - prop:value 绑定信号到 textarea 的内容
// - on:input 事件将用户输入写回信号
// - 字符计数通过 value.read().len() 获取

use leptos::prelude::*;

fn main() {
    // 使用 mount_to_body 将 Exercise 组件挂载到页面
    // 提示: mount_to_body(|| view! { <Exercise/> })
    todo!();
}

#[component]
fn Exercise() -> impl IntoView {
    // 创建 RwSignal<String> 信号存储文本域内容，初始为空字符串
    // 提示: RwSignal::new(String::new())
    let content = todo!();

    view! {
        <div>
            <h2>"练习 255: 受控文本域"</h2>

            {/*
                添加受控 textarea:
                1. prop:value={content} 绑定信号到 textarea 的 value 属性
                2. on:input 事件中使用 event_target_value(&ev) 获取内容
                3. 通过 content.set() 更新信号
            */}
            <textarea prop:value={todo!()} on:input=move |ev| todo!() />

            {/*
                显示字符计数
                提示: move || content.read().len()
            */}
            <p>"字符数: " {todo!()}</p>
        </div>
    }
}
