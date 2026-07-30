// ============================================================
// 练习 e369: WebGL 集成 — canvas + WebGL 渲染
//
// 核心知识点:
//   - 通过 <canvas> 元素获取 WebGL 上下文
//   - 使用 web_sys::WebGlRenderingContext 绘制简单图形
//   - 使用 node_ref= 获取 canvas 元素的引用
//   - 在 Effect::new 中初始化 WebGL
//
// 难度: ⭐⭐⭐ (关键位置有 TODO，补全约 50%)
// ============================================================

use leptos::prelude::*;
use std::cell::Cell;

// TODO: 完成 WebGL 初始化
// 1. 在 Effect::new 中通过 canvas_ref.get() 获取 canvas 元素
// 2. 调用 canvas.get_context("webgl", ...) 获取 WebGL 上下文
// 3. 设置清空颜色 (clearColor) 并清空画布 (clear)
// 4. 绘制一个基本图形（如三角形或矩形）

#[component]
fn Exercise() -> impl IntoView {
    let canvas_ref = NodeRef::<leptos::html::Canvas>::new();
    let initialized = Cell::new(false);

    // TODO: 在 Effect::new 中初始化 WebGL
    // 获取 canvas 元素、获取 WebGL 上下文、绘制图形

    view! {
        <div style="padding: 1rem; font-family: sans-serif; max-width: 480px; margin: 0 auto;">
            <h3>"练习 e369: WebGL 集成"</h3>
            <p style="color: #666; font-size: 14px;">
                "Canvas 显示 WebGL 渲染的彩色图形"
            </p>
            <canvas
                node_ref=canvas_ref
                width="400"
                height="300"
                style="border: 1px solid #ddd; border-radius: 8px; display: block;"
            ></canvas>
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
