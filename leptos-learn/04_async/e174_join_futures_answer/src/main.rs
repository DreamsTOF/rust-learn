// ============================================================
// Exercise 174 - join_futures
// ============================================================

use futures::join;
use leptos::prelude::*;
use leptos::task::spawn_local;

async fn fetch_user() -> String {
    "用户数据".to_string()
}

async fn fetch_posts() -> String {
    "帖子数据".to_string()
}

#[component]
fn Exercise() -> impl IntoView {
    let (result, set_result) = signal(String::new());

    spawn_local(async move {
        let (user, posts) = join!(fetch_user(), fetch_posts());
        set_result.set(format!("{user} + {posts}"));
    });

    view! {
        <div>
            <h2>"e174: join_futures"</h2>
            <p>{result}</p>
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
