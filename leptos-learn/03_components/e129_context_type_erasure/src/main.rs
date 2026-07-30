// ============================================================
// 练习 e129: Context 类型擦除 — AnyMap 模式
//
// 核心知识点:
//   - Context 底层: HashMap<TypeId, Box<dyn Any>>
//   - 类型擦除: 不同类型可共存，按类型精确检索
//
// 难度: ⭐⭐⭐ (补全关键位置)
// ============================================================

use leptos::prelude::*;

#[component]
fn Child() -> impl IntoView {
    // TODO: 从 context 中检索不同类型 (String, u32, f64)
    let name = use_context::<String>().unwrap_or_default();
    let score = use_context::<u32>().unwrap_or(0);
    let ratio = use_context::<f64>().unwrap_or(0.0);
    // 不同类型通过 TypeId 区分，在同一个 map 中共存

    view! {
        <p>"名称: " {name}</p>
        <p>"分数: " {score}</p>
        <p>"比例: " {ratio}</p>
    }
}

#[component]
fn Exercise() -> impl IntoView {
    // TODO: 提供多个不同类型的值到 context
    // 底层使用 HashMap<TypeId, Box<dyn Any>> 存储
    provide_context(String::from("Leptos"));
    provide_context(95u32);
    provide_context(3.14f64);

    view! {
        <h2>"Context 类型擦除（AnyMap）"</h2>
        <p><em>"不同类型通过 TypeId 区分"</em></p>
        <Child/>
    }
}

fn main() {
    mount_to_body(Exercise);
}

// <details>
// 参考答案 (去除注释后的纯净版本):
//
// use leptos::prelude::*;
//
// #[component]
// fn Child() -> impl IntoView {
//     let name = use_context::<String>().unwrap_or_default();
//     let score = use_context::<u32>().unwrap_or(0);
//     let ratio = use_context::<f64>().unwrap_or(0.0);
//     view! {
//         <p>"名称: " {name}</p>
//         <p>"分数: " {score}</p>
//         <p>"比例: " {ratio}</p>
//     }
// }
//
// #[component]
// fn Exercise() -> impl IntoView {
//     provide_context(String::from("Leptos"));
//     provide_context(95u32);
//     provide_context(3.14f64);
//     view! {
//         <h2>"Context 类型擦除（AnyMap）"</h2>
//         <p><em>"不同类型通过 TypeId 区分"</em></p>
//         <Child/>
//     }
// }
//
// fn main() {
//     mount_to_body(Exercise);
// }
//
// ### 知识点
// - Context 底层: HashMap<TypeId, Box<dyn Any>> (类型擦除)
// - 不同类型可共存，同类型只能有一个值
// - 按类型检索是类型安全的 (编译期 TypeId 保证)
// - "类型擦除": 存储时丢失具体类型信息，检索时通过 TypeId 恢复
// </details>
