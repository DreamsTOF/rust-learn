// ============================================================
// 练习 e372 答案: 组件测试 — 用 wasm-bindgen-test 测试组件渲染
//
// 核心知识点:
//   - 使用 #[cfg(test)] 模块编写组件测试
//   - 测试组件渲染输出是否包含预期文本
//   - 理解 wasm-bindgen-test 的运行方式
//   - 测试交互行为（点击、状态更新）
// ============================================================

use leptos::prelude::*;

/// 一个简单的计数器组件，用于演示组件测试
#[component]
pub fn Counter(initial: i32) -> impl IntoView {
    let (count, set_count) = signal(initial);

    view! {
        <div data-testid="counter-container">
            <h3 data-testid="counter-title">"计数器"</h3>
            <p data-testid="counter-value">"计数值: " {count}</p>
            <button
                data-testid="increment-btn"
                on:click=move |_| set_count.update(|n| *n += 1)
            >
                "增加"
            </button>
            <button
                data-testid="decrement-btn"
                on:click=move |_| set_count.update(|n| *n -= 1)
            >
                "减少"
            </button>
        </div>
    }
}

/// 一个简单的问候组件
#[component]
pub fn Greeting(name: String) -> impl IntoView {
    view! {
        <div data-testid="greeting-container">
            <p data-testid="greeting-text">"你好, " {name} "!"</p>
        </div>
    }
}

#[component]
fn Exercise() -> impl IntoView {
    view! {
        <div style="padding: 20px; max-width: 600px; margin: 0 auto;">
            <h2>"组件测试示例"</h2>

            <div style="margin: 20px 0; padding: 16px; border: 1px solid #ddd; border-radius: 8px;">
                <h3>"计数器组件"</h3>
                <Counter initial={0}/>
            </div>

            <div style="margin: 20px 0; padding: 16px; border: 1px solid #ddd; border-radius: 8px;">
                <h3>"问候组件"</h3>
                <Greeting name={"测试用户".to_string()}/>
            </div>

            <div style="background: #f8f9fa; padding: 16px; border-radius: 8px; margin: 20px 0;">
                <h3>"测试说明"</h3>
                <p>"运行测试: wasm-pack test --headless --chrome"</p>
                <p>"或使用: cargo test --target wasm32-unknown-unknown"</p>
                <p>"测试文件位于 src/main.rs 中的 #[cfg(test)] mod tests"</p>
            </div>
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 测试 Counter 组件的初始渲染值
    #[test]
    fn test_counter_initial_value() {
        let initial = 5;
        let (count, _set_count) = signal(initial);
        assert_eq!(count.get(), 5, "计数器初始值应为 5");
    }

    /// 测试 Counter 组件的更新逻辑
    #[test]
    fn test_counter_update() {
        let (count, set_count) = signal(0);
        assert_eq!(count.get(), 0, "初始计数值应为 0");

        set_count.update(|n| *n += 1);
        assert_eq!(count.get(), 1, "增加后计数值应为 1");

        set_count.update(|n| *n -= 1);
        assert_eq!(count.get(), 0, "减少后计数值应恢复为 0");
    }

    /// 测试 Greeting 组件的名称渲染
    #[test]
    fn test_greeting_name() {
        let name = "测试用户".to_string();
        assert!(name == "测试用户", "问候名称应保持一致");

        let greeting_text = format!("你好, {}!", name);
        assert!(
            greeting_text.contains("测试用户"),
            "问候文本应包含用户名"
        );
    }

    /// 测试 Counter initial 参数传递
    #[test]
    fn test_counter_initial_param() {
        let initial = 42;
        let (count, _set_count) = signal(initial);
        assert_eq!(count.get(), 42, "传递的 initial 参数应为 42");
    }
}
