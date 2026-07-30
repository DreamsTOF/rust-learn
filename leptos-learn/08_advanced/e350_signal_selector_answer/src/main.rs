// ============================================================
// 练习 e350: Signal Selector — 从复杂状态中派生选择器
//
// 核心知识点:
//   - 使用复杂状态结构体管理多个字段（购物车）
//   - 用 Memo::new 创建带缓存的派生值（选择器）
//   - 用派生闭包创建轻量级选择器
//   - 对比 Memo 与派生闭包的缓存行为差异
//
// 难度: ⭐⭐
// ============================================================

use leptos::prelude::*;

#[derive(Clone, Debug)]
struct CartItem {
    name: String,
    price: f64,
    quantity: u32,
}

#[derive(Clone, Debug)]
struct ShoppingCart {
    items: Vec<CartItem>,
}

#[component]
fn Exercise() -> impl IntoView {
    let (cart, set_cart) = signal(ShoppingCart {
        items: vec![
            CartItem { name: "苹果".to_string(), price: 5.0, quantity: 3 },
            CartItem { name: "香蕉".to_string(), price: 3.5, quantity: 2 },
        ],
    });

    // Memo 选择器：总价（带缓存，仅当依赖变化时重算）
    let total_price = Memo::new(move |_| {
        cart.get()
            .items
            .iter()
            .map(|item| item.price * item.quantity as f64)
            .sum::<f64>()
    });

    // 派生闭包：商品总数（无缓存，每次渲染都执行）
    let total_items = move || -> u32 {
        cart.get().items.iter().map(|item| item.quantity).sum()
    };

    let add_item = move |_| {
        set_cart.update(|c| {
            c.items.push(CartItem {
                name: "橘子".to_string(),
                price: 4.0,
                quantity: 1,
            });
        });
    };

    view! {
        <div>
            <h2>"购物车"</h2>
            <ul>
                {move || cart.get().items.iter().map(|item| {
                    let subtotal = item.price * item.quantity as f64;
                    view! { <li>{format!("{} x{} = ¥{:.1}", item.name, item.quantity, subtotal)}</li> }
                }).collect::<Vec<_>>()}
            </ul>
            <p>"商品总数（派生闭包）: " {total_items}</p>
            <p>"总价（Memo）: ¥" {move || format!("{:.2}", total_price.get())}</p>
            <button on:click=add_item>"添加橘子 (¥4.0)"</button>
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
