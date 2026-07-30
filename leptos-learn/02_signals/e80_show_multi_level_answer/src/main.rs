use leptos::prelude::*;

#[component]
fn Exercise() -> impl IntoView {
    let (level1, set_level1) = signal(false);
    let (level2, set_level2) = signal(false);
    let (level3, set_level3) = signal(false);

    view! {
        <div style="padding: 1rem;">
            <h3>"多级嵌套菜单"</h3>

            <button on:click=move |_| set_level1.set(!level1.get())>
                {move || if level1.get() { "▾ 收起" } else { "▸ 展开" }} " 一级"
            </button>

            <Show when=move || level1.get()>
                <div style="padding: 8px 16px; border: 1px solid #ccc; margin-top: 4px;">
                    <p>"📁 一级内容"</p>

                    <button on:click=move |_| set_level2.set(!level2.get())>
                        {move || if level2.get() { "▾ 收起" } else { "▸ 展开" }} " 二级"
                    </button>

                    <Show when=move || level2.get()>
                        <div style="padding: 8px 16px; border: 1px solid #aaa; margin-top: 4px; margin-left: 16px;">
                            <p>"📂 二级内容"</p>

                            <button on:click=move |_| set_level3.set(!level3.get())>
                                {move || if level3.get() { "▾ 收起" } else { "▸ 展开" }} " 三级"
                            </button>

                            <Show when=move || level3.get()>
                                <div style="padding: 8px 16px; border: 1px solid #888; margin-top: 4px; margin-left: 16px; background: #f5f5f5;">
                                    <p>"📄 三级内容（最深层）"</p>
                                </div>
                            </Show>
                        </div>
                    </Show>
                </div>
            </Show>
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
