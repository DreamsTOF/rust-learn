// ============================================================
// 练习 e43: 链式 Memo — a -> memo1 -> memo2
//
// 核心知识点:
//   - Memo::new: 创建 Memo，闭包参数为 Option<&T>（上一次的值）
//   - Memo 链式依赖: 一个 Memo 依赖另一个 Memo 的值
//   - Memo 缓存: 多次读取不会重复计算
//   - 响应式传播: a 变化 → memo1 重新计算 → memo2 重新计算
//
// 难度: ⭐⭐ (补全约 50%，关键位置有 TODO)
// ============================================================

use leptos::prelude::*;

fn main() {
    mount_to_body(|| view! { <Exercise/> });
}

/// TODO: 创建信号 a 和两个 Memo，形成链式依赖
/// 依赖链: a() -> memo1(a*2) -> memo2(memo1+3)
#[component]
fn Exercise() -> impl IntoView {
    // TODO: 创建信号 a，初始值 1
    // 完成度: signal(1)
   let (a, set_a) = signal(1);

   // TODO: 创建 memo1，计算 a 的两倍
   // 提示: Memo::new(move |_| a.get() * 2)
   let memo1 = Memo::new(move |_| a.get() * 2);

   // TODO: 创建 memo2，在 memo1 基础上加 3
   // 提示: Memo::new(move |_| memo1.get() + 3)
   let memo2 = Memo::new(move |_| memo1.get() + 3);

    view! {
        <div>
            <p>"练习 43 (memo_chain)"</p>
            // TODO: 在 view! 中显示 a、memo1、memo2 的值
            <p>"a = " {a} "（原始信号）"</p>
            <p>"memo1 = a * 2 = " {memo1}</p>
            <p>"memo2 = memo1 + 3 = " {memo2}</p>
            // TODO: 添加按钮，点击时 a 值 +1
            <button on:click=move |_| set_a.set(a.get() + 1)>"a + 1"</button>
            <p>"提示: a 变化后 memo1 和 memo2 会自动更新"</p>
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
    let (a, set_a) = signal(1);
    let memo1 = Memo::new(move |_| a.get() * 2);
    let memo2 = Memo::new(move |_| memo1.get() + 3);

    view! {
        <div>
            <p>"练习 43 (memo_chain)"</p>
            <p>"a = " {a} "（原始信号）"</p>
            <p>"memo1 = a * 2 = " {memo1()}</p>
            <p>"memo2 = memo1 + 3 = " {memo2()}</p>
            <button on:click=move |_| set_a(a() + 1)>"a + 1"</button>
        </div>
    }
}
```

### 知识点
- `Memo::new(move |_| expr)` 创建 Memo，闭包接收 `Option<&T>` 表示上一次的值（首次为 `None`）
- Memo 是惰性的: 首次读取时才运行计算闭包
- `memo()` 通过 nightly fn-call 语法读取 Memo 的值（等价于 `.get()`）
- Memo 只在其依赖的值发生变化时才重新计算，且只通知一次所有订阅者
- 链式依赖: a 变化 → memo1 重算 → memo2 重算，更新沿依赖图传播
- 与派生闭包 (`move || a.get() * 2`) 的区别: Memo 缓存计算结果，多次读取不会重复执行

</details>
*/
