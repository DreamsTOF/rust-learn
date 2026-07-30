// ============================================================
// 参考答案 e369: WebGL 集成 — canvas + WebGL 渲染
//
// 核心知识点:
//   - 通过 <canvas> 元素获取 WebGL 上下文
//   - 使用 inline_js 封装 WebGL 绘制逻辑
//   - 使用 node_ref= 获取 canvas 元素的引用
//   - 在 Effect::new 中初始化 WebGL
// ============================================================

use leptos::prelude::*;
use std::cell::Cell;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;

// 通过 inline_js 封装 WebGL 初始化与绘制
#[wasm_bindgen(inline_js = r#"
export function initWebGL(canvas) {
    const gl = canvas.getContext("webgl");
    if (!gl) return false;

    // 顶点着色器
    const vs = gl.createShader(gl.VERTEX_SHADER);
    gl.shaderSource(vs, "attribute vec2 a_position; void main() { gl_Position = vec4(a_position, 0.0, 1.0); }");
    gl.compileShader(vs);

    // 片元着色器
    const fs = gl.createShader(gl.FRAGMENT_SHADER);
    gl.shaderSource(fs, "precision mediump float; void main() { gl_FragColor = vec4(0.3, 0.6, 1.0, 1.0); }");
    gl.compileShader(fs);

    // 着色器程序
    const program = gl.createProgram();
    gl.attachShader(program, vs);
    gl.attachShader(program, fs);
    gl.linkProgram(program);
    gl.useProgram(program);

    // 顶点数据 — 三角形
    const vertices = new Float32Array([0.0, 0.5, -0.5, -0.5, 0.5, -0.5]);
    const buffer = gl.createBuffer();
    gl.bindBuffer(gl.ARRAY_BUFFER, buffer);
    gl.bufferData(gl.ARRAY_BUFFER, vertices, gl.STATIC_DRAW);

    const positionLoc = gl.getAttribLocation(program, "a_position");
    gl.vertexAttribPointer(positionLoc, 2, gl.FLOAT, false, 0, 0);
    gl.enableVertexAttribArray(positionLoc);

    // 清空并绘制
    gl.clearColor(0.95, 0.95, 0.95, 1.0);
    gl.clear(gl.COLOR_BUFFER_BIT);
    gl.drawArrays(gl.TRIANGLES, 0, 3);

    return true;
}
"#)]
extern "C" {
    fn initWebGL(canvas: &JsValue) -> bool;
}

#[component]
fn Exercise() -> impl IntoView {
    let canvas_ref = NodeRef::<leptos::html::Canvas>::new();
    let initialized = Cell::new(false);

    // 一次性初始化 WebGL
    Effect::new(move || {
        if initialized.get() {
            return;
        }
        if let Some(canvas) = canvas_ref.get() {
            initWebGL(&JsValue::from(canvas));
            initialized.set(true);
        }
    });

    view! {
        <div style="padding: 1rem; font-family: sans-serif; max-width: 480px; margin: 0 auto;">
            <h3>"练习 e369: WebGL 集成"</h3>
            <p style="color: #666; font-size: 14px;">
                "Canvas 显示 WebGL 渲染的彩色三角形"
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
