// ============================================================
// 练习 e366: 无限滚动加载 — IntersectionObserver 触发加载更多
//
// 核心知识点:
//   - IntersectionObserver API 检测哨兵元素进入视口
//   - wasm_bindgen::closure::Closure 包装 JS 回调
//   - Effect::new + Cell<bool> 做一次性初始化
//   - 模拟异步分批加载数据
//
// 难度: ⭐⭐⭐ (关键位置有 TODO，补全约 50%)
// ============================================================

use leptos::prelude::*;
use std::cell::Cell;
use std::time::Duration;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;

// TODO: 定义 IntersectionObserver 的 inline_js
// 使用 #[wasm_bindgen(inline_js = r#"... "#)]
// 导出函数 observeIntersection(element, callback) -> observer
// 当哨兵元素进入视口时调用 callback()

#[component]
fn Exercise() -> impl IntoView {
    let (items, set_items) = signal(Vec::<String>::new());
    let (loading, set_loading) = signal(false);
    let (has_more, set_has_more) = signal(true);
    let initialized = Cell::new(false);

    // 加载更多数据（模拟异步 API）
    let load_more = {
        let set_items = set_items.clone();
        let set_loading = set_loading.clone();
        let set_has_more = set_has_more.clone();
        move || {
            if loading() || !has_more() {
                return;
            }
            set_loading(true);
            let current_len = items().len();
            set_timeout(
                move || {
                    let mut new_items: Vec<String> = (current_len + 1..=current_len + 10)
                        .map(|i| format!("项目 #{} — 模拟加载的数据项", i))
                        .collect();
                    set_items.update(|v| v.append(&mut new_items));
                    set_loading(false);
                    if items().len() >= 100 {
                        set_has_more(false);
                    }
                },
                Duration::from_millis(800),
            );
        }
    };

    // 首次加载
    load_more();

    // TODO: 设置 IntersectionObserver — 一次性初始化
    // 1. 在 Effect::new 中获取哨兵元素 #sentinel
    // 2. 创建 Closure 包装 load_more
    // 3. 调用 observeIntersection
    // 4. callback.forget() 防止 GC
    // 5. initialized.set(true)

    // TODO: 在 view! 中遍历 items() 渲染列表
    // 并在底部显示 loading / has_more 状态

    view! {
        <div style="padding: 1rem; font-family: sans-serif; max-width: 480px; margin: 0 auto;">
            <h3>"练习 e366: 无限滚动（IntersectionObserver）"</h3>
            <p>"滚动到底部 — 哨兵元素进入视口时自动加载更多"</p>

            <div style="border: 1px solid #ddd; border-radius: 8px; padding: 4px 12px;"></div>

            <div id="sentinel" style="height: 1px;"></div>

            <div style="text-align: center; padding: 12px; color: #888;"></div>
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
