use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos_router::components::A;
use thaw::*;

use crate::server::auth::register_user;

#[component]
pub fn RegisterPage() -> impl IntoView {
    let username = RwSignal::new(String::new());
    let email = RwSignal::new(String::new());
    let password = RwSignal::new(String::new());
    let confirm = RwSignal::new(String::new());
    let error_msg = RwSignal::new(String::new());
    let pending = RwSignal::new(false);
    let success = RwSignal::new(false);

    let do_register = move |_| {
        let user = username.get();
        let mail = email.get();
        let pass = password.get();
        let conf = confirm.get();

        if user.is_empty() || mail.is_empty() || pass.is_empty() {
            error_msg.set("请填写所有字段".into());
            return;
        }
        if pass != conf {
            error_msg.set("两次密码输入不一致".into());
            return;
        }
        if pass.len() < 6 {
            error_msg.set("密码长度至少6位".into());
            return;
        }
        if !mail.contains('@') {
            error_msg.set("邮箱格式不正确".into());
            return;
        }

        pending.set(true);
        error_msg.set(String::new());
        spawn_local(async move {
            let result = register_user(user, mail, pass).await;
            pending.set(false);
            match result {
                Ok(_resp) => {
                    success.set(true);
                }
                Err(e) => {
                    error_msg.set(format!("注册失败: {}", e));
                }
            }
        });
    };

    view! {
        <div class="auth-container">
            <Card class="auth-card">
                <h2>"ShopOS 注册"</h2>
                {move || {
                    if success.get() {
                        view! {
                            <Space vertical=true>
                                <p>"注册成功！请登录。"</p>
                                <A href="/login">
                                    <button
                                        class="thaw-button thaw-button--primary"
                                        style="padding: 8px 16px; border: none; border-radius: 4px; cursor: pointer; background: #1890ff; color: white; font-size: 14px;"
                                    >"去登录"</button>
                                </A>
                            </Space>
                        }.into_any()
                    } else {
                        view! {
                            <Space vertical=true>
                                <div style="display: flex; flex-direction: column; gap: 4px;">
                                    <p>"用户名"</p>
                                    <input
                                        type="text"
                                        value=move || username.get()
                                        on:input=move |ev| username.set(event_target_value(&ev))
                                        style="width: 100%; padding: 4px 8px; border: 1px solid #d9d9d9; border-radius: 4px;"
                                    />
                                </div>
                                <div style="display: flex; flex-direction: column; gap: 4px;">
                                    <p>"邮箱"</p>
                                    <input
                                        type="text"
                                        value=move || email.get()
                                        on:input=move |ev| email.set(event_target_value(&ev))
                                        style="width: 100%; padding: 4px 8px; border: 1px solid #d9d9d9; border-radius: 4px;"
                                    />
                                </div>
                                <div style="display: flex; flex-direction: column; gap: 4px;">
                                    <p>"密码（至少6位）"</p>
                                    <input
                                        type="password"
                                        value=move || password.get()
                                        on:input=move |ev| password.set(event_target_value(&ev))
                                        style="width: 100%; padding: 4px 8px; border: 1px solid #d9d9d9; border-radius: 4px;"
                                    />
                                </div>
                                <div style="display: flex; flex-direction: column; gap: 4px;">
                                    <p>"确认密码"</p>
                                    <input
                                        type="password"
                                        value=move || confirm.get()
                                        on:input=move |ev| confirm.set(event_target_value(&ev))
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
                                    on:click=do_register
                                    disabled=move || pending.get()
                                >
                                    {move || if pending.get() { "注册中..." } else { "注册" }}
                                </button>
                                <A href="/login">"已有账号？去登录"</A>
                            </Space>
                        }.into_any()
                    }
                }}
            </Card>
        </div>
    }
}
