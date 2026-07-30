 use leptos::prelude::*;
 
 // ============================================================
 // 练习 e11 — 条件 if 在 view 中
 // 目标: 在 view! 中使用 if-else 条件渲染
 // 难度: ⭐⭐⭐
 // 核心知识点: { if cond { "A" } else { "B" } }
 // ============================================================
 
 fn main() {
     console_error_panic_hook::set_once();
     mount_to_body(|| view! { <Exercise/> });
 }
 
/// TODO: 创建一个布尔信号和一个按钮，点击按钮切换信号值
///       使用 if 表达式在 <p> 中显示不同内容
 #[component]
 fn Exercise() -> impl IntoView {
    // ========== 学生需要补全以下代码 ==========
    // TODO 1: 使用 signal() 创建布尔信号，初始值为 false
    // let (show, set_show) = signal(false);
    //
    // TODO 2: 在 view! 中使用 if 表达式显示 "已激活" 或 "未激活"
    // { if show() { "已激活" } else { "未激活" } }
    //
    // TODO 3: 添加按钮，点击时切换信号值
    // <button on:click=move |_| set_show(!show())>"切换状态"</button>
    // ==========================================

    // 临时占位，完成任务后请删除以下内容
    let _placeholder = "请完成练习内容";
    view! {
        <p>{ _placeholder }</p>
    }
}
