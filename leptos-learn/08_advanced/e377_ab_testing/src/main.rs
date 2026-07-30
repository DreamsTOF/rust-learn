// ============================================================
// 练习 e377: A/B 测试框架 — 用户分桶 + 不同实验组 UI
//
// 核心知识点:
//   - 基于用户 ID 哈希的分桶算法
//   - 组别持久化到 localStorage
//   - 不同实验组渲染不同 UI
//   - 模拟用量数据上报
//
// 难度: ⭐⭐ (需补全分桶逻辑、持久化和实验组 UI，约 50%)
// ============================================================

use leptos::prelude::*;

/// 根据用户 ID 进行哈希分桶，返回 "A" 或 "B"
fn assign_group(user_id: &str) -> &'static str {
    // TODO 1: 使用字符串字节的哈希值判断奇偶，偶数返回 "A"，奇数返回 "B"
    // 提示: user_id.bytes().fold(0u32, |acc, b| acc.wrapping_mul(31).wrapping_add(b as u32))
    "A" // placeholder
}

/// 从 localStorage 读取用户组别，若不存在则分配并存储
fn get_or_create_group() -> String {
    // TODO 2: 从 localStorage 获取 "ab_group" 键的值
    // 提示: 使用 window().local_storage() 获取 Storage 对象
    // 如果不存在则调用 assign_group 生成并用 set_item 存储
    "A".to_string() // placeholder
}

/// 模拟上报用量数据到分析平台
fn track_event(event: &str, group: &str) {
    // TODO 3: 使用 console.log 输出事件和组别信息
    // 提示: web_sys::console::log_1(&format!("[AB Test] {} | group={}", event, group).into())
}

#[component]
fn Exercise() -> impl IntoView {
    let group = get_or_create_group();

    // 上报展示事件
    track_event("view_experiment", &group);

    let handle_click = move |action: &'static str| {
        // TODO 4: 在点击按钮时上报交互事件
        // 提示: 调用 track_event 并传入 action 和 group
    };

    view! {
        <div style="padding: 20px; max-width: 600px; margin: 0 auto; font-family: system-ui, sans-serif;">
            <h2>"🧪 A/B 测试实验"</h2>
            <p>
                "您当前属于 "
                <strong>{group.clone()}</strong>
                " 组"
            </p>

            <div style="border: 1px solid #ddd; padding: 16px; border-radius: 8px; margin: 16px 0;">
                // TODO 5: 根据组别渲染不同的实验 UI
                // A 组: 蓝色按钮 + "简洁版" 标题
                // B 组: 绿色按钮 + "增强版" 标题
            </div>

            <div style="border: 1px solid #ddd; padding: 16px; border-radius: 8px;">
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
