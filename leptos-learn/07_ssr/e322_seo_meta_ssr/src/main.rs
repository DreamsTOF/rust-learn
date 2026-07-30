use leptos::prelude::*;

// TODO: Add dynamic <meta> tags for SEO/Open Graph
//
// Core: <Meta/> component, document metadata, Open Graph tags
//
// Hints:
//   1. Define #[component] fn Exercise() -> impl IntoView
//   2. Use <Meta name="..." content="..." /> for standard meta tags
//   3. Use <Meta property="..." content="..." /> for Open Graph (e.g. og:title, og:description)
//   4. Import Meta from leptos::meta (or use in view! macro)
//   5. Also try <Title text="..."/> for the page <title>

#[component]
fn Exercise() -> impl IntoView {
    view! {
        <div>
            <h2>"SEO & Meta Tags (SSR)"</h2>
            // TODO: Add <Meta name="description" content="..." />
            // TODO: Add <Meta property="og:title" content="..." />
            // TODO: Add <Meta property="og:description" content="..." />
            // TODO: Add <Meta property="og:image" content="..." />
            <p>"练习 322 (seo_meta_ssr)"</p>
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
