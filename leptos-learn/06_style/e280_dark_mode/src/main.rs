// ============================================================
// 练习 e280: 暗黑模式 — CSS 变量 + 信号控制主题切换
//
// 核心知识点:
//   - class:xxx 动态切换 CSS 类名
//   - CSS 变量（自定义属性）实现主题色定义
//   - @media (prefers-color-scheme: dark) 媒体查询
//   - <Style/> 组件定义全局样式
//
// 难度: ⭐⭐ (补全关键代码)
// ============================================================

use leptos::prelude::*;

#[component]
fn Exercise() -> impl IntoView {
    let (dark, set_dark) = signal(false);

    view! {
        // TODO: 为最外层 div 添加 class:dark={move || dark.get()} 动态类名绑定
        <div>
            // TODO: 使用 <Style/> 组件定义暗黑模式相关样式:
            //   :root { --bg: #ffffff; --text: #333333; --primary: #0066cc; }
            //   .dark { --bg: #1a1a2e; --text: #e0e0e0; --primary: #66b3ff; }
            //   @media (prefers-color-scheme: dark) { :root { ... } }
            //   body { background-color: var(--bg); color: var(--text); ... }
            <Style></Style>

            <div>
                <h1>"暗黑模式"</h1>
                // TODO: 添加按钮，点击时切换 dark 信号的值
                // 按钮文字应反映当前模式
                <button>
                    {move || if dark.get() { "切换到亮色模式" } else { "切换到暗黑模式" }}
                </button>
            </div>
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
