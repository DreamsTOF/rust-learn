use leptos::prelude::*;

// ============================================================
// 练习 318: 水合基础 (Hydration Basic) — 参考答案
//
// 核心: hydrate 复用服务端 HTML 附加事件，mount_to_body 从头渲染
// ============================================================

/// 有交互的计数器，展示水合后的客户端行为
#[component]
fn ButtonCounter() -> impl IntoView {
    let (count, set_count) = signal(0);

    view! {
        <div>
            <p>"计数: " {move || count.get()}</p>
            <button on:click=move |_| set_count.update(|c| *c += 1)>"+1"</button>
        </div>
    }
}

#[component]
fn Exercise() -> impl IntoView {
    view! {
        <div>
            <h1>"练习 318: 水合基础"</h1>
            <p>"下面是一个可交互的计数器（水合后正常工作）："</p>
            <ButtonCounter/>
            // ponytail: SSR 时服务器渲染静态 HTML
            //          hydrate() 复用它并附加事件监听
            //          mount_to_body() 则丢弃 SSR HTML 从头渲染
            //          实际 SSR 项目用 hydrate()，纯 CSR 用 mount_to_body()
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}

// ponytail: 在 SSR 场景中 main() 通常改为:
//   fn main() {
//       _ = console_error_panic_hook::set_once();
//       hydrate(Exercise);
//   }
// hydrate() 会查找服务端生成的相同 HTML 结构，
// 不重新创建 DOM 节点，只附加事件处理器。
// 而 mount_to_body() 会完全替换 body 内容。
