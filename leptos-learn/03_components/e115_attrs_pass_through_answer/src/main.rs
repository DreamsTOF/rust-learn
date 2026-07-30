// ============================================================
// Exercise e115: attrs_pass_through — Answer
// ============================================================

use leptos::prelude::*;

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
            <StyledCard
                title="带全部属性"
                class="primary-card"
                id="card1"
                style="border: 2px solid #0d6efd;"
            >
                <p>"这个卡片有 class、id 和 style"</p>
            </StyledCard>
            <StyledCard
                title="仅带样式"
                style="background: #f0f0f0; padding: 8px;"
            >
                <p>"这个卡片只有 style"</p>
            </StyledCard>
            <StyledCard title="无额外属性">
                <p>"这个卡片没有任何额外属性透传"</p>
            </StyledCard>
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
