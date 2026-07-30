use leptos::prelude::*;

// ============================================================
// 练习 e12 — match 在 view 中
// 目标: 在 view! 中使用 match 表达式进行模式匹配
// 难度: ⭐⭐⭐
// 核心知识点: { match x { 1 => "一", _ => "其他" } }
// ============================================================

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| view! { <Exercise/> });
}

/// TODO: 创建一个数字信号，用 match 匹配不同值显示对应文字
#[component]
fn Exercise() -> impl IntoView {
    // TODO: 创建数字信号
    let (num, set_num) = signal(1);

    view! {
        <div>
            <p>
                "数字 "
                {num}
                " 对应中文: "
                // TODO: 使用 match 表达式匹配 num 的值
                //       1 => "一", 2 => "二", 3 => "三", _ => "其他"
                {
                    match num() {
                        1 => "一",
                        2 => "二",
                        3 => "三",
                        _ => "其他",
                    }
                }
            </p>
            // TODO: 添加三个按钮分别设置 num 为 1、2、3
            <button on:click=move |_| set_num(1)>"设置 1"</button>
            <button on:click=move |_| set_num(2)>"设置 2"</button>
            <button on:click=move |_| set_num(3)>"设置 3"</button>
        </div>
    }
}

/*
<details>
<summary>参考答案</summary>

```rust
use leptos::prelude::*;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| view! { <Exercise/> });
}

#[component]
fn Exercise() -> impl IntoView {
    let (num, set_num) = signal(1);

    view! {
        <div>
            <p>
                "数字 "
                {num}
                " 对应中文: "
                {
                    match num() {
                        1 => "一",
                        2 => "二",
                        3 => "三",
                        _ => "其他",
                    }
                }
            </p>
            <button on:click=move |_| set_num(1)>"设置 1"</button>
            <button on:click=move |_| set_num(2)>"设置 2"</button>
            <button on:click=move |_| set_num(3)>"设置 3"</button>
        </div>
    }
}
```

### 知识点
- `{ match x { 1 => "一", _ => "其他" } }` 在 view! 中嵌入模式匹配
- match 的所有分支必须返回相同类型（此处均为 &str）
- 通配符 `_` 处理所有未列出的值，保证匹配穷尽性
- 通过按钮点击设置信号值，驱动视图更新

</details>
*/
