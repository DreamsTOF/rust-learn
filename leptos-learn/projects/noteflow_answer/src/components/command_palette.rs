use crate::hooks::use_theme::ThemeManager;
use crate::state::AppState;
use crate::types::DocNode;
use leptos::prelude::*;

const COMMANDS: &[(&str, &str, Option<&str>)] = &[
    ("new-doc", "新建文档", Some("Ctrl+N")),
    ("new-folder", "新建文件夹", Some("Ctrl+Shift+N")),
    ("toggle-theme", "切换暗黑模式", Some("Ctrl+Shift+T")),
    ("search", "搜索", Some("Ctrl+K")),
    ("export", "导出文档", None),
];

#[component]
pub fn CommandPalette() -> impl IntoView {
    let state = use_context::<AppState>().expect("AppState not provided");
    let is_open = RwSignal::new(false);
    let query = RwSignal::new(String::new());
    let filtered_ids = RwSignal::new(Vec::<&'static str>::new());

    Effect::new(move |_| {
        let q = query.get().to_lowercase();
        if q.is_empty() {
            filtered_ids.set(COMMANDS.iter().map(|c| c.0).collect());
        } else {
            filtered_ids.set(
                COMMANDS
                    .iter()
                    .filter(|c| c.1.to_lowercase().contains(&q))
                    .map(|c| c.0)
                    .collect(),
            );
        }
    });

    let render_commands = move || {
        if !is_open.get() {
            return view! { <div></div> }.into_any();
        }
        let ids = filtered_ids.get();
        let items: Vec<_> = ids
            .iter()
            .map(|id| {
                let cmd = COMMANDS.iter().find(|c| c.0 == *id).unwrap();
                let state = state.clone();
                let is_open = is_open.clone();
                let query = query.clone();
                let cmd_id = cmd.0;
                let cmd_name = cmd.1;
                let cmd_shortcut = cmd.2;
                view! {
                    <div
                        class="command-palette-item"
                        on:mousedown=move |_| {
                            match cmd_id {
                                "new-doc" => {
                                    state.docs.update(|docs| {
                                        docs.push(DocNode::new(
                                            format!("新文档 {}", docs.len() + 1),
                                            false,
                                            None,
                                        ));
                                    });
                                }
                                "new-folder" => {
                                    state.docs.update(|docs| {
                                        docs.push(DocNode::new(
                                            format!("新文件夹 {}", docs.len() + 1),
                                            true,
                                            None,
                                        ));
                                    });
                                }
                                "toggle-theme" => {
                                    ThemeManager::toggle(&state);
                                }
                                _ => {}
                            }
                            is_open.set(false);
                            query.set(String::new());
                        }
                    >
                        <span class="command-name">{cmd_name}</span>
                        {cmd_shortcut.map(|s| {
                            view! { <span class="command-shortcut">{s}</span> }.into_any()
                        }).unwrap_or_else(|| view! { <span></span> }.into_any())}
                    </div>
                }
            })
            .collect();

        view! {
            <div class="command-palette-overlay" on:click=move |_| is_open.set(false)>
                <div class="command-palette" on:click=move |ev| ev.stop_propagation()>
                    <div class="command-palette-header">
                        <input
                            type="text"
                            class="command-palette-input"
                            placeholder="输入命令名称..."
                            prop:value=query
                            on:input=move |ev| { query.set(event_target_value(&ev)); }
                            on:keydown=move |ev: leptos::ev::KeyboardEvent| {
                                if ev.key() == "Escape" { is_open.set(false); }
                            }
                            autofocus=true
                        />
                    </div>
                    <div class="command-palette-list">
                        {items}
                    </div>
                </div>
            </div>
        }.into_any()
    };

    render_commands
}
