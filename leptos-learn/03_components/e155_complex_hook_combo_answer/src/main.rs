// ============================================================
// Exercise 155 - Answer: complex_hook_combo
// ============================================================

use leptos::prelude::*;

#[derive(Clone, Debug)]
struct FormValues {
    username: String,
    email: String,
    age: String,
}

impl Default for FormValues {
    fn default() -> Self {
        Self {
            username: String::new(),
            email: String::new(),
            age: String::new(),
        }
    }
}

#[derive(Clone, Debug)]
struct FormErrors {
    username: Option<String>,
    email: Option<String>,
    age: Option<String>,
}

impl Default for FormErrors {
    fn default() -> Self {
        Self {
            username: None,
            email: None,
            age: None,
        }
    }
}

fn use_form_state<F>(
    initial: FormValues,
    validator: F,
    on_submit: impl Fn(FormValues) + 'static,
) -> (
    RwSignal<FormValues>,
    RwSignal<FormErrors>,
    ReadSignal<bool>,
    impl Fn() + 'static,
    impl Fn() + 'static,
)
where
    F: Fn(&FormValues) -> FormErrors + 'static,
{
    let values = RwSignal::new(initial.clone());
    let errors = RwSignal::new(FormErrors::default());
    let (is_submitting, set_is_submitting) = signal(false);

    let submit = {
        let validator = validator;
        move || {
            let errs = validator(&values.get());
            errors.set(errs);

            let e = errors.get();
            if e.username.is_none() && e.email.is_none() && e.age.is_none() {
                set_is_submitting.set(true);
                on_submit(values.get());
                set_is_submitting.set(false);
            }
        }
    };

    let reset = {
        let initial = initial;
        move || {
            values.set(initial.clone());
            errors.set(FormErrors::default());
            set_is_submitting.set(false);
        }
    };

    (values, errors, is_submitting, submit, reset)
}

fn validate_username(v: &str) -> Option<String> {
    if v.is_empty() {
        Some("用户名不能为空".to_string())
    } else if v.len() < 3 {
        Some("用户名至少 3 个字符".to_string())
    } else {
        None
    }
}

fn validate_email(v: &str) -> Option<String> {
    if v.is_empty() {
        Some("邮箱不能为空".to_string())
    } else if !v.contains('@') {
        Some("邮箱格式不正确".to_string())
    } else {
        None
    }
}

fn validate_age(v: &str) -> Option<String> {
    if v.is_empty() {
        Some("年龄不能为空".to_string())
    } else {
        match v.parse::<u32>() {
            Ok(n) if n < 1 || n > 150 => Some("年龄应在 1-150 之间".to_string()),
            Ok(_) => None,
            Err(_) => Some("请输入有效数字".to_string()),
        }
    }
}

fn form_validator(values: &FormValues) -> FormErrors {
    FormErrors {
        username: validate_username(&values.username),
        email: validate_email(&values.email),
        age: validate_age(&values.age),
    }
}

#[derive(Clone, Copy)]
enum Field {
    Username,
    Email,
    Age,
}

#[component]
fn Exercise() -> impl IntoView {
    let (values, errors, is_submitting, submit, reset) = use_form_state(
        FormValues::default(),
        form_validator,
        |vals| {
            leptos::logging::log!("表单提交: {:?}", vals);
        },
    );

    let update_field = move |field: Field, value: String| {
        values.update(|v| match field {
            Field::Username => v.username = value,
            Field::Email => v.email = value,
            Field::Age => v.age = value,
        });
        let errs = form_validator(&values.get());
        errors.set(errs);
    };

    view! {
        <div>
            <h3>"练习 155: complex_hook_combo"</h3>
            <div>
                <label>"用户名: "</label>
                <input
                    type="text"
                    prop:value=move || values.get().username
                    on:input=move |ev| update_field(Field::Username, event_target_value(&ev))
                />
                <span style="color: red;">
                    {move || errors.get().username.unwrap_or_default()}
                </span>
            </div>
            <div>
                <label>"邮箱: "</label>
                <input
                    type="text"
                    prop:value=move || values.get().email
                    on:input=move |ev| update_field(Field::Email, event_target_value(&ev))
                />
                <span style="color: red;">
                    {move || errors.get().email.unwrap_or_default()}
                </span>
            </div>
            <div>
                <label>"年龄: "</label>
                <input
                    type="text"
                    prop:value=move || values.get().age
                    on:input=move |ev| update_field(Field::Age, event_target_value(&ev))
                />
                <span style="color: red;">
                    {move || errors.get().age.unwrap_or_default()}
                </span>
            </div>
            <div>
                <button on:click=move |_| submit()>
                    {move || if is_submitting() { "提交中..." } else { "提交" }}
                </button>
                <button on:click=move |_| reset()>"重置"</button>
            </div>
            <div>
                <p>"提交状态: " {move || if is_submitting() { "⏳ 提交中" } else { "✅ 就绪" }}</p>
            </div>
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
