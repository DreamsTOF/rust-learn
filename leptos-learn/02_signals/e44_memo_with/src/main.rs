// ============================================================
// 练习 e44: Memo 的 .with() 方法 — 避免不必要的克隆
//
// 核心知识点:
//   - .get() : 克隆 Memo 内部的值返回（需要 T: Clone）
//   - .with() : 通过闭包访问 Memo 值的引用，避免克隆
//   - .with(|v| ...) 在闭包内对引用进行操作
//   - 适用于大型数据结构（如 String、Vec）时节省性能
//
// 难度: ⭐⭐ (补全约 50%，关键位置有 TODO)
// ============================================================

use leptos::prelude::*;

fn main() {
    mount_to_body(|| view! { <Exercise/> });
}

/// TODO: 创建 String 信号和 Memo，演示 .with() 避免克隆
#[component]
fn Exercise() -> impl IntoView {
    // TODO: 创建字符串信号
    let (name, set_name) = signal(String::from("Leptos"));

    // TODO: 创建 Memo，将 name 转为大写
    let upper = Memo::new(move |_| name.get().to_uppercase());

    // TODO: 使用 .with() 访问 Memo 的值引用（不克隆）
    //       .get()  会克隆整个 String
    //       .with() 通过引用访问
    // 这里演示两种方式的效果

    view! {
        <div>
            <p>"练习 44 (memo_with)"</p>
            <p>"原始值: " {name}</p>
            // TODO: 用 .get() 读取 Memo（发生克隆）
            <p>"upper.get()  = " {upper.get()}</p>
            // TODO: 用 .with() 读取 Memo
            // .with() 返回闭包的结果，这里用 clone 在闭包内复制以便显示
            <p>"upper.with() = " {move || upper.with(|s| s.clone())}</p>
            <p>"长度 (.with): " {move || upper.with(|s| s.len())}</p>
            <button on:click=move |_| set_name.set(format!("{}x", name.get()))>"追加 x"</button>
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
    let (name, set_name) = signal(String::from("Leptos"));
    let upper = Memo::new(move |_| name().to_uppercase());

    view! {
        <div>
            <p>"练习 44 (memo_with)"</p>
            <p>"原始值: " {name}</p>
            <p>"upper.get()  = " {upper.get()}</p>
            <p>"upper.with() = " {move || upper.with(|s| s.clone())}</p>
            <p>"长度 (.with): " {move || upper.with(|s| s.len())}</p>
            <button on:click=move |_| set_name(format!("{}x", name()))>"追加 x"</button>
        </div>
    }
}
```

### 知识点
- `.get()` 要求 `T: Clone`，每次调用克隆整个值
- `.with(f)` 传入闭包 `f: &T -> O`，通过引用访问值，不产生克隆
- `.with()` 返回闭包 `f` 的返回值 `O`
- 对于 `String`、`Vec` 等堆分配类型，`.with()` 可避免不必要的内存分配
- 在 `view!` 中使用 `.with()` 时需用 `move || upper.with(...)` 包装为闭包以保持响应式

</details>
*/
