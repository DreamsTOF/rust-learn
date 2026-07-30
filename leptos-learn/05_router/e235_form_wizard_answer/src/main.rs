use leptos::prelude::*;
use leptos_router::components::*;
use leptos_router::hooks::use_navigate;
use leptos_router::{path, NavigateOptions};

#[derive(Clone, Default)]
struct WizardData {
    name: String,
    email: String,
    address: String,
}

#[component]
fn StepIndicator(current: &'static str) -> impl IntoView {
    view! {
        <div style="margin-bottom: 12px;">
            <span>"步骤: "</span>
            <A href="/wizard/step1" attr:class=move || if current == "step1" { "active" } else { "" }>
                "1. 基本信息"
            </A>
            <span>" → "</span>
            <A href="/wizard/step2" attr:class=move || if current == "step2" { "active" } else { "" }>
                "2. 地址"
            </A>
            <span>" → "</span>
            <A href="/wizard/step3" attr:class=move || if current == "step3" { "active" } else { "" }>
                "3. 确认"
            </A>
        </div>
    }
}

#[component]
fn Step1() -> impl IntoView {
    let data = use_context::<RwSignal<WizardData>>()
        .expect("WizardData not provided");
    let navigate = use_navigate();
    let (name, set_name) = signal(String::new());
    let (email, set_email) = signal(String::new());

    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        data.update(|d| {
            d.name = name.get();
            d.email = email.get();
        });
        let _ = navigate("/wizard/step2", NavigateOptions::default());
    };

    view! {
        <StepIndicator current="step1"/>
        <h3>"步骤 1: 基本信息"</h3>
        <form on:submit=on_submit>
            <div>
                <label>"姓名: "
                    <input type="text" name="name"
                        prop:value=name
                        on:input=move |ev| set_name.set(event_target_value(&ev))
                        required
                    />
                </label>
            </div>
            <div>
                <label>"邮箱: "
                    <input type="email" name="email"
                        prop:value=email
                        on:input=move |ev| set_email.set(event_target_value(&ev))
                        required
                    />
                </label>
            </div>
            <button type="submit">"下一步"</button>
        </form>
    }
}

#[component]
fn Step2() -> impl IntoView {
    let data = use_context::<RwSignal<WizardData>>()
        .expect("WizardData not provided");
    let navigate = use_navigate();
    let (address, set_address) = signal(String::new());

    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        data.update(|d| d.address = address.get());
        let _ = navigate("/wizard/step3", NavigateOptions::default());
    };

    view! {
        <StepIndicator current="step2"/>
        <h3>"步骤 2: 地址信息"</h3>
        <form on:submit=on_submit>
            <div>
                <label>"地址: "
                    <input type="text" name="address"
                        prop:value=address
                        on:input=move |ev| set_address.set(event_target_value(&ev))
                        required
                    />
                </label>
            </div>
            <button type="submit">"下一步"</button>
        </form>
    }
}

#[component]
fn Step3() -> impl IntoView {
    let data = use_context::<RwSignal<WizardData>>()
        .expect("WizardData not provided");
    let (submitted, set_submitted) = signal(false);

    let on_submit = move |_| {
        set_submitted.set(true);
        data.set(WizardData::default());
    };

    view! {
        <StepIndicator current="step3"/>
        <h3>"步骤 3: 确认信息"</h3>
        {move || {
            if submitted() {
                view! {
                    <div>
                        <p>"提交成功！"</p>
                        <A href="/wizard/step1">"重新开始"</A>
                    </div>
                }.into_any()
            } else {
                let d = data.get();
                view! {
                    <div style="border: 1px solid #ccc; padding: 12px; margin: 8px 0;">
                        <p>"姓名: " {d.name.clone()}</p>
                        <p>"邮箱: " {d.email.clone()}</p>
                        <p>"地址: " {d.address.clone()}</p>
                    </div>
                    <button on:click=on_submit>"确认提交"</button>
                }.into_any()
            }
        }}
    }
}

#[component]
fn WizardLayout() -> impl IntoView {
    let data = RwSignal::new(WizardData::default());
    provide_context(data);

    view! {
        <div style="border: 2px solid #2196F3; padding: 16px; border-radius: 8px;">
            <h2>"注册向导"</h2>
            <Outlet/>
        </div>
    }
}

#[component]
fn Exercise() -> impl IntoView {
    view! {
        <Router>
            <h1>"e235: 表单向导"</h1>
            <Routes fallback=|| "页面未找到">
                <Route path=path!("") view=|| view! {
                    <p>"欢迎使用多步注册向导"</p>
                    <A href="/wizard/step1">"开始注册"</A>
                }/>
                <ParentRoute path=path!("wizard") view=WizardLayout>
                    <Route path=path!("step1") view=Step1/>
                    <Route path=path!("step2") view=Step2/>
                    <Route path=path!("step3") view=Step3/>
                </ParentRoute>
            </Routes>
        </Router>
    }
}

fn main() {
    mount_to_body(Exercise);
}
