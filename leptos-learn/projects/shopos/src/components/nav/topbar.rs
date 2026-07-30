use leptos::prelude::*;
use leptos_router::components::A;
use thaw::{Button, ButtonAppearance, LayoutHeader, Space};

#[component]
pub fn Topbar(
    #[prop(into)]
    collapsed: RwSignal<bool>,
) -> impl IntoView {
    let toggle = move |_| {
        collapsed.update(|v| *v = !*v);
    };

    view! {
        <LayoutHeader>
            <div class="topbar">
                <div class="topbar-left">
                    <Button appearance=ButtonAppearance::Subtle on_click=toggle>
                        "☰"
                    </Button>
                </div>
                <div class="topbar-right">
                    <Space>
                        <A href="/cart">
                            <Button appearance=ButtonAppearance::Subtle>
                                "🛒 购物车"
                            </Button>
                        </A>
                        <A href="/login">
                            <Button appearance=ButtonAppearance::Subtle>
                                "👤 登录"
                            </Button>
                        </A>
                    </Space>
                </div>
            </div>
        </LayoutHeader>
    }
}
