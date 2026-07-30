use leptos::prelude::*;
use leptos::web_sys;

#[component]
fn Exercise() -> impl IntoView {
    let (name, set_name) = signal(String::new());
    let (email, set_email) = signal(String::new());
    let (age, set_age) = signal(String::new());
    let (city, set_city) = signal(String::new());

    let submit_handler = move |ev: web_sys::SubmitEvent| {
        ev.prevent_default();
        leptos::logging::log!("姓名: {}", name.get());
        leptos::logging::log!("邮箱: {}", email.get());
        leptos::logging::log!("年龄: {}", age.get());
        leptos::logging::log!("城市: {}", city.get());
        set_name.set(String::new());
        set_email.set(String::new());
        set_age.set(String::new());
        set_city.set(String::new());
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
                <div>
                    <label>"年龄:"</label>
                    <input type="number"
                        prop:value={age.get()}
                        on:input=move |ev| set_age.set(event_target_value(&ev)) />
                </div>
                <div>
                    <label>"城市:"</label>
                    <input type="text"
                        prop:value={city.get()}
                        on:input=move |ev| set_city.set(event_target_value(&ev)) />
                </div>
                <button type="submit">"提交"</button>
            </form>
            <hr />
            <h3>"信息预览"</h3>
            <p>"姓名: " {name.get()}</p>
            <p>"邮箱: " {email.get()}</p>
            <p>"年龄: " {age.get()}</p>
            <p>"城市: " {city.get()}</p>
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
