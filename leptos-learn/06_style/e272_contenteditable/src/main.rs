use leptos::prelude::*;

// 练习目标：创建可编辑 div，实时显示 HTML 内容
//
// 知识点：
// - contenteditable="true" 让 div 可编辑
// - inner_html={...} 属性设置 div 的 HTML 内容
// - event_target::<web_sys::HtmlElement>(&ev) 获取事件目标元素
// - target.inner_html() 读取元素的 innerHTML
//
// 要求：
// 1. 创建一个 contenteditable div，初始内容为"点击此处开始编辑..."
// 2. 监听 on:input 事件，读取 div 的 innerHTML
// 3. 用一个 <pre> 标签实时显示 HTML 内容

#[component]
fn Exercise() -> impl IntoView {
    // TODO: 创建信号用于存储 HTML 内容
    // let (html_content, set_html_content) = signal(/* TODO */);

    // TODO: 实现 input 事件处理函数
    // 提示：用 event_target::<web_sys::HtmlElement>(&ev) 获取元素
    //       然后调用 target.inner_html() 读取 HTML 内容

    view! {
        <div style="padding: 20px;">
            <h2>"可编辑内容"</h2>
            <p>"下面是一个可编辑的 div，修改内容后下方会实时显示 HTML："</p>

            <div
                /* TODO: 设置 contenteditable="true" */
                /* TODO: 设置 style 属性（边框、内边距等） */
                /* TODO: 添加 on:input 事件 */
                /* TODO: 设置 inner_html 属性绑定信号 */
            >

            <h3>"当前的 HTML 内容："</h3>
            <pre style="background: #f5f5f5; padding: 10px; border-radius: 4px;">
                {/* TODO: 显示 html_content */}
            </pre>
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
