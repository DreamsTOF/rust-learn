// ============================================================
// 练习 e47: 派生中调用函数 — move || format!("{}", count.get())
//
// 核心知识点:
//   - 派生信号中调用 format! 宏格式化输出
//   - 派生闭包可包含任意 Rust 表达式
//   - 响应式字符串格式化: 信号变化时自动重新格式化
//
// 难度: ⭐⭐ (补全约 50%，关键位置有 TODO)
// ============================================================

use leptos::prelude::*;

fn main() {
    mount_to_body(|| view! { <Exercise/> });
}

/// TODO: 使用 format! 在派生信号中格式化数值
#[component]
fn Exercise() -> impl IntoView {
    // TODO: 创建数值信号 count
   let (count, set_count) = signal(42);

   // TODO: 创建派生信号，使用 format! 格式化 count 的值
   let formatted = move || format!("当前计数: {} ({}的二进制: {:b})", count.get(), count.get(), count.get());

    view! {
        <div>
            <p>"练习 47 (derived_format)"</p>
            // TODO: 在 view! 中显示派生的格式化字符串
            <p>{formatted}</p>
            // TODO: 添加按钮修改 count 的值
           <button on:click=move |_| set_count.set(count.get() + 1)>"+1"</button>
           <button on:click=move |_| set_count.set(count.get() * 2)>"×2"</button>
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
    let (count, set_count) = signal(42);
    let formatted = move || format!("当前计数: {} ({}的二进制: {:b})", count.get(), count.get(), count.get());

    view! {
        <div>
            <p>"练习 47 (derived_format)"</p>
            <p>{formatted}</p>
            <button on:click=move |_| set_count.set(count.get() + 1)>"+1"</button>
            <button on:click=move |_| set_count.set(count.get() * 2)>"×2"</button>
        </div>
    }
}
```

### 知识点
- 派生信号 `move || format!(..., count.get())` 在闭包中调用函数格式化信号值
- 每当 `count` 变化时，`formatted` 闭包重新生成格式化的字符串
- `format!` 宏在派生信号中的用法与普通 Rust 代码完全相同
- 可将任意函数/方法调用放在派生信号中，实现灵活的响应式转换
- 在 view! 中使用 `{formatted}` 会自动跟踪其依赖并响应式更新

</details>
*/
