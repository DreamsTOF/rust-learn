use leptos::prelude::*;
use leptos::portal::Portal;

#[component]
fn Exercise() -> impl IntoView {
    let (show, set_show) = signal(false);

    view! {
        <div style="padding: 1rem; border: 2px dashed #888;">
            <p>"这是普通 DOM 树内的内容"</p>
            <button on:click=move |_| set_show.update(|v| *v = !*v)>
                {move || if show.get() { "关闭 Portal" } else { "打开 Portal" }}
            </button>
        </div>

        {move || show.get().then(|| {
            view! {
                <Portal>
                    <div style="
                        position: fixed; inset: 0; background: rgba(0,0,0,0.4);
                        display: flex; align-items: center; justify-content: center;
                        z-index: 9999;
                    ">
                        <div style="
                            background: white; padding: 2rem; border-radius: 8px;
                            min-width: 300px; text-align: center;
                        ">
                            <h3>"Portal 模态框"</h3>
                            <p>"此内容通过 Portal 渲染在 <body> 下"</p>
                            <p>"查看 DevTools Elements 面板确认"</p>
                            <button on:click=move |_| set_show.set(false)>
                                "关闭"
                            </button>
                        </div>
                    </div>
                </Portal>
            }
        })}
    }
}

fn main() {
    mount_to_body(Exercise);
}
