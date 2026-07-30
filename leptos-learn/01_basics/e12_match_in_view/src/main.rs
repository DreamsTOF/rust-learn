use leptos::prelude::*;
 
// ============================================================
// 练习 e12 — match 在 view 中
// 目标: 在 view! 中使用 match 表达式进行模式匹配
// 难度: ⭐⭐⭐
// 核心知识点: { match x { 1 => "一", _ => "其他" } }
// ============================================================
 
fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| view! { <Exercise/> });
}
 
/// TODO: 创建一个数字信号，用 match 匹配不同值显示对应文字
#[component]
fn Exercise() -> impl IntoView {
    // ========== 学生需要补全以下代码 ==========
    // TODO 1: 使用 signal() 创建数字信号，初始值为 1
    // let (num, set_num) = signal(1);
    //
    // TODO 2: 在 view! 中使用 match 表达式匹配 num 的值
    //         { match num() {
    //             1 => "一",
    //             2 => "二",
    //             3 => "三",
    //             _ => "其他",
    //         } }
    //
    // TODO 3: 添加三个按钮分别设置 num 为 1、2、3
    // ==========================================

    // 临时占位，完成任务后请删除以下内容
    let _placeholder = "请完成练习内容";
    view! {
        <p>{ _placeholder }</p>
    }
}
