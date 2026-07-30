use leptos::prelude::*;

#[component]
fn Exercise() -> impl IntoView {
    let print_page = move |_| {
        let _ = window().print();
    };

    view! {
        <style>{"
            .no-print { display: block; }
            .print-only { display: none; }
            .content { padding: 2rem; font-size: 16px; line-height: 1.6; }
            @media print {
                .no-print { display: none !important; }
                .print-only { display: block !important; }
                body { font-size: 12pt; color: #000; }
                .content { padding: 0; }
            }
        "}</style>
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
