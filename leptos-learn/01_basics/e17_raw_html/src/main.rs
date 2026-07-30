// ============================================================
// 练习 e17: 原始 HTML 渲染
//
// 核心知识点:
//   - inner_html 属性（设置元素 innerHTML）
//   - XSS 跨站脚本攻击防范
//
// 难度: ⭐⭐ (关键位置有 TODO — 补全约 50%)
// ============================================================

use leptos::prelude::*;

#[component]
fn Exercise() -> impl IntoView {
    // 定义安全的 HTML 字符串（完全可控，非用户输入）
    let html_content = "<h2>原始 HTML 内容</h2><p style='color: green;'>这段 HTML 是通过 inner_html 渲染的。</p>";

    // ⚠️ XSS 风险警告:
    //   inner_html 会跳过 Leptos 的自动转义机制，直接将 HTML 字符串注入 DOM
    //   - 绝不将用户输入（如表单、URL 参数）直接传给 inner_html
    //   - 仅用于渲染你完全可控的、经过消毒的 HTML
    //   - 优先使用 Leptos 的 view! 宏和组件来构建 UI

    view! {
        // TODO: 使用 inner_html 属性渲染 html_content
        // 提示: <div inner_html=变量名></div>
        <div inner_html=html_content></div>

        // 安全对比：下方代码中的 <b> 标签会被 Leptos 自动转义为纯文本显示
        // TODO: 取消注释查看转义效果
        // <p>"这是 <b>文本插值</b> — 标签会被自动转义"</p>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(Exercise);
}

// <details>
// 参考答案（去除注释后的纯净版本）:
//
// use leptos::prelude::*;
//
// #[component]
// fn Exercise() -> impl IntoView {
//     let html_content = "<h2>原始 HTML 内容</h2><p style='color: green;'>这段 HTML 是通过 inner_html 渲染的。</p>";
//
//     view! {
//         <div inner_html=html_content></div>
//         <p>"这是 <b>文本插值</b> — 标签会被自动转义"</p>
//     }
// }
//
// fn main() {
//     console_error_panic_hook::set_once();
//     mount_to_body(Exercise);
// }
// </details>
