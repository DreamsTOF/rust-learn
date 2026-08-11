// ============================================================
// 练习 E01: 环境准备与项目创建
// 目标: 了解 Tauri 开发前置条件，用后端命令返回环境检查结果
// 知识点: #[tauri::command] / generate_handler! / serde::Serialize
// ============================================================

// 环境检查项：名称 + 是否就绪 + 说明
#[derive(serde::Serialize)]
struct EnvCheck {
    name: String,
    ok: bool,
    detail: String,
}

/// 返回开发前置条件检查清单。
/// 真实项目可在此读取 rustc / node 版本做动态判断，
/// 本练习以教学为目的直接给出结论。
#[tauri::command]
fn check_environment() -> Vec<EnvCheck> {
    vec![
        EnvCheck {
            name: "Rust 工具链".into(),
            ok: true,
            detail: "cargo 1.8x+ / rustc stable".into(),
        },
        EnvCheck {
            name: "Node.js 与 pnpm".into(),
            ok: true,
            detail: "Node 18+ / pnpm 9+".into(),
        },
        EnvCheck {
            name: "WebView2 Runtime".into(),
            ok: true,
            detail: "Windows 11 自带，Windows 10 需安装".into(),
        },
        EnvCheck {
            name: "Tauri CLI".into(),
            ok: true,
            detail: "cargo tauri 2.x（或 pnpm dlx tauri）".into(),
        },
        EnvCheck {
            name: "Rust 目标链".into(),
            ok: true,
            detail: "x86_64-pc-windows-msvc".into(),
        },
    ]
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![check_environment])
        .run(tauri::generate_context!())
        .expect("启动 Tauri 应用失败");
}