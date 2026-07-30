use crate::state::AppState;
use crate::types::UserInfo;
use leptos::prelude::*;
use uuid::Uuid;

#[component]
pub fn MembersPage() -> impl IntoView {
    let state = use_context::<AppState>().expect("AppState not provided");
    let members = RwSignal::new(Vec::<UserInfo>::new());

    // Add current user as owner if none exist
    if members.get_untracked().is_empty() {
        if let Some(user) = state.current_user.get_untracked() {
            members.set(vec![UserInfo {
                role: String::from("owner"),
                ..user
            }]);
        } else {
            members.set(vec![
                UserInfo {
                    id: Uuid::new_v4().to_string(),
                    username: "admin".to_string(),
                    email: "admin@noteflow.local".to_string(),
                    avatar: None,
                    role: "owner".to_string(),
                },
            ]);
        }
    }

    // TODO: 练习 - 实现角色修改
    // 提示: 根据 member_id 找到对应成员，更新其 role 字段
    let change_role = move |member_id: String, new_role: String| {
        members.update(|m| {
            if let Some(member) = m.iter_mut().find(|m| m.id == member_id) {
                member.role = new_role;
            }
        });
    };

    // TODO: 练习 - 实现成员移除
    let remove_member = move |member_id: String| {
        members.update(|m| m.retain(|member| member.id != member_id));
    };

    view! {
        <div class="members-page">
            <h1>"成员管理"</h1>
            <table class="members-table">
                <thead>
                    <tr>
                        <th>"用户名"</th>
                        <th>"邮箱"</th>
                        <th>"角色"</th>
                        <th>"操作"</th>
                    </tr>
                </thead>
                <tbody>
                    <For
                        each=move || members.get()
                        key=|m| m.id.clone()
                        children=move |member: UserInfo| {
                            let mid1 = member.id.clone();
                            let mid2 = member.id.clone();
                            view! {
                                <tr>
                                    <td>{member.username.clone()}</td>
                                    <td>{member.email.clone()}</td>
                                    <td>
                                        <select
                                            prop:value=move || member.role.clone()
                                            on:change=move |ev| {
                                                change_role(mid1.clone(), event_target_value(&ev));
                                            }
                                        >
                                            <option value="owner">"Owner"</option>
                                            <option value="admin">"Admin"</option>
                                            <option value="editor">"Editor"</option>
                                            <option value="viewer">"Viewer"</option>
                                        </select>
                                    </td>
                                    <td>
                                        <button on:click=move |_| remove_member(mid2.clone())>"移除"</button>
                                    </td>
                                </tr>
                            }
                        }
                    />
                </tbody>
            </table>
        </div>
    }
}
