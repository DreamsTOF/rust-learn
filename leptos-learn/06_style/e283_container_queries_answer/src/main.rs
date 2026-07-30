use leptos::prelude::*;

#[component]
fn Exercise() -> impl IntoView {
    view! {
        <style>{"
            .container-wrapper {
                container-type: inline-size;
                resize: horizontal;
                overflow: auto;
                border: 2px dashed #aaa;
                padding: 0.5rem;
            }
            .card {
                display: flex;
                flex-direction: column;
                gap: 1rem;
                padding: 1rem;
                border: 1px solid #ddd;
                border-radius: 8px;
            }
            .card img {
                width: 100%;
                height: auto;
                border-radius: 4px;
                background: #eee;
                min-height: 80px;
            }
            .card-content h3 { margin: 0 0 0.5rem; }
            .card-content p { margin: 0; color: #666; }
            @container (min-width: 400px) {
                .card {
                    flex-direction: row;
                }
                .card img {
                    width: 150px;
                }
            }
        "}</style>
        <div>
            <h2>"Container Queries 示例"</h2>
            <p>"调整下方容器宽度，观察卡片布局变化"</p>
            <div class="container-wrapper">
                <div class="card">
                    <img src="" alt="placeholder" />
                    <div class="card-content">
                        <h3>"卡片标题"</h3>
                        <p>"这是一段卡片描述文字。当容器宽度大于 400px 时，卡片会从垂直排列变为水平排列。"</p>
                    </div>
                </div>
            </div>
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
