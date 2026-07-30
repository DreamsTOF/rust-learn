// ============================================================
// Exercise 165 - Transition vs Suspense
// ============================================================

use leptos::prelude::*;
use std::time::Duration;

async fn delay(ms: u64) {
    let (tx, rx) = futures::channel::oneshot::channel::<()>();
    set_timeout(move || { let _ = tx.send(()); }, Duration::from_millis(ms));
    rx.await.unwrap();
}

async fn load_page(page: u32) -> String {
    delay(2000).await;
    format!("Page {} content (loaded at {:?})", page, std::time::SystemTime::now())
}

#[component]
fn Exercise() -> impl IntoView {
    let (suspense_page, set_suspense_page) = signal(1u32);
    let (transition_page, set_transition_page) = signal(1u32);

    let suspense_data = Resource::new(
        move || suspense_page.get(),
        |page| async move { load_page(page).await },
    );
    let transition_data = Resource::new(
        move || transition_page.get(),
        |page| async move { load_page(page).await },
    );

    view! {
        <div>
            <h2>"Exercise 165: Transition vs Suspense"</h2>
            <div style="display: flex; gap: 20px;">
                <div style="flex: 1; padding: 10px; border: 1px solid #e74c3c;">
                    <h3>"&lt;Suspense&gt;"</h3>
                    <button on:click=move |_| set_suspense_page.update(|n| *n += 1)>
                        "Next Page"
                    </button>
                    <Suspense fallback=|| view! { <p style="color: #e74c3c;">"Suspense: Loading..."</p> }>
                        <p>{move || suspense_data.get()}</p>
                    </Suspense>
                </div>
                <div style="flex: 1; padding: 10px; border: 1px solid #27ae60;">
                    <h3>"&lt;Transition&gt;"</h3>
                    <button on:click=move |_| set_transition_page.update(|n| *n += 1)>
                        "Next Page"
                    </button>
                    <Transition fallback=|| view! { <p style="color: #27ae60;">"Transition: Loading..."</p> }>
                        <p>{move || transition_data.get()}</p>
                    </Transition>
                </div>
            </div>
            <hr/>
            <p>"提示: 点击按钮后，Suspense 会清空并显示 fallback，Transition 保留旧内容直到新数据到达。"</p>
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
