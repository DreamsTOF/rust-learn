// ============================================================
// 练习 e375: 性能工具 — 测量渲染性能、帧率等
//
// 核心知识点:
//   - 使用 web_sys::Performance API 测量代码耗时
//   - 在 Effect::new 中测量渲染耗时
//   - 使用 request_animation_frame 计算帧率 (FPS)
//   - 分析列表渲染性能
//
// 难度: ⭐⭐ (关键位置有 TODO，补全约 50%)
// ============================================================

use leptos::prelude::*;
#[allow(unused_imports)]
use leptos::task::spawn_local;

/// 帧率计数器组件
#[component]
fn FpsCounter() -> impl IntoView {
    let (fps, set_fps) = signal(0.0_f64);
    let _set_fps = set_fps;
    let _frame_count = std::cell::Cell::new(0u32);
    let last_time = std::cell::Cell::new(0.0_f64);

    // TODO: 使用 Effect::new + request_animation_frame 计算 FPS
    // 提示:
    //   1. 在 Effect 中调用 request_animation_frame
    //   2. 每次回调增加帧计数
    //   3. 每秒计算一次 FPS = frame_count / elapsed_seconds
    //   4. 使用 Closure::new + wasm_bindgen::closure 实现 RAF 循环
    let started = std::cell::Cell::new(false);
    Effect::new(move || {
        if started.get() {
            return;
        }
        started.set(true);

        // 获取 performance 对象
        let performance = web_sys::window()
            .expect("window 不存在")
            .performance()
            .expect("performance API 不可用");

        last_time.set(performance.now());

        // TODO: 创建 request_animation_frame 循环
        // 使用 Closure::new + request_animation_frame
        // ...
    });

    view! {
        <div style="background: #f8f9fa; padding: 12px; border-radius: 8px; text-align: center;">
            <p style="font-size: 14px; color: #666;">"帧率 (FPS)"</p>
            <p style="font-size: 32px; font-weight: bold; color: #3498db; margin: 4px 0;">
                {move || format!("{:.1}", fps.get())}
            </p>
        </div>
    }
}

/// 性能测量组件 — 测量列表渲染性能
#[component]
fn RenderTimer() -> impl IntoView {
    let (item_count, set_item_count) = signal(100);
    let (render_time, set_render_time) = signal(0.0_f64);
    let (items, _set_items) = signal(Vec::<i32>::new());

    let update_items = move |count: i32| {
        // TODO: 使用 performance.now() 测量更新时间
        // 提示: 记录开始时间，生成数据，记录结束时间，计算差值
        let _performance = web_sys::window()
            .expect("window 不存在")
            .performance()
            .expect("performance API 不可用");

        // let start = performance.now();
        let new_items: Vec<i32> = (0..count).collect();
        // let end = performance.now();
        // set_render_time.set(end - start);
        _set_items.set(new_items);
    };

    view! {
        <div>
            <div style="margin: 12px 0;">
                <label>"渲染项数: " {item_count}</label>
                <input
                    type="range"
                    min="10"
                    max="10000"
                    step="10"
                    prop:value={move || item_count.get().to_string()}
                    on:input=move |ev| {
                        if let Ok(v) = event_target_value(&ev).parse::<i32>() {
                            set_item_count.set(v);
                            update_items(v);
                        }
                    }
                    style="width: 100%; margin: 8px 0;"
                />
            </div>

            <div style="background: #f8f9fa; padding: 12px; border-radius: 8px; margin: 8px 0;">
                <p>"数据生成耗时: " {move || format!("{:.3} ms", render_time.get())}</p>
                <p>"列表项数: " {item_count}</p>
            </div>

            <div style="max-height: 200px; overflow-y: auto; border: 1px solid #ddd;
                        border-radius: 4px; padding: 8px; margin: 8px 0;">
                {move || items.get().iter().take(100).map(|i| {
                    view! { <span style="margin: 2px 4px; display: inline-block;">{*i}</span> }
                }).collect_view()}
                {move || {
                    let total = items.get().len();
                    (total > 100).then(|| {
                        let extra = total - 100;
                        view! { <p style="color: #999;">"... 以及 " {extra} " 项"</p> }
                    })
                }}
            </div>
        </div>
    }
}

#[component]
fn Exercise() -> impl IntoView {
    view! {
        <div style="padding: 20px; max-width: 800px; margin: 0 auto;">
            <h2>"性能工具"</h2>

            <div style="margin: 16px 0;">
                <h3>"帧率监视器"</h3>
                <FpsCounter/>
            </div>

            <div style="margin: 16px 0; padding: 16px; border: 1px solid #ddd; border-radius: 8px;">
                <h3>"列表渲染性能"</h3>
                <RenderTimer/>
            </div>
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
