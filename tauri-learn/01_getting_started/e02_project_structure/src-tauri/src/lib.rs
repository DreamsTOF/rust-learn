// ============================================================
// 练习 E02: 项目结构
// 目标: 理解 src/ 与 src-tauri/ 的分工、lib.rs 与 main.rs 的职责
// TODO: 按照注释提示补全代码
// ============================================================


// 练习版允许存在未使用代码（TODO 尚未完成），
// 完成全部填空后该属性可删除。
#![allow(dead_code)]

// === 步骤 1: 编写命令 ————————————————————————————————————
// TODO: 添加 #[tauri::command] 属性，使该函数成为可调用命令
fn project_layout() -> Vec<String> {
    // TODO: 补充结构说明行（当前为空列表，运行后页面无内容）
    // 每行格式: "路径说明".into()
    // 参考结构：
    //   src/                  # 前端（Vite + TS + HTML）
    //     index.html / src/main.ts / src/styles.css
    //   src-tauri/            # 后端（独立 Rust crate）
    //     src/main.rs（平台入口，调用 lib 的 run()）
    //     src/lib.rs（命令、状态、Builder 配置）
    //     tauri.conf.json / capabilities/ / icons/ / Cargo.toml / build.rs
    vec![]
}

// === 步骤 2: 注册命令 ————————————————————————————————————
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        // TODO: 注册 project_layout 命令
        // 提示: .invoke_handler(tauri::generate_handler![project_layout])
        .run(tauri::generate_context!())
        .expect("启动 Tauri 应用失败");
}