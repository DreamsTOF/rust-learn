use leptos::prelude::*;

#[component]
fn Exercise() -> impl IntoView {
    let (outer, set_outer) = signal(0);
    let (inner, set_inner) = signal(0);

    let parent_effect = Effect::new(move || {
        let _o = outer.get();

        Effect::new(move || {
            let _i = inner.get();
            on_cleanup(|| {
                leptos::logging::log!("e74: 子 Effect 已清理");
            });
        });

        on_cleanup(|| {
            leptos::logging::log!("e74: 父 Effect 已清理");
        });
    });

    set_inner.set(1);
    set_outer.set(1);
    parent_effect.stop();

    view! {
        <p>"Effect 嵌套形成父子 Scope 层级：父停止则子自动清理。"</p>
    }
}

fn main() {
    mount_to_body(Exercise);
}
