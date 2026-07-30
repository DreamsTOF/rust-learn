// ============================================================
// 练习 e350: Signal Selector — 从复杂状态中派生选择器
//
// 核心知识点:
//   - 使用复杂状态结构体管理多个字段（购物车）
//   - 用 Memo::new 创建带缓存的派生值（选择器）
//   - 用派生闭包创建轻量级选择器
//   - 对比 Memo 与派生闭包的缓存行为差异
//
// 难度: ⭐⭐ (关键位置有 TODO，补全约 50%)
// ============================================================

use leptos::prelude::*;

// TODO: 定义 CartItem 结构体（name: String, price: f64, quantity: u32）
// 提示: 需要派生 Clone + Debug

// TODO: 定义 ShoppingCart 结构体（items: Vec<CartItem>）
// 提示: 需要派生 Clone + Debug

#[component]
fn Exercise() -> impl IntoView {
    // TODO: 使用 signal() 创建一个包含 ShoppingCart 的状态
    // 初始化为包含"苹果"(¥5.0 x3) 和"香蕉"(¥3.5 x2) 的购物车

    // TODO: 使用 Memo::new 创建总价选择器（带缓存）
    // 提示: Memo::new(move |_| { cart.get().items.iter().map(...).sum::<f64>() })

    // TODO: 使用派生闭包创建商品总数选择器（轻量，无缓存）
    // 提示: move || cart.get().items.iter().map(|i| i.quantity).sum::<u32>()

    // TODO: 创建"添加橘子"按钮，每次向购物车添加一个橘子(¥4.0 x1)
    // 提示: set_cart.update(|c| c.items.push(CartItem { ... }));

    view! {
        <div>
            <h2>"购物车"</h2>
            // TODO: 遍历 cart 的 items 显示每个商品（名称 x 数量 = 小计）
            // TODO: 显示商品总数（派生闭包）
            // TODO: 显示总价（Memo）
            // TODO: 添加"添加橘子"按钮
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}

// <details>
// 参考答案（去除注释后的纯净版本）:
//
// use leptos::prelude::*;
//
// #[derive(Clone, Debug)]
// struct CartItem {
//     name: String,
//     price: f64,
//     quantity: u32,
// }
//
// #[derive(Clone, Debug)]
// struct ShoppingCart {
//     items: Vec<CartItem>,
// }
//
// #[component]
// fn Exercise() -> impl IntoView {
//     let (cart, set_cart) = signal(ShoppingCart {
//         items: vec![
//             CartItem { name: "苹果".to_string(), price: 5.0, quantity: 3 },
//             CartItem { name: "香蕉".to_string(), price: 3.5, quantity: 2 },
//         ],
//     });
//
//     // Memo 选择器：总价（带缓存，仅当依赖变化时重算）
//     let total_price = Memo::new(move |_| {
//         cart.get().items.iter()
//             .map(|item| item.price * item.quantity as f64)
//             .sum::<f64>()
//     });
//
//     // 派生闭包：商品总数（无缓存，每次渲染都执行）
//     let total_items = move || -> u32 {
//         cart.get().items.iter().map(|item| item.quantity).sum()
//     };
//
//     let add_item = move |_| {
//         set_cart.update(|c| {
//             c.items.push(CartItem {
//                 name: "橘子".to_string(),
//                 price: 4.0,
//                 quantity: 1,
//             });
//         });
//     };
//
//     view! {
//         <div>
//             <h2>"购物车"</h2>
//             <ul>
//                 {move || cart.get().items.iter().map(|item| {
//                     let subtotal = item.price * item.quantity as f64;
//                     view! { <li>{format!("{} x{} = ¥{:.1}", item.name, item.quantity, subtotal)}</li> }
//                 }).collect::<Vec<_>>()}
//             </ul>
//             <p>"商品总数（派生闭包）: " {total_items}</p>
//             <p>"总价（Memo）: ¥" {move || format!("{:.2}", total_price.get())}</p>
//             <button on:click=add_item>"添加橘子 (¥4.0)"</button>
//         </div>
//     }
// }
//
// fn main() {
//     mount_to_body(Exercise);
// }
//
// ### 知识点
// - signal + 结构体模式：用嵌套结构体管理复杂状态
// - `set_cart.update(f)` 通过 &mut T 原地修改，避免创建新实例
// - `Memo::new(f)` 创建带缓存的计算值，仅依赖变化时重算
// - 派生闭包 `move || expr` 每次渲染时重新计算
// - 选择器模式：从复杂状态中投影出特定子集或计算值
// - 性能建议：计算昂贵时用 Memo，简单计算用派生闭包
// </details>
