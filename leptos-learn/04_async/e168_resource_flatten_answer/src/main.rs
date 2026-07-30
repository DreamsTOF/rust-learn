use leptos::prelude::*;

async fn fetch_user(id: u32) -> Result<String, String> {
    match id {
        1 => Ok("Alice".to_string()),
        2 => Ok("Bob".to_string()),
        _ => Err("用户不存在".to_string()),
    }
}

#[component]
fn Exercise() -> impl IntoView {
    let (user_id, set_user_id) = signal(1u32);

    let user_resource = Resource::new(
        move || user_id.get(),
        |id| async move { fetch_user(id).await },
    );

    view! {
        <div>
            <h2>"展平 Resource 示例"</h2>
            <button on:click=move |_| set_user_id.set(1)>"Alice"</button>
            <button on:click=move |_| set_user_id.set(2)>"Bob"</button>
            <button on:click=move |_| set_user_id.set(3)>"未知"</button>

            <hr/>

            <Transition
                fallback=|| view! { <p>"加载中..."</p> }
            >
                {move || Suspend::new(async move {
                    user_resource.await
                        .map(|user| view! { <p>"用户: " {user}</p> }.into_any())
                        .unwrap_or_else(|e| view! { <p class="error">"错误: " {e}</p> }.into_any())
                })}
            </Transition>
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
