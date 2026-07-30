use leptos::prelude::*;

#[component]
fn Exercise() -> impl IntoView {
    // TODO: 实现打印函数
    // 使用 web_sys::window() 获取 Window 对象，调用 .print() 方法
    let print_page = move |_| {
        // 补全打印逻辑
    };

    view! {
        // TODO: 添加 <Style> 组件，定义打印样式
        // - .no-print: 在 @media print 中隐藏 (display: none)
        // - .print-only: 在普通视图中隐藏，仅在打印时显示
        // - 打印时：body 使用 12pt 字号，黑色文字

        <div class="no-print">
            <h2>"打印样式示例"</h2>
            <button on:click=print_page>"打印此页"</button>
            <hr/>
        </div>
        <div class="content">
            <h1>"文章标题"</h1>
            <p>"这是文章正文内容。打印预览时会隐藏按钮等交互元素。"</p>
            <p>"打印版本会使用更简洁的样式。"</p>
        </div>
        <div class="print-only">
            <p>"此内容仅在打印时可见。"</p>
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
