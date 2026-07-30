use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos_router::components::A;
use thaw::*;

use crate::server::auth::login_user;

#[component]
pub fn LoginPage() -> impl IntoView {
    let username = RwSignal::new(String::new());
    let password = RwSignal::new(String::new());
    let error_msg = RwSignal::new(String::new());
    let pending = RwSignal::new(false);

    let do_login = move |_| {
        let user = username.get();
        let pass = password.get();
        if user.is_empty() || pass.is_empty() {
            error_msg.set("请输入用户名和密码".into());
            return;
        }
        pending.set(true);
        error_msg.set(String::new());
        spawn_local(async move {
            let result = login_user(user, pass).await;
            pending.set(false);
            match result {
                Ok(_resp) => {
                    let _ = leptos_router::hooks::use_navigate()("/admin/dashboard", Default::default());
                }
                Err(e) => {
                    error_msg.set(format!("登录失败: {}", e));
                }
            }
        });
    };

    view! {
        <div class="auth-container">
            <Card class="auth-card">
                <h2>"ShopOS 登录"</h2>
                <Space vertical=true>
                    <div style="display: flex; flex-direction: column; gap: 4px;">
                        <p>"用户名 / 邮箱"</p>
                        <input
                            type="text"
                            value=move || username.get()
                            on:input=move |ev| username.set(event_target_value(&ev))
                            style="width: 100%; padding: 4px 8px; border: 1px solid #d9d9d9; border-radius: 4px;"
                        />
                    </div>
                    <div style="display: flex; flex-direction: column; gap: 4px;">
                        <p>"密码"</p>
                        <input
                            type="password"
                            value=move || password.get()
                            on:input=move |ev| password.set(event_target_value(&ev))
                            style="width: 100%; padding: 4px 8px; border: 1px solid #d9d9d9; border-radius: 4px;"
                        />
                    </div>
                    {move || {
                        let msg = error_msg.get();
                        if msg.is_empty() {
                            view! { <span></span> }.into_any()
                        } else {
                            view! { <span style="color: #ff4d4f;">"错误: " {msg}</span> }.into_any()
                        }
                    }}
                    <button
                        class="thaw-button thaw-button--primary"
                        style="width: 100%; padding: 8px 16px; border: none; border-radius: 4px; cursor: pointer; background: #1890ff; color: white; font-size: 14px;"
                        on:click=do_login
                        disabled=move || pending.get()
                    >
                        {move || if pending.get() { "登录中..." } else { "登录" }}
                    </button>
                    <A href="/register">"没有账号？去注册"</A>
                </Space>
            </Card>
        </div>
    }
}
