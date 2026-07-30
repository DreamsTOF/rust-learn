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
        <style>{"
            .card-grid {
                display: grid;
                grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
                gap: 1rem;
                padding: 1rem;
            }
            .card {
                border: 1px solid #ddd;
                border-radius: 8px;
                padding: 1rem;
                background: #f9f9f9;
                box-shadow: 0 2px 4px rgba(0,0,0,0.1);
            }
            .card h3 {
                margin: 0 0 0.5rem 0;
                color: #333;
            }
            .card p {
                margin: 0;
                color: #666;
            }
        "}</style>
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
