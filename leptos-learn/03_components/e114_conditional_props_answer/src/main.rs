// ============================================================
// Exercise e114: conditional_props — Answer
// ============================================================

use leptos::prelude::*;

#[component]
fn MessageBox(
    /// 变体类型: "info" | "warning" | "error"
    #[prop(default = "info")]
    variant: &'static str,
    /// 是否高亮显示
    #[prop(default = false)]
    highlight: bool,
    /// 消息文本
    text: &'static str,
) -> impl IntoView {
    let bg_color = if highlight { "#fff3cd" } else { "#f8f9fa" };
    let border_color = match variant {
        "warning" => "#ffc107",
        "error" => "#dc3545",
        _ => "#0d6efd",
    };
    let icon = match variant {
        "warning" => "⚠️",
        "error" => "❌",
        _ => "ℹ️",
    };

    view! {
        <div style=format!(
            "background: {}; border-left: 4px solid {}; padding: 12px; border-radius: 4px; margin: 8px 0;",
            bg_color, border_color,
        )>
            <p>{icon} " [" {variant} "] " {text}</p>
        </div>
    }
}

#[component]
fn Exercise() -> impl IntoView {
    view! {
        <div>
            <h3>"条件 Props"</h3>
            <MessageBox text="这是一条普通信息" />
            <MessageBox variant="warning" text="注意：磁盘空间不足" />
            <MessageBox variant="error" highlight=true text="系统发生错误！" />
            <MessageBox highlight=true text="这是一条高亮信息" />
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
