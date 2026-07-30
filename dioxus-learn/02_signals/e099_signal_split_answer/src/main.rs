use dioxus::prelude::*;
fn App() -> Element {
    rsx! { div { h1 { "Exercise 99" } p { "Answer placeholder" } } }
}
fn main() { dioxus::launch(App); }
