// ============================================================
// 练习 e80: Show Multi-Level — 多级 Show 级联
//
// 核心知识点:
//   - Show 组件可以嵌套，实现多级条件渲染
//   - 外层 Show 控制内层 Show 的显隐，外层隐藏时内层自动卸载
//
// 难度: ⭐⭐ (TODO 约 50%)
// ============================================================

use leptos::prelude::*;

#[component]
fn Exercise() -> impl IntoView {
    // TODO: 创建三级布尔信号 level1、level2、level3（初始均为 false）
    let (level1, set_level1) = signal(false);
    let (level2, set_level2) = signal(false);
    let (level3, set_level3) = signal(false);

    view! {
        <div style="padding: 1rem;">
            <h3>"多级嵌套菜单"</h3>

            // 一级
            <button on:click=move |_| set_level1.set(!level1.get())>
                {move || if level1.get() { "▾ 收起" } else { "▸ 展开" }} " 一级"
            </button>

            // TODO: 嵌套三个 Show — 一级控制二级，二级控制三级
            <Show when=move || level1.get()>
                <div style="padding: 8px 16px; border: 1px solid #ccc; margin-top: 4px;">
                    <p>"📁 一级内容"</p>

                    <button on:click=move |_| set_level2.set(!level2.get())>
                        {move || if level2.get() { "▾ 收起" } else { "▸ 展开" }} " 二级"
                    </button>

                    <Show when=move || level2.get()>
                        <div style="padding: 8px 16px; border: 1px solid #aaa; margin-top: 4px; margin-left: 16px;">
                            <p>"📂 二级内容"</p>

                            <button on:click=move |_| set_level3.set(!level3.get())>
                                {move || if level3.get() { "▾ 收起" } else { "▸ 展开" }} " 三级"
                            </button>

                            <Show when=move || level3.get()>
                                <div style="padding: 8px 16px; border: 1px solid #888; margin-top: 4px; margin-left: 16px; background: #f5f5f5;">
                                    <p>"📄 三级内容（最深层）"</p>
                                </div>
                            </Show>
                        </div>
                    </Show>
                </div>
            </Show>
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
//     let (level1, set_level1) = signal(false);
//     let (level2, set_level2) = signal(false);
//     let (level3, set_level3) = signal(false);
//
//     view! {
//         <div style="padding: 1rem;">
//             <h3>"多级嵌套菜单"</h3>
//             <button on:click=move |_| set_level1.set(!level1.get())>
//                 {move || if level1.get() { "▾ 收起" } else { "▸ 展开" }} " 一级"
//             </button>
//             <Show when=move || level1.get()>
//                 <div style="padding: 8px 16px; border: 1px solid #ccc; margin-top: 4px;">
//                     <p>"📁 一级内容"</p>
//                     <button on:click=move |_| set_level2.set(!level2.get())>
//                         {move || if level2.get() { "▾ 收起" } else { "▸ 展开" }} " 二级"
//                     </button>
//                     <Show when=move || level2.get()>
//                         <div style="padding: 8px 16px; border: 1px solid #aaa; margin-top: 4px; margin-left: 16px;">
//                             <p>"📂 二级内容"</p>
//                             <button on:click=move |_| set_level3.set(!level3.get())>
//                                 {move || if level3.get() { "▾ 收起" } else { "▸ 展开" }} " 三级"
//                             </button>
//                             <Show when=move || level3.get()>
//                                 <div style="padding: 8px 16px; border: 1px solid #888; margin-top: 4px; margin-left: 16px; background: #f5f5f5;">
//                                     <p>"📄 三级内容（最深层）"</p>
//                                 </div>
//                             </Show>
//                         </div>
//                     </Show>
//                 </div>
//             </Show>
//         </div>
//     }
// }
//
// fn main() {
//     mount_to_body(Exercise);
// }
//
// ### 知识点
// - Show 组件可以无限嵌套，每层独立响应式控制
// - 外层 Show 的 when=false 会卸载整个子树（包含内层 Show），避免不必要的渲染
// - 这种模式常用于树形菜单、多级折叠面板等场景
// </details>
