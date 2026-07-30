// ============================================================
// 练习 155: complex_hook_combo — use_form_state 组合验证/提交/重置
//
// 目标: 实现一个完整的表单状态管理 hook，包含字段管理、验证、
//       提交和重置功能。
//
// 难度: ⭐⭐⭐
// 核心知识点: 复杂 Hook 组合、表单验证、状态管理
//
// TODO: 按照注释提示补全代码
// ============================================================

use leptos::prelude::*;

/// 表单字段值
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

/// 表单验证错误
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

/// 表单状态 Hook
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

    // 提交函数
    let submit = {
        let validator = validator;
        move || {
            // 验证
            let errs = validator(&values.get());
            errors.set(errs);

            // 无错误则提交
            let e = errors.get();
            if e.username.is_none() && e.email.is_none() && e.age.is_none() {
                set_is_submitting.set(true);
                on_submit(values.get());
                set_is_submitting.set(false);
            }
        }
    };

    // 重置函数
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

/// 验证用户名
fn validate_username(v: &str) -> Option<String> {
    if v.is_empty() {
        Some("用户名不能为空".to_string())
    } else if v.len() < 3 {
        Some("用户名至少 3 个字符".to_string())
    } else {
        None
    }
}

/// 验证邮箱
fn validate_email(v: &str) -> Option<String> {
    if v.is_empty() {
        Some("邮箱不能为空".to_string())
    } else if !v.contains('@') {
        Some("邮箱格式不正确".to_string())
    } else {
        None
    }
}

/// 验证年龄
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

/// 综合验证器
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
        // 实时验证
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
