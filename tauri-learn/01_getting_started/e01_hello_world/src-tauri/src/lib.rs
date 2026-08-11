// ============================================================
// 练习 E01: 环境准备与项目创建
// 目标: 了解 Tauri 开发前置条件，用后端命令返回环境检查结果
// TODO: 按照注释提示补全代码
// ============================================================


// 练习版允许存在未使用代码（TODO 尚未完成），
// 完成全部填空后该属性可删除。
#![allow(dead_code)]

// === 步骤 1: 定义数据结构 ————————————————————————————————
// 后端通过 serde 序列化把 Rust 结构体返回给前端
// TODO: 为 EnvCheck 添加 serde::Serialize 派生（保持字段不变）
// 提示: #[derive(serde::Serialize)]
#[derive(serde::Serialize)]
struct EnvCheck {
    name: String,
    ok: bool,
    detail: String,
}

// === 步骤 2: 编写命令 ————————————————————————————————————
// 命令 = 普通 Rust 函数 + #[tauri::command] 标注
// TODO: 添加 #[tauri::command] 属性，使该函数成为可调用命令
fn check_environment() -> Vec<EnvCheck> {
    // TODO: 补充检查项（当前为空列表，运行后页面无内容）
    // 每项格式: EnvCheck { name: "...".into(), ok: true, detail: "...".into() }
    // 建议 5 项：Rust 工具链 / Node.js 与 pnpm / WebView2 Runtime / Tauri CLI / Rust 目标链
    vec![]
}

// === 步骤 3: 注册命令 ————————————————————————————————————
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        // TODO: 注册 check_environment 命令，让前端可以 invoke
        // 提示: .invoke_handler(tauri::generate_handler![check_environment])
        .run(tauri::generate_context!())
        .expect("启动 Tauri 应用失败");
}