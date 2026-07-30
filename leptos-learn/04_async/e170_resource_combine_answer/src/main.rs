use leptos::prelude::*;

async fn fetch_user_name(user_id: u32) -> String {
    match user_id {
        1 => "Alice".to_string(),
        2 => "Bob".to_string(),
        _ => "访客".to_string(),
    }
}

async fn fetch_user_score(user_id: u32) -> u32 {
    match user_id {
        1 => 95,
        2 => 87,
        _ => 0,
    }
}

#[component]
fn Exercise() -> impl IntoView {
    let (user_id, set_user_id) = signal(1u32);

    let name_resource = Resource::new(
        move || user_id.get(),
        |id| async move { fetch_user_name(id).await },
    );

    let score_resource = Resource::new(
        move || user_id.get(),
        |id| async move { fetch_user_score(id).await },
    );

    let combined = move || {
        match (name_resource.get(), score_resource.get()) {
            (Some(name), Some(score)) => {
                format!("用户: {} | 分数: {}", name, score)
            }
            _ => "加载中...".to_string(),
        }
    };

    view! {
        <div>
            <h2>"组合多个 Resource"</h2>
            <button on:click=move |_| set_user_id.set(1)>"Alice"</button>
            <button on:click=move |_| set_user_id.set(2)>"Bob"</button>
            <button on:click=move |_| set_user_id.set(3)>"访客"</button>

            <p>{combined}</p>
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
