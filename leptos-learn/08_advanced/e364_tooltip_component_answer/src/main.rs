// ============================================================
// 练习 e364: tooltip_component — 工具提示组件
//
// 核心知识点:
//   - on:mouseenter / on:mouseleave 控制显隐
//   - 定位方向（上/下/左/右）
//   - 组件化封装与复用
//
// 难度: ⭐⭐
// ============================================================

use leptos::prelude::*;

/// 工具提示组件
#[component]
fn Tooltip(
    /// 提示文字内容
    text: String,
    /// 定位方向: "top" | "bottom" | "left" | "right"
    position: String,
    /// 触发元素
    children: ChildrenFn,
) -> impl IntoView {
    let (show, set_show) = signal(false);

    let position_styles = move || {
        match position.as_str() {
            "top" => "bottom: 100%; left: 50%; transform: translateX(-50%) translateY(-8px);",
            "bottom" => "top: 100%; left: 50%; transform: translateX(-50%) translateY(8px);",
            "left" => "right: 100%; top: 50%; transform: translateY(-50%) translateX(-8px);",
            "right" => "left: 100%; top: 50%; transform: translateY(-50%) translateX(8px);",
            _ => "bottom: 100%; left: 50%; transform: translateX(-50%) translateY(-8px);",
        }
    };

    view! {
        <div
            style="position: relative; display: inline-block;"
            on:mouseenter={move |_| set_show.set(true)}
            on:mouseleave={move |_| set_show.set(false)}
            on:focusin={move |_| set_show.set(true)}
            on:focusout={move |_| set_show.set(false)}
        >
            {children()}
            {move || show().then(|| {
                view! {
                    <div
                        style={format!(
                            "position: absolute; {} \
                             background: #333; color: white; padding: 6px 10px; \
                             border-radius: 4px; font-size: 0.85em; \
                             white-space: nowrap; z-index: 100; \
                             pointer-events: none; opacity: 0; \
                             animation: tooltipFadeIn 0.15s ease forwards;",
                            position_styles()
                        )}
                    >
                        {text.clone()}
                    </div>
                }
            })}
        </div>
    }
}

#[component]
fn Exercise() -> impl IntoView {
    view! {
        <div style="max-width: 500px; margin: 40px auto; font-family: sans-serif; text-align: center;">
            <h3>"工具提示演示"</h3>
            <p style="color: #666; margin-bottom: 40px;">
                "将鼠标悬停在下方的按钮上查看工具提示"
            </p>

            <div style="display: flex; justify-content: center; gap: 40px; flex-wrap: wrap;">
                <Tooltip
                    text="上方提示内容".to_string()
                    position="top".to_string()
                >
                    <button style="padding: 10px 20px; background: #3498db; color: white;
                                   border: none; border-radius: 6px; cursor: pointer;">
                        "上方（Top）"
                    </button>
                </Tooltip>

                <Tooltip
                    text="下方提示内容".to_string()
                    position="bottom".to_string()
                >
                    <button style="padding: 10px 20px; background: #2ecc71; color: white;
                                   border: none; border-radius: 6px; cursor: pointer;">
                        "下方（Bottom）"
                    </button>
                </Tooltip>

                <Tooltip
                    text="左侧提示内容".to_string()
                    position="left".to_string()
                >
                    <button style="padding: 10px 20px; background: #e67e22; color: white;
                                   border: none; border-radius: 6px; cursor: pointer;">
                        "左侧（Left）"
                    </button>
                </Tooltip>

                <Tooltip
                    text="右侧提示内容".to_string()
                    position="right".to_string()
                >
                    <button style="padding: 10px 20px; background: #9b59b6; color: white;
                                   border: none; border-radius: 6px; cursor: pointer;">
                        "右侧（Right）"
                    </button>
                </Tooltip>
            </div>

            <style>
                "@keyframes tooltipFadeIn {
                    from { opacity: 0; transform: scale(0.95); }
                    to { opacity: 1; transform: scale(1); }
                }"
            </style>
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
