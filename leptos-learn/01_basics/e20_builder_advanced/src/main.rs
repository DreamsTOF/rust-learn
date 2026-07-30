// ============================================================
// 练习 e20: 构建器模式高级
//
// 核心知识点:
//   - 纯构建器 API: div().child(...).on(...).build()
//   - 事件监听器: .on(ev::click, move |_| ...)
//   - 样式与方法链
//
// 难度: ⭐⭐⭐ (仅描述目标 — 几乎全部自己写)
// ============================================================
//
// 目标: 完全使用构建器 API 创建一个带事件监听和样式的交互组件
//   1. 使用 div() 作为根容器
//   2. 使用 .child() 添加子元素
//   3. 使用 .on(ev::click, ...) 添加点击事件
//   4. 使用 .attr() 或 style 设置样式
//   5. 最后完成构建器链（无需 .build()）
// 要求: 不要使用 view! 宏
//
// 注意: 构建器链直接实现 IntoView，无需调用 .build()，也无需末尾加分号

use leptos::html::{button, div, h2, p};
use leptos::{ev, prelude::*};

#[component]
fn Exercise() -> impl IntoView {
    // 创建计数器信号
    let (count, set_count) = signal(0);

    // TODO: 使用纯构建器 API 构建 UI:
    //   - div() 作为根容器
    //   - h2() 标题 "构建器模式高级"
    //   - p() 显示当前计数
    //   - "增加" 按钮 (点击时 count+1)
    //   - "重置" 按钮 (点击时 count=0，带 margin-left 样式)
    //
    // 提示: div().child(h2().child("...")).child(p().child(...)).child(button()...)

    // 占位，完成后删除此 view! 并替换为纯构建器 API
    view! {
        <p>"请完成构建器模式练习"</p>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(Exercise);
}
