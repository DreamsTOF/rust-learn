use leptos::prelude::*;

#[component]
fn Exercise() -> impl IntoView {
    let (name, set_name) = signal(String::new());
    let (email, set_email) = signal(String::new());
    // TODO: 为 age 创建信号 (signal(String::new()))
    // TODO: 为 city 创建信号 (signal(String::new()))

    let submit_handler = move |ev: leptos::html::FormEvent| {
        ev.prevent_default();
        leptos::logging::log!("姓名: {}", name.get());
        leptos::logging::log!("邮箱: {}", email.get());
        // TODO: 打印 age 和 city
        // TODO: 清空所有字段（使用 set_xxx.set(String::new())）
    };

    view! {
        <div style="padding: 1rem;">
            <h2>"复杂表单"</h2>
            <form on:submit=submit_handler>
                <div>
                    <label>"姓名:"</label>
                    <input type="text"
                        prop:value={name.get()}
                        on:input=move |ev| set_name.set(event_target_value(&ev)) />
                </div>
                <div>
                    <label>"邮箱:"</label>
                    <input type="email"
                        prop:value={email.get()}
                        on:input=move |ev| set_email.set(event_target_value(&ev)) />
                </div>
                // TODO: 添加年龄输入框 (type="number")
                // TODO: 添加城市输入框 (type="text")
                <button type="submit">"提交"</button>
            </form>
            <hr />
            <h3>"信息预览"</h3>
            <p>"姓名: " {name.get()}</p>
            <p>"邮箱: " {email.get()}</p>
            // TODO: 显示 age 和 city
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
