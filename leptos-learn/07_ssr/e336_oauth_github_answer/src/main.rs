// ============================================================
// Exercise 336 - Answer: GitHub OAuth Authorization Code Flow
// ============================================================

use leptos::prelude::*;

#[derive(Clone)]
struct GitHubUser {
    id: u64,
    login: String,
    avatar_url: String,
    name: String,
}

#[component]
fn Exercise() -> impl IntoView {
    let (user, set_user) = signal::<Option<GitHubUser>>(None);

    let login = move |_| {
        // Simulate successful OAuth authorization code flow
        set_user.set(Some(GitHubUser {
            id: 12345678,
            login: "octocat".into(),
            avatar_url: "https://avatars.githubusercontent.com/u/12345678".into(),
            name: "Octocat (Mona)".into(),
        }));
    };

    let logout = move |_| set_user.set(None);

    view! {
        <div style="max-width: 420px; margin: 2rem auto; padding: 1.5rem; border: 1px solid #d0d7de; border-radius: 8px; font-family: system-ui, sans-serif;">
            <h2 style="margin: 0 0 0.25rem;">"GitHub OAuth Demo"</h2>
            <p style="color: #656d76; font-size: 0.85rem; margin: 0 0 1rem;">
                "Authorization Code Flow"
            </p>
            <hr style="margin: 0 0 1rem; border: none; border-top: 1px solid #d0d7de;" />

            {move || match user.get() {
                None => view! {
                    <button on:click=login
                        style="padding: 0.75rem 1.5rem; background: #24292e; color: #fff; border: none; border-radius: 6px; cursor: pointer; font-size: 1rem;">
                        "Login with GitHub"
                    </button>
                }.into_any(),
                Some(u) => view! {
                    <div style="display: flex; flex-direction: column; align-items: center; gap: 0.75rem;">
                        <img src=u.avatar_url alt="avatar"
                            style="width: 80px; height: 80px; border-radius: 50%; border: 2px solid #d0d7de;" />
                        <h3 style="margin: 0;">{u.name}</h3>
                        <p style="color: #656d76; margin: 0;">"@" {u.login}</p>
                        <p style="color: #656d76; font-size: 0.85rem; margin: 0;">
                            "GitHub ID: " {u.id.to_string()}
                        </p>
                        <button on:click=logout
                            style="padding: 0.5rem 1rem; background: #cb2431; color: #fff; border: none; border-radius: 4px; cursor: pointer;">
                            "Logout"
                        </button>
                    </div>
                }.into_any(),
            }}
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
