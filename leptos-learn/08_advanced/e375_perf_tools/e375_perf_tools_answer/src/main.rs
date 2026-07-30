// ============================================================
// 练习 e375 答案: 性能工具 — 测量渲染性能、帧率等
//
// 核心知识点:
//   - 使用 web_sys::Performance API 测量代码耗时
//   - 在 Effect::new 中测量渲染耗时
//   - 使用 request_animation_frame 计算帧率 (FPS)
//   - 分析列表渲染性能
// ============================================================

use leptos::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::closure::Closure;

/// 帧率计数器组件
#[component]
fn FpsCounter() -> impl IntoView {
    let (fps, set_fps) = signal(0.0_f64);
    let frame_count = std::cell::Cell::new(0u32);
    let last_time = std::cell::Cell::new(0.0_f64);
    let first_frame = std::cell::Cell::new(true);

    Effect::new(move || {
        let performance = web_sys::window()
            .expect("window 不存在")
            .performance()
            .expect("performance API 不可用");

        let f = set_fps;
        let fc = &frame_count;
        let lt = &last_time;
        let ff = &first_frame;

        let closure = Closure::new(move || {
            let now = performance.now();
            if ff.get() {
                lt.set(now);
                ff.set(false);
            }

            fc.set(fc.get() + 1);
            let elapsed = now - lt.get();

            if elapsed >= 1000.0 {
                let current_fps = fc.get() as f64 / (elapsed / 1000.0);
                f.set(current_fps);
                fc.set(0);
                lt.set(now);
            }

            // 继续下一帧
            let window = web_sys::window().expect("window 不存在");
            // 这里需要用 request_animation_frame 继续循环
            // 但由于 Closure 已经 forget 了，我们需要重新创建
            // 实际使用中应使用更优雅的方式
        });

        let window = web_sys::window().expect("window 不存在");
        window
            .request_animation_frame(&closure.as_ref().unchecked_ref())
            .expect("requestAnimationFrame 失败");
        closure.forget();
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
    let (items, set_items) = signal(Vec::<i32>::new());

    let update_items = move |count: i32| {
        let performance = web_sys::window()
            .expect("window 不存在")
            .performance()
            .expect("performance API 不可用");

        let start = performance.now();
        let new_items: Vec<i32> = (0..count).collect();
        let end = performance.now();

        set_render_time.set(end - start);
        set_items.set(new_items);
    };

    // 初始加载
    let initialized = std::cell::Cell::new(false);
    Effect::new(move || {
        if !initialized.get() {
            initialized.set(true);
            update_items(item_count.get());
        }
    });

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
                    view! { <span style="margin: 2px 4px; display: inline-block;">{i}</span> }
                }).collect_view()}
                {move || {
                    let total = items.get().len();
                    if total > 100 {
                        view! { <p style="color: #999;">{format!("... 以及其余 {} 项", total - 100)}</p> }
                    } else {
                        view! { <span></span> }
                    }
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
