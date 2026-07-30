// ============================================================
// Exercise e111: generic_bounds — Answer
// ============================================================

use leptos::prelude::*;
use std::fmt::Display;

#[component]
fn DisplayItem<T: Display + 'static>(value: T, label: &'static str) -> impl IntoView {
    view! {
        <p><strong>{label}</strong> ": " {value.to_string()}</p>
    }
}

#[component]
fn Exercise() -> impl IntoView {
    view! {
        <div>
            <h3>"泛型 + Trait Bound"</h3>
            <DisplayItem value=42 label="数字" />
            <DisplayItem value="Hello Leptos!" label="字符串" />
            <DisplayItem value=3.14159 label="浮点数" />
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
