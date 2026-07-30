// ============================================================
// 参考答案 e377: A/B 测试框架
//
// 实现基于用户 ID 哈希的分桶、localStorage 持久化和不同实验组 UI
// ============================================================

use leptos::prelude::*;
use wasm_bindgen::prelude::wasm_bindgen;

/// 根据用户 ID 进行哈希分桶，返回 "A" 或 "B"
fn assign_group(user_id: &str) -> &'static str {
    let hash = user_id
        .bytes()
        .fold(0u32, |acc, b| acc.wrapping_mul(31).wrapping_add(b as u32));
    if hash % 2 == 0 { "A" } else { "B" }
}

/// 使用 wasm_bindgen inline_js 调用浏览器 localStorage API
#[wasm_bindgen(inline_js = r#"
export function storage_get_item(key) {
    return localStorage.getItem(key) || null;
}
export function storage_set_item(key, value) {
    localStorage.setItem(key, value);
}
export function console_log(msg) {
    console.log(msg);
}
"#)]
unsafe extern "C" {
    fn storage_get_item(key: &str) -> Option<String>;
    fn storage_set_item(key: &str, value: &str);
    fn console_log(msg: &str);
}

/// 从 localStorage 读取用户组别，若不存在则分配并存储
fn get_or_create_group() -> String {
    if let Some(group) = storage_get_item("ab_group") {
        return group;
    }
    // 首次访问：生成组别
    let user_id = "user_default_id";
    let group = assign_group(user_id);
    storage_set_item("ab_group", group);
    group.to_string()
}

/// 模拟上报用量数据到分析平台
fn track_event(event: &str, group: &str) {
    let msg = format!("[AB Test] {} | group={}", event, group);
    console_log(&msg);
}

#[component]
fn Exercise() -> impl IntoView {
    let group = get_or_create_group();

    // 上报展示事件
    track_event("view_experiment", group.as_str());

    // 使用 RwSignal 包装，以便可从任何闭包中访问
    let group_signal = RwSignal::new(group);

    let is_group_a = move || group_signal.get() == "A";

    view! {
        <div style="padding: 20px; max-width: 600px; margin: 0 auto; font-family: system-ui, sans-serif;">
            <h2>"🧪 A/B 测试实验"</h2>
            <p>
                "您当前属于 "
                <strong>{move || group_signal.get()}</strong>
                " 组"
            </p>

            <Show when=move || is_group_a()>
                <div style="border: 2px solid #2196f3; padding: 16px; border-radius: 8px; margin: 16px 0;">
                    <h3 style="color: #1565c0;">"📋 简洁版 (A 组)"</h3>
                    <p>"这是简洁版 UI，核心功能一目了然。"</p>
                    <button
                        style="background: #2196f3; color: white; border: none; padding: 10px 24px;
                               border-radius: 4px; cursor: pointer; font-size: 14px;"
                        on:click=move |_| track_event("click_cta_a", &group_signal.get())
                    >
                        "立即体验 (A)"
                    </button>
                </div>
            </Show>

            <Show when=move || !is_group_a()>
                <div style="border: 2px solid #4caf50; padding: 16px; border-radius: 8px; margin: 16px 0;">
                    <h3 style="color: #2e7d32;">"🚀 增强版 (B 组)"</h3>
                    <p>"这是增强版 UI，包含更多高级功能选项。"</p>
                    <button
                        style="background: #4caf50; color: white; border: none; padding: 10px 24px;
                               border-radius: 4px; cursor: pointer; font-size: 14px;"
                        on:click=move |_| track_event("click_cta_b", &group_signal.get())
                    >
                        "立即体验 (B)"
                    </button>
                </div>
            </Show>

            <div style="border: 1px solid #ddd; padding: 16px; border-radius: 8px; margin-top: 16px;">
                <h3>"实验说明"</h3>
                <ul style="line-height: 1.8;">
                    <li>"组别基于用户 ID 哈希确定，刷新页面不会改变。"</li>
                    <li>"组别已保存到浏览器 localStorage 中。"</li>
                    <li>"所有交互事件会输出到浏览器控制台。"</li>
                    <li>"实际项目中可替换为真实分析 SDK。"</li>
                </ul>
            </div>
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
