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
    // TODO: 练习 - 挂载 App 组件到 body
    // 提示: 使用 mount_to_body 函数，传入 app::App 作为参数
    mount_to_body(app::App);
}
