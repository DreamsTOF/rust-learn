// ============================================================
// 练习 e364: tooltip_component — 工具提示组件
//
// 核心知识点:
//   - on:mouseenter / on:mouseleave 控制显隐
//   - 定位方向（上/下/左/右）
//   - 组件化封装与复用
//
// 难度: ⭐⭐ (关键位置有 TODO，补全约 50%)
// ============================================================

use leptos::prelude::*;

/// 工具提示组件
#[component]
fn Tooltip(
    text: String,
    position: String,
    children: ChildrenFn,
) -> impl IntoView {
    // TODO: 创建 show 信号控制提示框显隐
    // 使用 signal(false) 创建 (ReadSignal, WriteSignal)

    // TODO: 根据 position 返回对应的 CSS 定位样式字符串
    // "top" → bottom:100%; left:50%; transform: translateX(-50%) translateY(-8px)
    // "bottom" → top:100%; left:50%; ...
    // "left" → right:100%; top:50%; ...
    // "right" → left:100%; top:50%; ...
    let position_styles = move || {
        // === 你的代码开始 ===
        todo!("根据 position 返回定位样式");
        // === 你的代码结束 ===
    };

    view! {
        <div
            style="position: relative; display: inline-block;"
            // TODO: 绑定 on:mouseenter / on:mouseleave 事件控制 show
            // TODO: 绑定 on:focusin / on:focusout 事件支持键盘聚焦
        >
            {children()}
            // TODO: 当 show() 为 true 时渲染提示框
            // 使用 format!() 拼接 position_styles() 到 style 属性中
            // 提示框背景 #333，文字白色，圆角 4px，动画 tooltipFadeIn
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
