// ============================================================
// Exercise 18 - Answer: Fragment Multi-Root
// ============================================================

use leptos::prelude::*;

#[component]
fn Exercise() -> impl IntoView {
    view! {
        <>
            <h2>"方式一：基础 Fragment"</h2>
            <p>"Fragment 不会在 DOM 中产生额外节点"</p>
            <p>"这是第二个根节点"</p>
        </>
        <>
            <h2>"方式二：嵌套 Fragment"</h2>
            <>
                <p>"这是嵌套 Fragment 中的内容"</p>
                <p>"Fragment 可以任意嵌套，不影响 DOM 结构"</p>
            </>
        </>
        <>
            <h2>"方式三：Fragment 的灵活性"</h2>
            <></>
        </>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(Exercise);
}
