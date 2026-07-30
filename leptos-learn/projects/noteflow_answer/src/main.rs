use leptos::prelude::*;

mod app;
mod components;
mod error;
mod hooks;
mod pages;
mod state;
mod types;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(app::App);
}
