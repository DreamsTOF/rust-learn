// ============================================================
// 练习 E02: 项目结构
// 目标: 理解 src/ 与 src-tauri/ 的分工、lib.rs 与 main.rs 的职责
// 知识点: 前后端目录约定 / main.rs 仅一行调用 lib / 配置文件
// ============================================================

/// 返回本项目结构说明（每行一个节点，前端按行渲染）
#[tauri::command]
fn project_layout() -> Vec<String> {
    vec![
        "src/                  # 前端（Vite + TS + HTML）".into(),
        "  index.html          # 页面骨架，浏览器加载入口".into(),
        "  src/main.ts         # 前端逻辑：UI 与 invoke 调用".into(),
        "  src/styles.css      # 全局样式".into(),
        "src-tauri/            # 后端（独立 Rust crate）".into(),
        "  src/main.rs         # 平台入口：只有一行，调用 lib 的 run()".into(),
        "  src/lib.rs          # 核心：命令、状态、Builder 配置都在这".into(),
        "  tauri.conf.json     # 应用配置：identifier / 窗口 / 构建命令".into(),
        "  capabilities/       # 权限声明（core:default 最小权限）".into(),
        "  icons/              # 应用图标（icon.ico / icon.png）".into(),
        "  Cargo.toml          # Rust 依赖（tauri 等）".into(),
        "  build.rs            # 构建脚本，调用 tauri_build::build()".into(),
    ]
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![project_layout])
        .run(tauri::generate_context!())
        .expect("启动 Tauri 应用失败");
}