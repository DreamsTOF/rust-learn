use leptos::prelude::*;
use leptos::task::spawn_local;
use thaw::*;

use crate::server::categories::{
    create_category, delete_category, get_category_tree, update_category,
};
use crate::Category;

#[component]
pub fn CategoriesPage() -> impl IntoView {
    let categories = Resource::new(|| (), |_| async { get_category_tree().await });
    let refresh = RwSignal::new(0i32);
    let show_modal = RwSignal::new(false);
    let editing_name = RwSignal::new(String::new());
    let msg = RwSignal::new(String::new());

    let reload = move || refresh.update(|v| *v += 1);

    let open_create = move |_| {
        editing_name.set(String::new());
        msg.set(String::new());
        show_modal.set(true);
    };

    let do_save = move |_| {
        let name = editing_name.get();
        if name.is_empty() {
            msg.set("名称不能为空".into());
            return;
        }
        msg.set(String::new());
        spawn_local(async move {
            match create_category(name, None).await {
                Ok(_) => {
                    show_modal.set(false);
                    reload();
                }
                Err(e) => msg.set(format!("创建失败: {}", e)),
            }
        });
    };

    view! {
        <div class="page-container">
            <div style="margin-bottom: 16px;"><Space align=SpaceAlign::Center>
                <h2>"商品分类"</h2>
                <Button appearance=ButtonAppearance::Primary on_click=open_create>"新增分类"</Button>
            </Space>
        </div>

            {move || {
                let m = msg.get();
                if !m.is_empty() {
                    view! { <Text>{m}</Text> }.into_any()
                } else {
                    view! { <span></span> }.into_any()
                }
            }}

            <Suspense fallback=move || view! { <Text>"加载中..."</Text> }>
                {move || refresh.get(); categories.get().map(|result| {
                    match result {
                        Ok(data) => view! {
                            <Card>
                                <ul>
                                    {data.into_iter().map(|cat| {
                                        let name = cat.name;
                                        let children = cat.children;
                                        if children.is_empty() {
                                            view! {
                                                <li><span>{name}</span></li>
                                            }.into_any()
                                        } else {
                                            view! {
                                                <li>
                                                    <span style="font-weight: bold;">{name}</span>
                                                    <ul>{children.into_iter().map(|child| {
                                                        let cn = child.name;
                                                        view! { <li><span>{cn}</span></li> }
                                                    }).collect_view()}</ul>
                                                </li>
                                            }.into_any()
                                        }
                                    }).collect_view()}
                                </ul>
                            </Card>
                        }.into_any(),
                        Err(e) => view! { <Text>"加载失败: " {e.to_string()}</Text> }.into_any(),
                    }
                })}
            </Suspense>

            <Dialog open=show_modal>
                <DialogTitle>"新增分类"</DialogTitle>
                <Space vertical=true>
                    <p>"分类名称"</p>
                    <input
                        type="text"
                        prop:value=move || editing_name.get()
                        on:input=move |ev| editing_name.set(event_target_value(&ev))
                    />
                    <Button appearance=ButtonAppearance::Primary on_click=do_save>"保存"</Button>
                </Space>
            </Dialog>
        </div>
    }
}
