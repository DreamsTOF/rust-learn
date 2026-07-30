use leptos::prelude::*;
use web_sys::window;

mod exercises;

fn get_exercise_param() -> String {
    let search = window().and_then(|w| w.location().search().ok()).unwrap_or_default();
    let search = search.trim_start_matches('?');
    for pair in search.split('&') {
        let mut parts = pair.splitn(2, '=');
        if parts.next().unwrap_or("") == "e" {
            return parts.next().unwrap_or("").to_string();
        }
    }
    String::new()
}

#[component]
fn App() -> impl IntoView {
    let key = get_exercise_param();
    match key.as_str() {
        "01" => view! { <exercises::e01_hello_world::Exercise/> }.into_any(),
        "01_answer" => view! { <exercises::e01_hello_world_answer::Exercise/> }.into_any(),
        _ => view! { <p>"请指定练习参数 ?e=01"</p> }.into_any(),
    }
}

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App);
}
