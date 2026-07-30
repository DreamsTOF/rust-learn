use leptos::prelude::*;

#[component]
fn Exercise() -> impl IntoView {
    let items = vec![
        ("卡片 1", "这是第一张卡片的内容"),
        ("卡片 2", "这是第二张卡片的内容"),
        ("卡片 3", "这是第三张卡片的内容"),
        ("卡片 4", "这是第四张卡片的内容"),
        ("卡片 5", "这是第五张卡片的内容"),
        ("卡片 6", "这是第六张卡片的内容"),
    ];

    view! {
        // TODO: 添加 <Style> 组件，定义 CSS Grid 响应式布局
        // - .card-grid: display: grid; grid-template-columns: repeat(auto-fit, minmax(250px, 1fr)); gap: 1rem;
        // - .card: 边框(border)、圆角(border-radius)、内边距(padding)、背景色(background)、阴影(box-shadow)
        // - .card h3 / .card p: 文字颜色

        <div class="card-grid">
            {items.into_iter().map(|(title, content)| {
                view! {
                    <div class="card">
                        <h3>{title}</h3>
                        <p>{content}</p>
                    </div>
                }
            }).collect::<Vec<_>>()}
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
