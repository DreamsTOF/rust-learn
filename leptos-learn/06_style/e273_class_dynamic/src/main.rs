use leptos::prelude::*;

// 练习目标：点击按钮切换 active 类，改变元素样式
//
// 知识点：
// - class:active={cond}：当 cond 为 true 时，元素获得 "active" 类
// - set_is_active.update(|v| *v = !*v)：切换布尔值
// - move || is_active.get()：在闭包中读取信号值

#[component]
fn Exercise() -> impl IntoView {
    // 创建布尔信号，初始为 false
    let (is_active, set_is_active) = signal(false);

    // 点击按钮时切换状态
    let toggle_active = move |_| {
        set_is_active.update(|v| *v = !*v);
    };

    view! {
        <div style="padding: 20px;">
            <h2>"动态类名"</h2>
            { /* 使用 class:active 指令，当 is_active 为 true 时添加 active 类 */ }
            <div
                class:active={move || is_active.get()}
                style="padding: 20px; border: 1px solid #ccc; border-radius: 8px; transition: all 0.3s; margin: 10px 0;"
            >
                <p>"这个元素的样式会随 active 类切换而变化"</p>
                <p>"当前状态：" {move || if is_active.get() { "激活" } else { "未激活" }}</p>
            </div>

            <button on:click=toggle_active style="padding: 8px 16px; cursor: pointer;">
                {move || if is_active.get() { "取消激活" } else { "激活" }}
            </button>

            <style>
                ".active {
                    background-color: #4caf50;
                    color: white;
                    font-weight: bold;
                    border-color: #388e3c !important;
                }"
            </style>
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
