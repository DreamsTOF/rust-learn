use leptos::prelude::*;
use leptos::task::spawn_local;
use thaw::*;

use crate::server::auth::change_password;

#[component]
pub fn SecurityPage() -> impl IntoView {
    let old_pw = RwSignal::new(String::new());
    let new_pw = RwSignal::new(String::new());
    let confirm_pw = RwSignal::new(String::new());
    let msg = RwSignal::new(String::new());
    let is_error = RwSignal::new(false);
    let pending = RwSignal::new(false);

    let do_change = move |_| {
        let old = old_pw.get();
        let new = new_pw.get();
        let confirm = confirm_pw.get();

        if old.is_empty() || new.is_empty() {
            msg.set("请填写所有字段".into());
            is_error.set(true);
            return;
        }
        if new != confirm {
            msg.set("两次密码不一致".into());
            is_error.set(true);
            return;
        }
        if new.len() < 6 {
            msg.set("新密码长度至少6位".into());
            is_error.set(true);
            return;
        }

        pending.set(true);
        msg.set(String::new());
        spawn_local(async move {
            let result = change_password(old, new, 0i64).await; // TODO: use actual user_id
            pending.set(false);
            match result {
                Ok(_) => {
                    msg.set("密码修改成功".into());
                    is_error.set(false);
                    old_pw.set(String::new());
                    new_pw.set(String::new());
                    confirm_pw.set(String::new());
                }
                Err(e) => {
                    msg.set(format!("修改失败: {}", e));
                    is_error.set(true);
                }
            }
        });
    };

    view! {
        <div class="page-container">
            <h2>"安全设置"</h2>
            <Card>
                <Space vertical=true>
                    <p>"旧密码"</p>
                    <input
                        type="password"
                        value=move || old_pw.get()
                        on:input=move |ev| old_pw.set(event_target_value(&ev))
                        style="width: 100%; padding: 4px 8px; border: 1px solid #d9d9d9; border-radius: 4px;"
                    />
                    <p>"新密码"</p>
                    <input
                        type="password"
                        value=move || new_pw.get()
                        on:input=move |ev| new_pw.set(event_target_value(&ev))
                        style="width: 100%; padding: 4px 8px; border: 1px solid #d9d9d9; border-radius: 4px;"
                    />
                    <p>"确认新密码"</p>
                    <input
                        type="password"
                        value=move || confirm_pw.get()
                        on:input=move |ev| confirm_pw.set(event_target_value(&ev))
                        style="width: 100%; padding: 4px 8px; border: 1px solid #d9d9d9; border-radius: 4px;"
                    />
                    {move || {
                        let m = msg.get();
                        if !m.is_empty() {
                            (if is_error.get() {
                                view! { <Text>"错误: " {m}</Text> }
                            } else {
                                view! { <Text>"成功: " {m}</Text> }
                            }).into_any()
                        } else {
                            view! { <span></span> }.into_any()
                        }
                    }}
                    <Button
                        appearance=ButtonAppearance::Primary
                        on_click=do_change
                        disabled=move || pending.get()
                    >
                        {move || if pending.get() { "修改中..." } else { "修改密码" }}
                    </Button>
                </Space>
            </Card>
        </div>
    }
}
