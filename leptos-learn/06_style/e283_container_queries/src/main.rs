use leptos::prelude::*;

#[component]
fn Exercise() -> impl IntoView {
    view! {
        // TODO: 添加 <Style> 组件，定义容器查询样式
        // - .container-wrapper: container-type: inline-size; resize: horizontal; overflow: auto;
        // - .card: display: flex; flex-direction: column; (默认垂直排列)
        // - @container (min-width: 400px) { .card { flex-direction: row; } } (宽容器时水平排列)

        <div>
            <h2>"Container Queries 示例"</h2>
            <p>"调整下方容器宽度，观察卡片布局变化"</p>
            <div class="container-wrapper">
                <div class="card">
                    // TODO: 添加卡片内容
                    // - 左侧图片占位 <img src="" alt="placeholder" />
                    // - 右侧 .card-content 包含标题和描述文字
                </div>
            </div>
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
