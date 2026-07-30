// ============================================================
// Exercise 20 - Answer: Builder Pattern Advanced
// ============================================================

use leptos::html::{button, div, h2, p};
use leptos::{ev, prelude::*};

#[component]
fn Exercise() -> impl IntoView {
    let (count, set_count) = signal(0);

    div()
        .child(h2().child("构建器模式高级"))
        .child(p().child(format!("计数: {}", count())))
        .child(
            button()
                .child("增加")
                .on(ev::click, move |_| {
                    set_count(count() + 1);
                }),
        )
        .child(
            button()
                .child("重置")
                .attr("style", "margin-left: 8px;")
                .on(ev::click, move |_| {
                    set_count(0);
                }),
        )
}

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(Exercise);
}
