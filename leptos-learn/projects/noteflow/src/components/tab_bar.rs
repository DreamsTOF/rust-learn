use crate::hooks::use_tabs::TabManager;
use crate::state::AppState;
use leptos::prelude::*;
use leptos_router::hooks::use_navigate;

#[component]
pub fn TabBar() -> impl IntoView {
    let state = use_context::<AppState>().expect("AppState not provided");
    let navigate = use_navigate();

    view! {
        <div class="tab-bar">
            <For
                each=move || state.open_tabs.get()
                key=|tab| tab.doc_id.clone()
                children=move |tab| {
                    let state1 = state.clone();
                    let state2 = state.clone();
                    let navigate1 = navigate.clone();
                    let tab_id = tab.doc_id.clone();
                    let tab_doc_id = tab.doc_id.clone();

                    let is_active = move || {
                        state1.active_tab_id.get().as_deref() == Some(&tab_id)
                    };

                    // TODO: 练习 - 实现标签点击激活
                    // 提示: 使用 TabManager::set_active_tab 设置当前标签，然后导航到对应文档路径
                    let select_tab = move |_| {
                        TabManager::set_active_tab(&state2, &tab_doc_id);
                        let _ = navigate1(&format!("/doc/{}", tab_doc_id), Default::default());
                    };

                    // TODO: 练习 - 实现标签关闭
                    // 提示: 使用 TabManager::close_tab 关闭标签，注意阻止事件冒泡
                    let state3 = state.clone();
                    let close_doc_id = tab.doc_id.clone();
                    let close_tab = move |ev: leptos::ev::MouseEvent| {
                        ev.stop_propagation();
                        TabManager::close_tab(&state3, &close_doc_id);
                    };

                    view! {
                        <div
                            class={move || {
                                let mut cls = String::from("tab");
                                if is_active() { cls.push_str(" active"); }
                                if tab.is_dirty { cls.push_str(" dirty"); }
                                cls
                            }}
                            on:click=select_tab
                        >
                            <span class="tab-title">{tab.title.clone()}</span>
                            {move || if tab.is_dirty {
                                view! { <span class="tab-dirty">"●"</span> }.into_any()
                            } else {
                                view! { <span></span> }.into_any()
                            }}
                            <button
                                class="tab-close"
                                on:click=close_tab
                            >"×"</button>
                        </div>
                    }
                }
            />
        </div>
    }
}
