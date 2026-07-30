use leptos::prelude::*;


// ============================================================
// 练习 319: 水合陷阱 (Hydration Pitfalls)
//
// 目标: 体验 CSR 与 SSR HTML 不一致导致的水合错误，学习修复策略
//
// 核心知识点:
//   - 水合要求 CSR 与 SSR 渲染一致
//   - 浏览器特有 API 在 SSR 不可用 → 使用 ClientOnly / is_server
//   - 随机/时间相关数据需要在客户端延迟生成
//
// ⭐⭐⭐: 创建会造成水合错误的组件，然后修复它
// ============================================================

// TODO: 创建一个会导致水合错误的组件 WrongRandom
//   - 使用 rand crate 生成随机数（或 web_sys::window() 获取浏览器宽高）
//   - 这会在 SSR 和 CSR 间产生不同输出
//   - CSR 中，在 Effect::new 内调用随机 API

// TODO: 创建修复后的组件 CorrectRandom
//   - 使用信号存储客户端值
//   - 在 Effect::new 中设置客户端值的信号
//   - 渲染客户端值（初始为 placeholder）

#[component]
fn Exercise() -> impl IntoView {
    view! {
        <div>
            <h1>"练习 319: 水合陷阱"</h1>
            // TODO: 取消下行注释来体验水合错误：
            // <WrongRandom/>

            <hr/>
            // TODO: 添加修复后的组件
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
