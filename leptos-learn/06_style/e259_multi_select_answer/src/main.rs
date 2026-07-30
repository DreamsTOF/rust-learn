use leptos::prelude::*;
use leptos::wasm_bindgen::JsCast;

#[component]
fn Exercise() -> impl IntoView {
    let languages = vec!["HTML", "CSS", "JavaScript", "Rust", "Python"];
    let selected = RwSignal::new(Vec::<String>::new());

    view! {
        <div>
            <h2>"练习 259 — 多选列表"</h2>
            <p>"按住 Ctrl 键可多选："</p>
            <select multiple on:change=move|ev| {
                let select = event_target::<leptos::web_sys::HtmlSelectElement>(&ev);
                let mut values = Vec::new();
                let opts = select.selected_options();
                for i in 0..opts.length() {
                    if let Some(opt) = opts.item(i) {
                        if let Ok(option) = opt.dyn_into::<leptos::web_sys::HtmlOptionElement>() {
                            values.push(option.value());
                        }
                    }
                }
                selected.set(values);
            }>
                {languages.into_iter().map(|lang| {
                    view! { <option value={lang}>{lang}</option> }
                }).collect::<Vec<_>>()}
            </select>
            <div>
                <p>"已选中："</p>
                <ul>
                    {move || selected.get().iter().map(|item| {
                        view! { <li>{item.clone()}</li> }
                    }).collect::<Vec<_>>()}
                </ul>
            </div>
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
