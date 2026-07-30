use leptos::prelude::*;

// 练习目标：监听剪贴板事件（copy/paste/cut），显示事件内容
//
// 提示：
// 1. 使用 signal() 创建 (copied_text, set_copied_text) 等信号
// 2. 在 on:copy、on:paste、on:cut 事件中更新信号
// 3. 使用 event_target_value(&ev) 获取输入框的值
// 4. paste 事件中可以使用 ev.unchecked_ref::<web_sys::ClipboardEvent>()
//    来访问 clipboard_data() 获取粘贴内容

#[component]
fn Exercise() -> impl IntoView {
    // TODO: 创建信号用于存储复制/粘贴/剪切的内容
    let (copied_text, set_copied_text) = signal(String::new());
    // let (pasted_text, set_pasted_text) = signal(/* TODO */);
    // let (cut_text, set_cut_text) = signal(/* TODO */);

    // TODO: 实现 copy 事件处理函数
    // let handle_copy = move |ev: leptos::ev::Event| {
    //     let value = event_target_value(&ev);
    //     set_copied_text.set(value);
    // };

    // TODO: 实现 paste 事件处理函数
    // 提示：使用 ev.unchecked_ref::<web_sys::ClipboardEvent>()
    //       然后通过 clipboard_data() 获取 DataTransfer
    //       再调用 get_data("text/plain") 获取文本

    // TODO: 实现 cut 事件处理函数

    view! {
        <div style="padding: 20px;">
            <h2>"剪贴板事件"</h2>

            <div style="margin-bottom: 12px;">
                <label>"复制演示："</label>
                <input
                    type="text"
                    placeholder="选中文本后按 Ctrl+C"
                    /* TODO: 添加 on:copy 事件 */
                />
                <p>"复制的文本：" {move || copied_text.get()}</p>
            </div>

            <div style="margin-bottom: 12px;">
                <label>"粘贴演示："</label>
                <input
                    type="text"
                    placeholder="在此按 Ctrl+V 粘贴"
                    /* TODO: 添加 on:paste 事件 */
                />
                <p>"粘贴的文本：" {/* TODO */}</p>
            </div>

            <div style="margin-bottom: 12px;">
                <label>"剪切演示："</label>
                <input
                    type="text"
                    placeholder="选中文本后按 Ctrl+X"
                    /* TODO: 添加 on:cut 事件 */
                />
                <p>"剪切的文本：" {/* TODO */}</p>
            </div>
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
