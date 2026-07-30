// ============================================================
// Exercise 121 - Answer: Context Override
// ============================================================

use leptos::prelude::*;

#[derive(Clone, Debug, PartialEq)]
enum Theme {
    Light,
    Dark,
}

#[component]
fn ThemeDisplay() -> impl IntoView {
    let theme = use_context::<Theme>()
        .expect("Theme should be provided by an ancestor");

    view! {
        <p>"Current theme: " {format!("{:?}", theme)}</p>
    }
}

#[component]
fn InnerSection() -> impl IntoView {
    provide_context(Theme::Dark);

    view! {
        <div style="border: 1px solid blue; padding: 8px; margin: 8px 0;">
            <h3>"Inner Section"</h3>
            <p>"内层提供 Theme::Dark"</p>
            <ThemeDisplay/>
        </div>
    }
}

#[component]
fn Exercise() -> impl IntoView {
    provide_context(Theme::Light);

    view! {
        <div style="border: 1px solid gray; padding: 8px;">
            <h2>"Context Override Demo"</h2>
            <p>"外层提供 Light，内层提供 Dark，内层覆盖外层"</p>
            <ThemeDisplay/>
            <InnerSection/>
            <p>"回到外层后:" <ThemeDisplay/></p>
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
