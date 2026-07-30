use leptos::prelude::*;

// TODO: Show Docker multi-stage build configuration with formatted text
//
// Core: Docker multi-stage build, Rust compilation, cargo-leptos build
//
// In a real project, the Dockerfile lives at the project root.
// Here we display a reference Dockerfile as formatted text in the view.
//
// Hints:
//   1. Define #[component] fn Exercise() -> impl IntoView
//   2. Use <pre> with a raw string literal (r#"..."#) to display the Dockerfile
//   3. The Dockerfile should show multi-stage: build stage + runtime stage
//   4. Show the Dockerfile content in a <pre><code> block for readability

#[component]
fn Exercise() -> impl IntoView {
    // TODO: define a &str with the Dockerfile content using r#"..."#
    // let dockerfile = r#"..."#;

    view! {
        <div>
            <h2>"Docker Multi-Stage Build"</h2>
            // TODO: Display Dockerfile in a <pre> block
            <p>"练习 325 (dockerfile_build)"</p>
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
