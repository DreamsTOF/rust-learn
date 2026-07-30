// ============================================================
// 练习 e85: For Nested — 嵌套 For 组件渲染二维列表
//
// 核心知识点:
//   - For 的 children 中可以再嵌套 For
//   - 内外层需分别指定 each 闭包和 key 闭包
//   - 适用于矩阵/网格/分组列表等二维数据
//
// 难度: ⭐⭐ (TODO 约 60%)
// ============================================================

use leptos::prelude::*;

#[component]
fn Exercise() -> impl IntoView {
    // 静态二维矩阵
    let matrix = vec![
        vec![1, 2, 3],
        vec![4, 5, 6],
        vec![7, 8, 9],
    ];

    view! {
        <h3>"二维矩阵"</h3>
        // TODO: 外层 For 遍历行，行内嵌套 For 遍历列
        <For each=move || matrix.clone() key=|row| row[0] let(row)>
            <div style="display: flex; gap: 6px; margin: 4px 0;">
                <For each=move || row.clone() key=|&x| x let(cell)>
                    <span style="border: 1px solid #888; padding: 4px 10px; min-width: 24px; text-align: center;">
                        {cell}
                    </span>
                </For>
            </div>
        </For>
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
//     let matrix = vec![
//         vec![1, 2, 3],
//         vec![4, 5, 6],
//         vec![7, 8, 9],
//     ];
//
//     view! {
//         <h3>"二维矩阵"</h3>
//         <For each=move || matrix.clone() key=|row| row[0] let(row)>
//             <div style="display: flex; gap: 6px; margin: 4px 0;">
//                 <For each=move || row.clone() key=|&x| x let(cell)>
//                     <span style="border: 1px solid #888; padding: 4px 10px;">{cell}</span>
//                 </For>
//             </div>
//         </For>
//     }
// }
//
// fn main() {
//     mount_to_body(Exercise);
// }
//
// ### 知识点
// - For 的 let(item) 语法可直接在 children 中使用 item
// - 嵌套 For 时内层 let 捕获外层 let 的变量
// - 内外层各自需要独立的 key 闭包
// - 静态数据也建议用 clone() 确保 each 返回 owned 数据
// </details>
