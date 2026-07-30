use leptos::prelude::*;

// 练习: 防抖提交
//
// 目标: 输入框内容变化后 500ms 自动保存
//
// 实现提示:
// 1. 创建 text 信号 (signal(String::new())) 和 saved 信号
// 2. 在 Effect::new 中监听 text 变化
// 3. 使用 set_timeout 设置 500ms 延迟保存
// 4. 使用 on_cleanup 取消上一次未执行的 timeout
// 5. API: set_timeout(|| { ... }, std::time::Duration::from_millis(500))
//        on_cleanup(move || { handle.clear(); })

#[component]
fn Exercise() -> impl IntoView {
    // TODO: 创建 text 信号
    // TODO: 创建 saved 信号（保存已保存的内容）

    // TODO: 创建 Effect::new，实现防抖逻辑
    //   - 读取 text 的值
    //   - 如果非空，用 set_timeout 设置 500ms 延迟
    //   - 延迟回调中更新 saved 并 log
    //   - 用 on_cleanup 取消上一次 timeout

    view! {
        <div style="padding: 1rem;">
            <h2>"防抖提交"</h2>
            <p>"输入内容，500ms 后自动保存:"</p>
            // TODO: 添加 textarea，绑定 prop:value 和 on:input
            // TODO: 显示已保存的内容
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
