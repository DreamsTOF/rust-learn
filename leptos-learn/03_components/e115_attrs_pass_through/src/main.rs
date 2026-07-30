// ============================================================
// 练习 e115: attrs_pass_through — 属性透传
//
// 核心知识点:
//   - 透传 HTML 属性到根元素
//   - 使用 #[prop(optional)] 处理可选属性
//   - 属性值为 None 时不渲染该属性
//
// 难度: ⭐⭐⭐
// ============================================================

use leptos::prelude::*;

// TODO: 定义 StyledCard 组件，将 class、id、style 透传到根元素
#[component]
fn StyledCard(
    /// 卡片标题
    title: &'static str,
    /// 透传给根 div 的 class 属性（可选）
    #[prop(optional)]
    class: Option<&'static str>,
    /// 透传给根 div 的 id 属性（可选）
    #[prop(optional)]
    id: Option<&'static str>,
    /// 透传给根 div 的 style 属性（可选）
    #[prop(optional)]
    style: Option<&'static str>,
    /// 子节点
    children: Children,
) -> impl IntoView {
    view! {
        // TODO: 将 class、id、style 透传给此 div
        // 提示: None 值时属性不会渲染到 DOM
        <div class={class} id={id} style={style}>
            <h3>{title}</h3>
            <div class="card-body">
                {children()}
            </div>
        </div>
    }
}

#[component]
fn Exercise() -> impl IntoView {
    view! {
        <div>
            <h3>"属性透传"</h3>
            // TODO: 使用 StyledCard，分别测试不同透传情况
            // 传入所有属性
            <StyledCard
                title="带全部属性"
                class="primary-card"
                id="card1"
                style="border: 2px solid #0d6efd;"
            >
                <p>"这个卡片有 class、id 和 style"</p>
            </StyledCard>
            // 只传部分属性
            <StyledCard
                title="仅带样式"
                style="background: #f0f0f0; padding: 8px;"
            >
                <p>"这个卡片只有 style"</p>
            </StyledCard>
            // 不传额外属性
            <StyledCard title="无额外属性">
                <p>"这个卡片没有任何额外属性透传"</p>
            </StyledCard>
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}

// ============================================================
// 参考答案 (思考后再看!)
// ============================================================
// <details>
// <summary>点击展开答案</summary>
//
// ### 代码
// ```rust
// use leptos::prelude::*;
//
// #[component]
// fn StyledCard(
//     title: &'static str,
//     #[prop(optional)]
//     class: Option<&'static str>,
//     #[prop(optional)]
//     id: Option<&'static str>,
//     #[prop(optional)]
//     style: Option<&'static str>,
//     children: Children,
// ) -> impl IntoView {
//     view! {
//         <div class={class} id={id} style={style}>
//             <h3>{title}</h3>
//             <div class="card-body">
//                 {children()}
//             </div>
//         </div>
//     }
// }
//
// #[component]
// fn Exercise() -> impl IntoView {
//     view! {
//         <div>
//             <h3>"属性透传"</h3>
//             <StyledCard
//                 title="带全部属性"
//                 class="primary-card"
//                 id="card1"
//                 style="border: 2px solid #0d6efd;"
//             >
//                 <p>"这个卡片有 class、id 和 style"</p>
//             </StyledCard>
//             <StyledCard
//                 title="仅带样式"
//                 style="background: #f0f0f0; padding: 8px;"
//             >
//                 <p>"这个卡片只有 style"</p>
//             </StyledCard>
//             <StyledCard title="无额外属性">
//                 <p>"这个卡片没有任何额外属性透传"</p>
//             </StyledCard>
//         </div>
//     }
// }
//
// fn main() {
//     mount_to_body(Exercise);
// }
// ```
//
// ### 知识点
// - `#[prop(optional)]` 配合 `Option<T>` 让 prop 可选，不传时为 `None`
// - 属性值为 `None` 时，Leptos 不会在 DOM 上渲染该属性
// - 这种模式常用于封装 HTML 元素的组件（如 Card、Button、Input）
// - 透传使用户可以控制组件根元素的样式和行为，提升组件灵活性
//
// </details>
