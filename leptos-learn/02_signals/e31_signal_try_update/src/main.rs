// ============================================================
// 练习 e31: signal_try_update — 带返回值的更新操作
//
// 核心知识点:
//   - .try_update() 闭包可以返回值（.update() 只能返回 ()）
//   - 返回类型是 Option<R>，信号已丢弃时返回 None
//   - 适用于"读取旧值并设置新值"的原子操作
//
// 难度: ⭐⭐⭐
// ============================================================

use leptos::prelude::*;

#[component]
fn Exercise() -> impl IntoView {
    // TODO: 创建 count 信号
    let (count, set_count) = signal(0);
    // TODO: 创建 last_old 信号存放 try_update 返回的旧值
    let (last_old, set_last_old) = signal(Option::<i32>::None);

    let do_try_update = move |_| {
        // TODO: 使用 try_update，闭包返回旧值，同时更新 count
        // 提示: set_count.try_update(|n| { let prev = *n; *n = ...; prev })
        //       返回 Option<i32>
        let old = set_count.try_update(|n| {
            let prev = *n;
            *n = *n + 10;
            prev
        });
        set_last_old.set(old);
    };

    view! {
        <div>
            <p>"当前值: " {count}</p>
            <p>
                "上一次旧值: "
                {move || match last_old.get() {
                    Some(v) => format!("{}", v),
                    None => "暂无".to_string(),
                }}
            </p>
            <button on:click=do_try_update>"try_update (加 10)"</button>
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
// #[component]
// fn Exercise() -> impl IntoView {
//     let (count, set_count) = signal(0);
//     let (last_old, set_last_old) = signal(Option::<i32>::None);
//
//     let do_try_update = move |_| {
//         let old = set_count.try_update(|n| {
//             let prev = *n;
//             *n = *n + 10;
//             prev
//         });
//         set_last_old.set(old);
//     };
//
//     view! {
//         <div>
//             <p>"当前值: " {count}</p>
//             <p>
//                 "上一次旧值: "
//                 {move || match last_old.get() {
//                     Some(v) => format!("{}", v),
//                     None => "暂无".to_string(),
//                 }}
//             </p>
//             <button on:click=do_try_update>"try_update (加 10)"</button>
//         </div>
//     }
// }
//
// fn main() {
//     mount_to_body(Exercise);
// }
//
// 知识点:
// 1. try_update 的闭包可以返回值，整个调用返回 Option<R>
// 2. 信号已丢弃时 try_update 返回 None 而不执行闭包
// 3. update 等价于 try_update(|x| { f(x); }) 并忽略返回值
// </details>
