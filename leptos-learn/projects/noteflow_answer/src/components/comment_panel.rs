use crate::state::AppState;
use crate::types::Comment;
use leptos::prelude::*;
use uuid::Uuid;

#[component]
pub fn CommentPanel() -> impl IntoView {
    let state = use_context::<AppState>().expect("AppState not provided");
    let comments = RwSignal::new(Vec::<Comment>::new());
    let new_comment = RwSignal::new(String::new());
    let selected_text = RwSignal::new(String::new());

    let add_comment = move || {
        let content = new_comment.get_untracked();
        if content.is_empty() {
            return;
        }
        let doc_id = state.active_tab_id.get_untracked().unwrap_or_default();
        let user = state.current_user.get_untracked();
        let comment = Comment {
            id: Uuid::new_v4().to_string(),
            doc_id: doc_id.clone(),
            user_id: user.as_ref().map(|u| u.id.clone()).unwrap_or_default(),
            username: user.as_ref().map(|u| u.username.clone()).unwrap_or_default(),
            selected_text: selected_text.get_untracked(),
            content,
            resolved: false,
            created_at: chrono::Utc::now().timestamp(),
        };
        comments.update(|c| c.push(comment));
        new_comment.set(String::new());
    };

    let toggle_resolved = move |comment_id: String| {
        comments.update(|c| {
            if let Some(comment) = c.iter_mut().find(|c| c.id == comment_id) {
                comment.resolved = !comment.resolved;
            }
        });
    };

    view! {
        <div class="comment-panel">
            <h4>"评论"</h4>
            <div class="new-comment">
                {move || {
                    let text = selected_text.get();
                    if !text.is_empty() {
                        view! {
                            <div class="selected-text">
                                "引用: " {text}
                            </div>
                        }.into_any()
                    } else {
                        view! { <div></div> }.into_any()
                    }
                }}
                <textarea
                    prop:value=new_comment
                    on:input=move |ev| { new_comment.set(event_target_value(&ev)); }
                    placeholder="添加评论..."
                    rows=3
                ></textarea>
                <button on:click=move |_| add_comment()>"提交"</button>
            </div>
            <div class="comments-list">
                <For
                    each=move || comments.get()
                    key=|c| c.id.clone()
                    children=move |comment: Comment| {
                        view! {
                            <div class={move || {
                                if comment.resolved { "comment-item resolved" } else { "comment-item" }
                            }}>
                                <div class="comment-header">
                                    <strong>{comment.username.clone()}</strong>
                                    <span class="comment-time">
                                        {chrono::DateTime::from_timestamp(comment.created_at, 0)
                                            .map(|dt| dt.format("%m-%d %H:%M").to_string())
                                            .unwrap_or_default()}
                                    </span>
                                </div>
                                {move || {
                                    let text = comment.selected_text.clone();
                                    if !text.is_empty() {
                                        view! {
                                            <div class="comment-quote">"「" {text} "」"</div>
                                        }.into_any()
                                    } else {
                                        view! { <div></div> }.into_any()
                                    }
                                }}
                                <div class="comment-content">{comment.content.clone()}</div>
                                <button on:click=move |_| toggle_resolved(comment.id.clone())>
                                    {if comment.resolved { "重新打开" } else { "解决" }}
                                </button>
                            </div>
                        }
                    }
                />
            </div>
        </div>
    }
}
