use leptos::prelude::*;

#[component]
fn Exercise() -> impl IntoView {
    // TODO: 创建 count 信号，初始值为 0
    // 使用 signal(0) 创建 (ReadSignal<i32>, WriteSignal<i32>)

    view! {
        <div>
            // TODO: 添加 <h2> 标题 "HMR 热重载演示"
            // TODO: 添加 <p> 说明文字，提示用户修改组件文本观察热更新

            // TODO: 显示当前计数: <p>"当前计数: " {count}</p>

            // TODO: 添加两个按钮
            // - "+1" 按钮: on:click 调用 set_count 增加计数
            // - "-1" 按钮: on:click 调用 set_count 减少计数
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
