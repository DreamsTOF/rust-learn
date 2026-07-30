// ============================================================
// 练习 e46: 惰性派生 — 闭包捕获信号但不立即计算
//
// 核心知识点:
//   - 派生闭包 (`move || expr`) 惰性求值: 捕获信号但不运行
//   - Memo 也是惰性的: 创建时不运行，首次读取时才执行
//   - 信号本身是 eager 的: 创建时就有初始值
//   - 对比: 派生 vs 信号的计算时机
//
// 难度: ⭐⭐ (补全约 50%，关键位置有 TODO)
// ============================================================

use leptos::prelude::*;

fn main() {
    mount_to_body(|| view! { <Exercise/> });
}

/// TODO: 演示派生闭包的惰性特性
#[component]
fn Exercise() -> impl IntoView {
    // TODO: 创建信号，初始值 10
   let (a, set_a) = signal(10);

   // 派生闭包: 捕获 a 但不立即执行
   // 只有每次被调用时才会计算 a + 1
   let derived = move || a.get() + 1;

   // Memo: 也是惰性的，创建时不会运行
   let memo = Memo::new(move |_| {
       // 这个闭包在创建时不会运行
       a.get() * 10
   });

    // 标记是否已读取 Memo
    let (read_memo, set_read_memo) = signal(false);

    view! {
        <div>
            <p>"练习 46 (lazy_derived)"</p>
            <p>"信号 a = " {a.get()}</p>
            // TODO: 显示派生闭包的值（每次渲染时计算）
            <p>"派生 a+1 = " {derived}</p>
            // TODO: 显示 Memo 的值（仅在首次读取时计算，之后缓存）
           <p>"Memo a*10 = " {move || if read_memo.get() { memo.get().to_string() } else { "未读取（惰性）".to_string() }}</p>
           <button on:click=move |_| set_a.set(a.get() + 1)>"a + 1"</button>
           <button on:click=move |_| { set_read_memo.set(true); }>"读取 Memo"</button>
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
    let (a, set_a) = signal(10);
    let derived = move || a.get() + 1;
    let memo = Memo::new(move |_| a.get() * 10);
    let (read_memo, set_read_memo) = signal(false);

    view! {
        <div>
            <p>"练习 46 (lazy_derived)"</p>
            <p>"信号 a = " {a.get()}</p>
            <p>"派生 a+1 = " {derived}</p>
            <p>"Memo a*10 = " {move || if read_memo.get() { memo.get().to_string() } else { "未读取（惰性）".to_string() }}</p>
            <button on:click=move |_| set_a.set(a.get() + 1)>"a + 1"</button>
            <button on:click=move |_| { set_read_memo.set(true); }>"读取 Memo"</button>
        </div>
    }
}
```

### 知识点
- **信号**: 创建时即拥有初始值（eager）
- **派生闭包** (`move || expr`): 仅是闭包，捕获信号但不执行，每次调用才计算
- **Memo**: 创建时闭包不运行，首次读取（`.get()` / 函数调用）时执行一次，之后缓存结果
- 当源信号变化时:
  - 派生闭包: 每次读取都重新计算（不缓存）
  - Memo: 惰性重算 — 只在被读取且源信号已变化时才执行
- 惰性求值是 Leptos 响应式系统的核心设计，避免不必要的计算

</details>
*/
