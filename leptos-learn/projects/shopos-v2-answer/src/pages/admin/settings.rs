use leptos::prelude::*;
use leptos::task::spawn_local;
use thaw::*;

use crate::server::settings::{get_all_settings, update_setting};
use crate::Setting;

#[component]
pub fn SettingsPage() -> impl IntoView {
    let settings = Resource::new(|| (), |_| async { get_all_settings().await });
    let edit_values = RwSignal::new(std::collections::HashMap::<String, String>::new());
    let msg = RwSignal::new(String::new());
    let saving = RwSignal::new(false);

    let save_all = move |_| {
        saving.set(true);
        msg.set(String::new());
        let values = edit_values.get();
        let keys: Vec<String> = values.keys().cloned().collect();

        spawn_local(async move {
            let mut errs = Vec::new();
            for key in keys {
                let val = values.get(&key).cloned().unwrap_or_default();
                if let Err(e) = update_setting(key.clone(), val).await {
                    errs.push(format!("{}: {}", key, e));
                }
            }
            saving.set(false);
            if errs.is_empty() {
                msg.set("所有设置已保存".into());
            } else {
                msg.set(format!("部分保存失败: {}", errs.join("; ")));
            }
        });
    };

    view! {
        <div class="page-container">
            <div style="margin-bottom: 16px;"><Space align=SpaceAlign::Center>
                <h2>"系统设置"</h2>
                <Button
                    appearance=ButtonAppearance::Primary
                    on_click=save_all
                    disabled=move || saving.get()
                >
                    {move || if saving.get() { "保存中..." } else { "保存全部" }}
                </Button>
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
                {move || settings.get().map(|result| {
                    match result {
                        Ok(data) => {
                            let map: std::collections::HashMap<String, String> = data.iter().map(|s| (s.key.clone(), s.value.clone())).collect();
                            edit_values.set(map);
                            let items = data.clone();
                            view! {
                                <table class="thaw-table">
                                    <thead>
                                        <tr>
                                            <th>"配置项"</th>
                                            <th>"说明"</th>
                                            <th>"值"</th>
                                        </tr>
                                    </thead>
                                    <tbody>
                                        {items.into_iter().map(|s| {
                                            let key = s.key.clone();
                                            let label = s.key.clone();
                                            let desc = s.description.clone().unwrap_or_default();
                                            view! {
                                                <tr>
                                                    <td><code>{label}</code></td>
                                                    <td>{desc}</td>
                                                    <td>
                                                        <input
                                                            type="text"
                                                            prop:value={ let k = key.clone(); move || edit_values.get().get(&k).cloned().unwrap_or_default() }
                                                            on:input=move |evt| { edit_values.update(|m| { m.insert(key.clone(), event_target_value(&evt)); }); }
                                                        />
                                                    </td>
                                                </tr>
                                            }
                                        }).collect_view()}
                                    </tbody>
                                </table>
                            }.into_any()
                        }
                        Err(e) => view! { <Text>"加载失败: " {e.to_string()}</Text> }.into_any(),
                    }
                })}
            </Suspense>
        </div>
    }
}
