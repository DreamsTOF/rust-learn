// ============================================================
// Exercise 187 - Action Form
// ============================================================

use leptos::prelude::*;

#[component]
fn Exercise() -> impl IntoView {
    let (name, set_name) = signal(String::new());
    let (rating, set_rating) = signal(String::from("5"));
    let (comment, set_comment) = signal(String::new());

    let action = Action::new(|input: &(String, String, String)| {
        let (name, rating, comment) = input.clone();
        async move {
            format!("感谢 {} 的反馈！\n评分: {} 星\n留言: {}", name, rating, comment)
        }
    });

    view! {
        <div>
            <p>"练习 187 — Action + 表单 (action_form)"</p>
            <form on:submit=move |ev| {
                ev.prevent_default();
                ev.stop_propagation();
                action.dispatch((name.get(), rating.get(), comment.get()));
            }>
                <div>
                    <label>"姓名: "
                        <input type="text" name="name"
                            prop:value=move || name.get()
                            on:input=move |ev| set_name(event_target_value(&ev))
                        />
                    </label>
                </div>
                <div>
                    <label>"评分: "
                        <select name="rating"
                            prop:value=move || rating.get()
                            on:change=move |ev| set_rating(event_target_value(&ev))
                        >
                            <option value="5">"⭐⭐⭐⭐⭐"</option>
                            <option value="4">"⭐⭐⭐⭐"</option>
                            <option value="3">"⭐⭐⭐"</option>
                            <option value="2">"⭐⭐"</option>
                            <option value="1">"⭐"</option>
                        </select>
                    </label>
                </div>
                <div>
                    <label>"留言: "
                        <textarea name="comment"
                            prop:value=move || comment.get()
                            on:input=move |ev| set_comment(event_target_value(&ev))
                        ></textarea>
                    </label>
                </div>
                <button type="submit" disabled=move || action.pending().get()>
                    {move || if action.pending().get() { "提交中..." } else { "提交反馈" }}
                </button>
            </form>
            <hr />
            <div>
                <h3>"提交结果:"</h3>
                {move || match action.value().get() {
                    None => view! { <p>"还没有提交反馈。"</p> }.into_any(),
                    Some(result) => view! { <pre>{result}</pre> }.into_any(),
                }}
            </div>
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
