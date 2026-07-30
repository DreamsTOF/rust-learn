use leptos::prelude::*;

#[component]
fn Exercise() -> impl IntoView {
    let phones = RwSignal::new(vec![String::new()]);

    let add_phone = move |_| {
        phones.update(|p| p.push(String::new()));
    };

    let remove_phone = move |idx: usize| {
        phones.update(|p| {
            if p.len() > 1 {
                p.remove(idx);
            }
        });
    };

    view! {
        <div style="padding: 1rem; max-width: 500px;">
            <h2>"动态表单字段"</h2>
            <p>"电话号码:"</p>
            {move || phones.get().iter().enumerate().map(|(i, val)| {
                let val = val.clone();
                view! {
                    <div style="display: flex; gap: 0.5rem; margin-bottom: 0.5rem; align-items: center;">
                        <input type="tel"
                            prop:value={val}
                            on:input=move |ev| {
                                let v = event_target_value(&ev);
                                phones.update(|p| p[i] = v);
                            }
                            style="flex: 1;" />
                        <button on:click=move |_| remove_phone(i)
                            disabled={move || phones.get().len() <= 1}>
                            "删除"
                        </button>
                        <span>{i + 1}</span>
                    </div>
                }
            }).collect::<Vec<_>>()
            }
            <button on:click=add_phone>"添加电话"</button>
            <hr />
            <h3>"已录入的电话"</h3>
            <ul>
                {move || phones.get().iter().enumerate().map(|(i, p)| {
                    view! { <li>"电话 " {i + 1} ": " {p.clone()}</li> }
                }).collect::<Vec<_>>()
                }
            </ul>
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
