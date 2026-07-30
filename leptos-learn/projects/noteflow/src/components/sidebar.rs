use crate::state::AppState;
use crate::types::DocNode;
use leptos::prelude::*;

#[component]
pub fn Sidebar() -> impl IntoView {
    let state = use_context::<AppState>().expect("AppState not provided");
    let new_doc_name = RwSignal::new(String::new());
    let show_new_input = RwSignal::new(false);

    // TODO: 练习 - 实现文档创建函数
    // 提示: 获取 new_doc_name 的值，使用 DocNode::new() 创建文档，
    //       然后更新 state.docs，最后清空输入框并隐藏输入区域
    let create_doc = move |is_folder: bool| {
        let name = new_doc_name.get_untracked();
        if name.is_empty() {
            return;
        }
        let doc = DocNode::new(name, is_folder, None);
        state.docs.update(|docs| docs.push(doc));
        new_doc_name.set(String::new());
        show_new_input.set(false);
    };

    // TODO: 练习 - 实现文档删除函数
    // 提示: 根据文档 id 从 state.docs 中移除对应文档
    let delete_doc = move |id: String| {
        state.docs.update(|docs| {
            docs.retain(|d| d.id != id);
        });
    };

    view! {
        <aside class="sidebar">
            <div class="sidebar-header">
                <h2 class="workspace-name">"NoteFlow"</h2>
                <div class="sidebar-actions">
                    <button class="icon-btn" title="新建文档" on:click=move |_| { show_new_input.set(true); }>
                        "+"
                    </button>
                </div>
            </div>

            {move || show_new_input.get().then(|| {
                view! {
                    <div class="new-doc-input">
                        <input
                            type="text"
                            placeholder="输入名称..."
                            prop:value=new_doc_name
                            on:input=move |ev| { new_doc_name.set(event_target_value(&ev)); }
                            on:keydown=move |ev: leptos::ev::KeyboardEvent| {
                                if ev.key() == "Enter" { create_doc(false); }
                                if ev.key() == "Escape" { show_new_input.set(false); }
                            }
                        />
                        <button on:click=move |_| create_doc(false)>"文档"</button>
                        <button on:click=move |_| create_doc(true)>"文件夹"</button>
                    </div>
                }
            })}

            <nav class="doc-tree">
                <For
                    each=move || state.docs.get()
                    key=|doc| doc.id.clone()
                    children=move |doc| {
                        let doc_id = doc.id.clone();
                        let doc_title = doc.title.clone();
                        let doc_is_folder = doc.is_folder;
                        let doc_children = doc.children.clone();
                        view! {
                            <DocTreeNode
                                id=doc_id
                                title=doc_title
                                is_folder=doc_is_folder
                                children=doc_children
                                delete_doc=delete_doc
                            />
                        }
                    }
                />
            </nav>
        </aside>
    }
}

#[component]
fn DocTreeNode(
    id: String,
    title: String,
    is_folder: bool,
    children: Vec<DocNode>,
    #[prop(into)]
    delete_doc: Callback<String>,
) -> impl IntoView {
    let state = use_context::<AppState>().expect("AppState not provided");
    let is_expanded = RwSignal::new(true);
    let is_editing = RwSignal::new(false);
    let edit_title = RwSignal::new(title.clone());

    let id0 = id.clone();
    let id1 = id.clone();

    // TODO: 练习 - 实现文档选择
    // 提示: 点击文档节点时，设置 state.selected_doc_id 和 state.active_tab_id
    let select_doc = move |_| {
        state.selected_doc_id.set(Some(id0.clone()));
        state.active_tab_id.set(Some(id0.clone()));
    };

    let toggle_expand = move |_| {
        is_expanded.update(|v| *v = !*v);
    };

    let title0 = title.clone();
    let title1 = title.clone();

    let start_edit = move |_| {
        edit_title.set(title0.clone());
        is_editing.set(true);
    };

    let save_edit = {
        let is_editing = is_editing.clone();
        let edit_title = edit_title.clone();
        let state = state.clone();
        move || {
            let new_title = edit_title.get_untracked();
            state.docs.update(|docs| {
                for d in docs.iter_mut() {
                    if d.id == id1 {
                        d.title = new_title.clone();
                        d.updated_at = chrono::Utc::now().timestamp();
                    }
                }
            });
            is_editing.set(false);
        }
    };

    let show_input = move || {
        let editing = is_editing.get();
        if editing {
            let save = save_edit.clone();
            let save2 = save_edit.clone();
            view! {
                <input
                    type="text"
                    prop:value=edit_title
                    on:input=move |ev| { edit_title.set(event_target_value(&ev)); }
                    on:keydown=move |ev: leptos::ev::KeyboardEvent| {
                        if ev.key() == "Enter" { save(); }
                        if ev.key() == "Escape" { is_editing.set(false); }
                    }
                    on:blur=move |_| save2()
                    autofocus=true
                />
            }.into_any()
        } else {
            view! {
                <span class="doc-title">{title1.clone()}</span>
            }.into_any()
        }
    };

    let show_children = move || {
        let expanded = is_expanded.get();
        if is_folder && expanded {
            let child_list = children.clone();
            view! {
                <div class="doc-tree-children">
                    <For
                        each=move || child_list.clone()
                        key=|child| child.id.clone()
                        children=move |child| {
                            let child_id = child.id.clone();
                            let child_title = child.title.clone();
                            let child_is_folder = child.is_folder;
                            let child_children = child.children.clone();
                            view! {
                                <DocTreeNode
                                    id=child_id
                                    title=child_title
                                    is_folder=child_is_folder
                                    children=child_children
                                    delete_doc=delete_doc
                                />
                            }
                        }
                    />
                </div>
            }.into_any()
        } else {
            view! { <div></div> }.into_any()
        }
    };

    view! {
        <div class="doc-tree-node" draggable="true">
            <div class="doc-tree-item" on:click=select_doc>
                {move || if is_folder {
                    view! {
                        <span class="expand-icon" on:click=toggle_expand>
                            {move || if is_expanded.get() { "▼" } else { "▶" }}
                        </span>
                    }.into_any()
                } else {
                    view! { <span class="doc-icon">"📄"</span> }.into_any()
                }}

                {show_input}

                <span class="doc-context-menu">
                    <button class="icon-btn small" title="重命名" on:click=start_edit>"✏️"</button>
                    <button class="icon-btn small" title="删除" on:click=move |_| { delete_doc.run(id.clone()); }>"🗑️"</button>
                </span>
            </div>

            {show_children}
        </div>
    }
}
