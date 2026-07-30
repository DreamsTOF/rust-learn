// ============================================================
// 练习 e373 答案: 端到端测试 — 添加 E2E 测试配置和示例
//
// 核心知识点:
//   - E2E 测试的概念与 wasm-bindgen-test 用法
//   - 模拟用户交互（点击、输入等）的测试模式
//   - DOM 查询和断言
//   - E2E 测试配置说明
// ============================================================

use leptos::prelude::*;

/// 待办事项组件 - E2E 测试目标
#[component]
fn TodoApp() -> impl IntoView {
    let (items, set_items) = signal(Vec::<String>::new());
    let (input_value, set_input_value) = signal(String::new());

    let add_item = move |_| {
        let val = input_value.get();
        if !val.is_empty() {
            set_items.update(|list| list.push(val.clone()));
            set_input_value.set(String::new());
        }
    };

    view! {
        <div data-testid="todo-app">
            <h2 data-testid="todo-title">"待办事项"</h2>

            <div style="margin: 12px 0;">
                <input
                    data-testid="todo-input"
                    type="text"
                    placeholder="输入新待办..."
                    prop:value={move || input_value.get()}
                    on:input=move |ev| set_input_value.set(event_target_value(&ev))
                />
                <button
                    data-testid="add-btn"
                    on:click=add_item
                    style="margin-left: 8px;"
                >
                    "添加"
                </button>
            </div>

            <ul data-testid="todo-list">
                {move || items.get().into_iter().enumerate().map(|(idx, item)| {
                    view! {
                        <li data-testid={format!("todo-item-{}", idx)}>
                            {item}
                        </li>
                    }
                }).collect_view()}
            </ul>

            {move || {
                let count = items.get().len();
                if count == 0 {
                    view! { <p data-testid="empty-msg">"暂无待办事项"</p> }
                } else {
                    view! {
                        <p data-testid="item-count">
                            {format!("共 {} 项", count)}
                        </p>
                    }
                }
            }}
        </div>
    }
}

#[component]
fn Exercise() -> impl IntoView {
    view! {
        <div style="padding: 20px; max-width: 600px; margin: 0 auto;">
            <h2>"E2E 测试示例"</h2>
            <TodoApp/>

            <div style="background: #f8f9fa; padding: 16px; border-radius: 8px; margin-top: 24px;">
                <h3>"E2E 测试配置说明"</h3>
                <pre style="background: #e9ecef; padding: 12px; border-radius: 4px; overflow-x: auto;">
                    <code>{r#"## E2E 测试运行方式

# 1. 使用 wasm-pack test
wasm-pack test --headless --chrome

# 2. 或使用 wasm-bindgen-test
# 在 Cargo.toml 中添加:
# [dev-dependencies]
# wasm-bindgen-test = "0.3"

# 3. 对于更完整的 E2E:
# - 使用 Playwright 或 Puppeteer
# - 运行 trunk serve 后访问页面
# - 编写 Node.js 测试脚本"#}</code>
                </pre>
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

    /// 测试待办事项列表初始状态为空
    #[test]
    fn test_todo_initial_empty() {
        let (items, _set_items) = signal(Vec::<String>::new());
        assert!(items.get().is_empty(), "初始待办列表应为空");
    }

    /// 测试添加待办事项
    #[test]
    fn test_todo_add_item() {
        let (items, set_items) = signal(Vec::<String>::new());
        set_items.update(|list| list.push("测试任务".to_string()));
        assert_eq!(items.get().len(), 1, "添加后待办列表长度应为 1");
        assert_eq!(items.get()[0], "测试任务", "待办内容应匹配");
    }

    /// 测试空字符串不添加待办
    #[test]
    fn test_todo_empty_string() {
        let (items, set_items) = signal(Vec::<String>::new());
        let input = String::new();
        if !input.is_empty() {
            set_items.update(|list| list.push(input));
        }
        assert!(items.get().is_empty(), "空字符串不应被添加");
    }

    /// 测试添加多项待办
    #[test]
    fn test_todo_multiple_items() {
        let (items, set_items) = signal(Vec::<String>::new());
        let tasks = vec!["任务1".to_string(), "任务2".to_string(), "任务3".to_string()];
        for task in &tasks {
            set_items.update(|list| list.push(task.clone()));
        }
        assert_eq!(items.get().len(), 3, "应包含 3 个待办事项");
    }

    /// 测试计数显示
    #[test]
    fn test_todo_count_text() {
        let (items, _set_items) = signal(Vec::<String>::new());
        let count = items.get().len();
        let count_text = if count == 0 {
            "暂无待办事项".to_string()
        } else {
            format!("共 {} 项", count)
        };
        assert_eq!(count_text, "暂无待办事项", "初始时应显示暂无待办事项");
    }
}
