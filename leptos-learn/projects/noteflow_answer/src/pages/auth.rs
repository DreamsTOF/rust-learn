use crate::hooks::use_auth::AuthManager;
use crate::state::AppState;
use leptos::prelude::*;
use leptos_router::hooks::use_navigate;

#[component]
pub fn AuthPage() -> impl IntoView {
    let state = use_context::<AppState>().expect("AppState not provided");
    let navigate = use_navigate();
    let is_login = RwSignal::new(true);
    let username = RwSignal::new(String::new());
    let email = RwSignal::new(String::new());
    let password = RwSignal::new(String::new());
    let error = RwSignal::new(String::new());

    let submit = move |_| {
        error.set(String::new());
        if is_login.get() {
            if AuthManager::login(&state, &username.get(), &password.get()) {
                let _ = navigate("/", Default::default());
            } else {
                error.set("登录失败".to_string());
            }
        } else {
            if AuthManager::register(&state, &username.get(), &email.get(), &password.get()) {
                let _ = navigate("/", Default::default());
            } else {
                error.set("注册失败".to_string());
            }
        }
    };

    view! {
        <div class="auth-page">
            <div class="auth-card">
                <h1>"NoteFlow"</h1>
                <h2>{move || if is_login.get() { "登录" } else { "注册" }}</h2>
                <form on:submit=|ev| ev.prevent_default()>
                    <div class="form-group">
                        <label>"用户名"</label>
                        <input
                            type="text"
                            prop:value=username
                            on:input=move |ev| { username.set(event_target_value(&ev)); }
                            required=true
                        />
                    </div>
                    {move || if !is_login.get() {
                        view! {
                            <div class="form-group">
                                <label>"邮箱"</label>
                                <input
                                    type="email"
                                    prop:value=email
                                    on:input=move |ev| { email.set(event_target_value(&ev)); }
                                />
                            </div>
                        }.into_any()
                    } else {
                        view! { <div></div> }.into_any()
                    }}
                    <div class="form-group">
                        <label>"密码"</label>
                        <input
                            type="password"
                            prop:value=password
                            on:input=move |ev| { password.set(event_target_value(&ev)); }
                            required=true
                        />
                    </div>
                    {move || {
                        let err = error.get();
                        if !err.is_empty() {
                            view! { <div class="error-message">{err}</div> }.into_any()
                        } else {
                            view! { <div></div> }.into_any()
                        }
                    }}
                    <button type="button" on:click=submit>
                        {move || if is_login.get() { "登录" } else { "注册" }}
                    </button>
                </form>
                <p class="toggle-auth">
                    {move || if is_login.get() { "还没有账号？" } else { "已有账号？" }}
                    <button class="link-btn" on:click=move |_| is_login.update(|v| *v = !*v)>
                        {move || if is_login.get() { "注册" } else { "登录" }}
                    </button>
                </p>
            </div>
        </div>
    }
}
