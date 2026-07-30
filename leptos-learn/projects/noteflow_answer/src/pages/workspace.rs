use crate::state::AppState;
use crate::types::Workspace;
use leptos::prelude::*;
use uuid::Uuid;

#[component]
pub fn WorkspacePage() -> impl IntoView {
    let state = use_context::<AppState>().expect("AppState not provided");
    let workspaces = RwSignal::new(Vec::<Workspace>::new());
    let new_name = RwSignal::new(String::new());

    let create_workspace = move |_: leptos::ev::MouseEvent| {
        let name = new_name.get_untracked();
        if name.is_empty() {
            return;
        }
        let ws = Workspace {
            id: Uuid::new_v4().to_string(),
            name,
            owner_id: state.current_user.get_untracked().map(|u| u.id).unwrap_or_default(),
            member_ids: Vec::new(),
            created_at: chrono::Utc::now().timestamp(),
        };
        workspaces.update(|w| w.push(ws));
        new_name.set(String::new());
    };

    let create_on_enter = move |ev: leptos::ev::KeyboardEvent| {
        if ev.key() == "Enter" {
            let name = new_name.get_untracked();
            if name.is_empty() {
                return;
            }
            let ws = Workspace {
                id: Uuid::new_v4().to_string(),
                name,
                owner_id: state.current_user.get_untracked().map(|u| u.id).unwrap_or_default(),
                member_ids: Vec::new(),
                created_at: chrono::Utc::now().timestamp(),
            };
            workspaces.update(|w| w.push(ws));
            new_name.set(String::new());
        }
    };

    view! {
        <div class="workspace-page">
            <h1>"工作区管理"</h1>
            <div class="create-workspace">
                <input
                    type="text"
                    placeholder="工作区名称..."
                    prop:value=new_name
                    on:input=move |ev| { new_name.set(event_target_value(&ev)); }
                    on:keydown=create_on_enter
                />
                <button on:click=create_workspace>"创建工作区"</button>
            </div>
            <div class="workspace-list">
                <For
                    each=move || workspaces.get()
                    key=|ws| ws.id.clone()
                    children=move |ws: Workspace| {
                        view! {
                            <div class="workspace-card">
                                <div class="workspace-info">
                                    <h3>{ws.name.clone()}</h3>
                                    <span class="workspace-meta">
                                        {ws.member_ids.len()} " 位成员"
                                    </span>
                                </div>
                            </div>
                        }
                    }
                />
            </div>
        </div>
    }
}
