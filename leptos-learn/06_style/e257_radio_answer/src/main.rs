use leptos::prelude::*;

#[component]
fn Exercise() -> impl IntoView {
    let fruits = vec!["苹果", "香蕉", "橙子", "葡萄"];
    let (selected, set_selected) = signal(String::new());

    view! {
        <div>
            <h2>"练习 257 — 单选按钮"</h2>
            <p>"请选择你最喜欢的水果："</p>
            {
                fruits.into_iter().map(|fruit| {
                    let fruit_name = fruit;
                    view! {
                        <label>
                            <input type="radio" name="fruit" value={fruit_name}
                                on:change=move|_| set_selected.set(fruit_name.to_string())
                            />
                            {fruit_name}
                        </label>
                    }
                }).collect::<Vec<_>>()
            }
            <p>"已选择: " {
                move || if selected.get().is_empty() { "(无)".to_string() } else { selected.get() }
            }</p>
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
