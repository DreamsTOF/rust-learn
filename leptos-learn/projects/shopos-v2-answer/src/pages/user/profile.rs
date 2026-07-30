use leptos::prelude::*;
use leptos::task::spawn_local;
use thaw::*;

use crate::server::users::{get_user_profile, update_user_profile};
use crate::UserInfo;

#[component]
pub fn ProfilePage() -> impl IntoView {
    let profile = Resource::new(|| (), |_| async { get_user_profile(0i64).await }); // TODO: use actual user_id
    let username = RwSignal::new(String::new());
    let email = RwSignal::new(String::new());
    let avatar_url = RwSignal::new(String::new());
    let saving = RwSignal::new(false);
    let msg = RwSignal::new(String::new());

    let init_form = move |u: crate::UserInfo| {
        username.set(u.username);
        email.set(u.email);
        avatar_url.set(u.avatar_url.unwrap_or_default());
    };

    let save = move |_| {
        saving.set(true);
        msg.set(String::new());
        let u = username.get();
        let e = email.get();
        let a = avatar_url.get();
        let a_opt = if a.is_empty() { None } else { Some(a) };
        spawn_local(async move {
            match update_user_profile(Some(u), Some(e), a_opt, 0i64).await { // TODO: use actual user_id
                Ok(_) => msg.set("保存成功".into()),
                Err(err) => msg.set(format!("保存失败: {}", err)),
            }
            saving.set(false);
        });
    };

    view! {
        <div class="page-container">
            <h2>"个人资料"</h2>
            <Suspense fallback=move || view! { <Text>"加载中..."</Text> }>
                {move || profile.get().map(|result| {
                    match result {
                        Ok(user) => {
                            init_form(user.clone());
                            view! {
                                <Card>
                                    <Space vertical=true>
                                        <p>"用户名"</p>
                                        <input
                                            type="text"
                                            value=move || username.get()
                                            on:input=move |ev| username.set(event_target_value(&ev))
                                            style="width: 100%; padding: 4px 8px; border: 1px solid #d9d9d9; border-radius: 4px;"
                                        />
                                        <p>"邮箱"</p>
                                        <input
                                            type="text"
                                            value=move || email.get()
                                            on:input=move |ev| email.set(event_target_value(&ev))
                                            style="width: 100%; padding: 4px 8px; border: 1px solid #d9d9d9; border-radius: 4px;"
                                        />
                                        <p>"头像URL"</p>
                                        <input
                                            type="text"
                                            value=move || avatar_url.get()
                                            on:input=move |ev| avatar_url.set(event_target_value(&ev))
                                            style="width: 100%; padding: 4px 8px; border: 1px solid #d9d9d9; border-radius: 4px;"
                                        />
                                        {move || {
                                            let m = msg.get();
                                            if !m.is_empty() {
                                                view! { <Text>{m}</Text> }.into_any()
                                            } else {
                                                view! { <span></span> }.into_any()
                                            }
                                        }}
                                        <Button
                                            appearance=ButtonAppearance::Primary
                                            on_click=save
                                            disabled=move || saving.get()
                                        >
                                            {move || if saving.get() { "保存中..." } else { "保存" }}
                                        </Button>
                                    </Space>
                                </Card>
                            }.into_any()
                        }
                        Err(e) => view! { <Text>"加载失败: " {e.to_string()}</Text> }.into_any(),
                    }
                })}
            </Suspense>
        </div>
    }
}
