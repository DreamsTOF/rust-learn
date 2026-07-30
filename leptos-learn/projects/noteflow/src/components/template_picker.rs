use crate::state::AppState;
use crate::types::DocNode;
use leptos::prelude::*;

#[derive(Clone)]
struct Template {
    name: &'static str,
    icon: &'static str,
    content: &'static str,
}

const TEMPLATES: &[Template] = &[
    Template {
        name: "空白文档",
        icon: "📄",
        content: "",
    },
    Template {
        name: "会议记录",
        icon: "📋",
        content: "# 会议记录\n\n**日期:** {{日期}}\n**参会人:** {{参会人}}\n\n## 议程\n\n1. \n2. \n3. \n\n## 讨论\n\n\n## 行动项\n\n- [ ] \n- [ ] \n",
    },
    Template {
        name: "周报",
        icon: "📊",
        content: "# 周报 - {{日期}}\n\n## 本周完成\n\n1. \n2. \n3. \n\n## 下周计划\n\n1. \n2. \n3. \n\n## 问题与风险\n\n",
    },
    Template {
        name: "需求文档",
        icon: "📝",
        content: "# 需求文档\n\n## 背景\n{{背景}}\n\n## 目标\n{{目标}}\n\n## 功能需求\n\n1. \n2. \n3. \n\n## 非功能需求\n\n",
    },
];

#[component]
pub fn TemplatePicker() -> impl IntoView {
    let state = use_context::<AppState>().expect("AppState not provided");
    let is_open = RwSignal::new(false);

    let apply_template = move |template: &'static Template| {
        // TODO: 练习 - 实现模板变量替换并创建文档
        // 提示: 替换 {{日期}} 等模板变量，使用 DocNode::new 创建文档，
        //       将替换后的内容赋值给文档，然后将文档添加到 state.docs
        let content = template.content.replace("{{日期}}", &chrono::Local::now().format("%Y-%m-%d").to_string());
        let doc = DocNode::new(template.name.to_string(), false, None);
        let id = doc.id.clone();
        state.docs.update(|docs| {
            docs.push(DocNode {
                content,
                ..doc
            });
        });
        state.selected_doc_id.set(Some(id));
        is_open.set(false);
    };

    view! {
        <>
            <button class="icon-btn" title="从模板创建" on:click=move |_| is_open.set(true)>"📋"</button>
            {move || {
                if is_open.get() {
                    view! {
                        <div class="modal-overlay" on:click=move |_| is_open.set(false)>
                            <div class="template-picker modal" on:click=move |ev| ev.stop_propagation()>
                                <h3>"选择模板"</h3>
                                <div class="template-grid">
                                    {TEMPLATES.iter().map(|t| {
                                        let name = t.name;
                                        let icon = t.icon;
                                        view! {
                                            <div
                                                class="template-card"
                                                on:click=move |_| apply_template(t)
                                            >
                                                <span class="template-icon">{icon}</span>
                                                <span class="template-name">{name}</span>
                                            </div>
                                        }
                                    }).collect::<Vec<_>>()}
                                </div>
                                <button class="modal-close" on:click=move |_| is_open.set(false)>"取消"</button>
                            </div>
                        </div>
                    }.into_any()
                } else {
                    view! { <div></div> }.into_any()
                }
            }}
        </>
    }
}
