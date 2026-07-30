// 练习 288: 性能分析 (Performance Profiling)
//
// 目标: 使用 Web Performance API (web_sys::performance) 测量操作耗时。
//
// 提示:
// - leptos 重导出了 web_sys，通过 leptos::web_sys 访问
// - window.performance.now() 返回高精度时间戳（毫秒）
// - 获取方式: web_sys::window().unwrap().performance().unwrap().now()
//
// 步骤:
// 1. 获取 window.performance 对象
// 2. 创建一个带计数器和计时器的组件
// 3. 每次按钮点击时测量操作耗时并显示

use leptos::prelude::*;

#[component]
fn Exercise() -> impl IntoView {
    // TODO: 获取 performance 对象
    // let performance = web_sys::window()
    //     .expect("window 不存在")
    //     .performance()
    //     .expect("performance API 不可用");

    // TODO: 创建信号存储计数和时间
    let (count, set_count) = signal(0);
    // let (last_time, set_last_time) = signal(0.0_f64);

    view! {
        <div>
            <h2>"性能分析示例"</h2>
            <p>"当前计数: " {count}</p>
            // TODO: 显示上次操作耗时
            // <p>"上次操作耗时: " {move || format!("{:.3} ms", last_time.get())} </p>
            <button on:click=move |_| {
                // TODO: 记录开始时间
                set_count.update(|n| *n += 1);
                // TODO: 记录结束时间并计算耗时
            }>"更新"</button>
            <p>"提示: 打开浏览器 DevTools Performance 面板"</p>
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
