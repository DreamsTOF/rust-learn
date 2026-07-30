// ============================================================
// 练习 e136: document_window_access - 答案
// ============================================================

use leptos::prelude::*;

#[component]
fn DocumentWindowAccess() -> impl IntoView {
    let doc = document();
    let win = window();
    let title = doc.title();
    let href = win.location().href().unwrap_or_default();
    let width = win.inner_width().ok().and_then(|w| w.as_f64()).unwrap_or(0.0);
    let height = win.inner_height().ok().and_then(|h| h.as_f64()).unwrap_or(0.0);

    view! {
        <div>
            <h2>"练习 e136: Document / Window 访问"</h2>
            <p>"标题: " {title}</p>
            <p>"URL: " {href}</p>
            <p>"视口: " {format!("{width:.0} × {height:.0}")}</p>
        </div>
    }
}

fn main() {
    mount_to_body(DocumentWindowAccess);
}
