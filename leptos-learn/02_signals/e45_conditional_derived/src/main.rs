// ============================================================
// 练习 e45: 条件派生（非均匀更新）
//
// 核心知识点:
//   - 条件跟踪: 派生信号可依据条件只跟踪部分信号
//   - 非均匀更新: 条件不同时，依赖的信号集合不同
//   - 响应式剪枝: 未被读取的信号变化不会触发派生重算
//
// 难度: ⭐⭐⭐ (补全约 50%，关键位置有 TODO)
// ============================================================

use leptos::prelude::*;

fn main() {
    mount_to_body(|| view! { <Exercise/> });
}

/// TODO: 创建 count 和 toggle 信号，派生信号根据 toggle 决定是否跟踪 count
#[component]
fn Exercise() -> impl IntoView {
    // TODO: 创建数值信号 count(0) 和布尔开关 toggle(true)
    let (count, set_count) = signal(0);
    let (toggle, set_toggle) = signal(true);

    // TODO: 创建派生信号，仅当 toggle=true 时跟踪 count 的变化
    //       当 toggle=false 时始终返回 "已关闭"，count 的变化不会触发重算
    let derived = move || {
        if toggle.get() {
            format!("当前值: {}", count.get())
        } else {
            "已关闭".to_string()
        }
    };

    view! {
        <div>
            <p>"练习 45 (conditional_derived)"</p>
            <p>"派生值: " {derived}</p>
            // TODO: 添加 "+1" 按钮增加 count
            <button on:click=move |_| set_count.set(count.get() + 1)>"+1"</button>
            // TODO: 添加 "重置" 按钮将 count 归零
            <button on:click=move |_| set_count.set(0)>"重置"</button>
            // TODO: 添加 "切换" 按钮切换 toggle
            <button on:click=move |_| set_toggle.set(!toggle.get())>
                {move || if toggle.get() { "关闭跟踪" } else { "开启跟踪" }}
            </button>
            <p>"提示: 关闭跟踪后修改 count 不会更新派生值"</p>
        </div>
    }
}

/*
<details>
<summary>参考答案</summary>

```rust
use leptos::prelude::*;

fn main() {
    mount_to_body(|| view! { <Exercise/> });
}

#[component]
fn Exercise() -> impl IntoView {
    let (count, set_count) = signal(0);
    let (toggle, set_toggle) = signal(true);

    let derived = move || {
        if toggle() {
            format!("当前值: {}", count())
        } else {
            "已关闭".to_string()
        }
    };

    view! {
        <div>
            <p>"练习 45 (conditional_derived)"</p>
            <p>"派生值: " {derived}</p>
            <button on:click=move |_| set_count(count() + 1)>"+1"</button>
            <button on:click=move |_| set_count(0)>"重置"</button>
            <button on:click=move |_| set_toggle(!toggle())>
                {move || if toggle() { "关闭跟踪" } else { "开启跟踪" }}
            </button>
        </div>
    }
}
```

### 知识点
- **非均匀更新**: `derived` 闭包的条件分支读取不同的信号
  - `toggle()` 为 true 时: 跟踪 `toggle` 和 `count`
  - `toggle()` 为 false 时: 只跟踪 `toggle`（不跟踪 `count`）
- `toggle` 变化时无论 true/false 都会触发派生重算
- `count` 变化时**仅当** `toggle` 为 true 才会触发派生重算
- 这是 Leptos 细粒度响应式的重要特性: 运行时不维护静态依赖列表，而是每次执行时动态收集

</details>
*/
