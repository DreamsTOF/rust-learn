use leptos::prelude::*;

#[component]
fn Exercise() -> impl IntoView {
    let (count, set_count) = signal(0);

    on_cleanup(|| {
        leptos::logging::log!("e73: 组件作用域已清理");
    });

    let effect = Effect::new(move || {
        let _ = count.get();
        on_cleanup(|| {
            leptos::logging::log!("e73: effect 已停止");
        });
    });

    set_count.set(1);
    effect.stop();

    view! {
        <p>"on_cleanup 在当前 Owner 清理时执行——包括组件销毁和 Effect 停止。"</p>
    }
}

fn main() {
    mount_to_body(Exercise);
}
