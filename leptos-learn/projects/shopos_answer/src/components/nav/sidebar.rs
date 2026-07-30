use leptos::prelude::*;
use leptos_router::components::A;
use crate::state::{get_sidebar_menu, NavItem};

#[component]
pub fn Sidebar(
    #[prop(into)]
    collapsed: RwSignal<bool>,
) -> impl IntoView {
    let menu_items = get_sidebar_menu();

    view! {
        <div class="sidebar">
            <div class="sidebar-logo">
                <span class="sidebar-logo-text">"ShopOS"</span>
            </div>
            <nav class="sidebar-nav">
                <ul>
                    {menu_items.into_iter().map(|item| {
                        let label = item.label;
                        if let Some(path) = item.path {
                            view! {
                                <li>
                                    <A href=path>{label}</A>
                                </li>
                            }.into_any()
                        } else if let Some(children) = item.children {
                            view! {
                                <li>
                                    <span>{label}</span>
                                    <ul>{children.into_iter().map(|child| {
                                        let cl = child.label;
                                        if let Some(p) = child.path {
                                            view! { <li><A href=p>{cl}</A></li> }.into_any()
                                        } else {
                                            view! { <li><span>{cl}</span></li> }.into_any()
                                        }
                                    }).collect::<Vec<_>>()}</ul>
                                </li>
                            }.into_any()
                        } else {
                            view! {
                                <li>
                                    <span>{label}</span>
                                </li>
                            }.into_any()
                        }
                    }).collect::<Vec<_>>()}
                </ul>
            </nav>
        </div>
    }
}
