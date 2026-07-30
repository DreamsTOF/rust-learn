// 练习 253 — 受控输入框
//
// 目标: 实现受控输入框，使用 signal + on:input + prop:value 完成双向绑定。
//
// 受控模式 = 输入框的值始终由信号驱动:
// - prop:value={value} 将信号值推送到输入框显示
// - on:input 事件将用户输入写回信号
// - 两者配合实现"双向绑定"

use leptos::prelude::*;

fn main() {
    mount_to_body(|| view! { <Exercise/> });
}

#[component]
fn Exercise() -> impl IntoView {
    // 创建响应式信号来存储输入文本
    let value = RwSignal::new(String::new());

    view! {
        <div>
            <h2>"练习 253: 受控输入框"</h2>

            {/*
                补全受控输入框的绑定:
                - prop:value={value} — 将信号绑定到 DOM value 属性
                - on:input=move |ev| value.set(event_target_value(&ev)) — 更新信号
            */}
            <input type="text"
                prop:value={todo!()}
                on:input=move |ev| todo!()
            />

            {/*
                补全重置按钮，点击后清空 value
                提示: value.set(String::new())
            */}
            <button on:click=todo!()>"重置"</button>

            <p>
                "值: " {value}
                "，长度: "
                {/* 补全字符长度显示: value.read().len() */}
                {todo!()}
            </p>
        </div>
    }
}
