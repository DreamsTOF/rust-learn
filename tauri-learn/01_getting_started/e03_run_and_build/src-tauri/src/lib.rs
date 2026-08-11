// ============================================================
// 练习 E03: 运行与构建
// 目标: 理解 tauri dev / tauri build 与 devUrl / frontendDist
// TODO: 按照注释提示补全代码
// ============================================================

// 练习版允许存在未使用代码（TODO 尚未完成），
// 完成全部填空后该属性可删除。
#![allow(dead_code)]

// 返回给前端的构建信息结构体
#[derive(serde::Serialize)]
struct BuildInfo {
    dev_url: Option<String>,
    frontend_dist: String,
    identifier: String,
    product_name: String,
}

// === 步骤 1: 编写命令 ————————————————————————————————————
// 通过 AppHandle 注入读取运行时配置（app.config() / app.package_info()
// 都是 AppHandle 的固有方法，无需额外引入 trait）
// TODO: 添加 #[tauri::command] 属性
// TODO: 把参数 _app 改名为 app，并在函数内读取：
//   let config  = app.config();        // 应用配置（devUrl / frontendDist / identifier）
//   let package = app.package_info();  // 应用清单（name）
fn build_info(_app: tauri::AppHandle) -> BuildInfo {
    BuildInfo {
        // TODO: 从 config.build.dev_url 读取 dev 模式加载源
        // 提示: config.build.dev_url.as_ref().map(|u| u.to_string())
        dev_url: None,
        // TODO: 从 config.build.frontend_dist 读取构建产物目录（注意是 Option）
        // 提示: config.build.frontend_dist.as_ref().map(|d| d.to_string()).unwrap_or_default()
        frontend_dist: String::new(),
        // TODO: 从 config.identifier 读取应用标识（identifier 在配置中，不在清单里）
        identifier: String::new(),
        // TODO: 从 package.name 读取产品名
        product_name: String::new(),
    }
}

// === 步骤 2: 注册命令 ————————————————————————————————————
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        // TODO: 注册 build_info 命令
        // 提示: .invoke_handler(tauri::generate_handler![build_info])
        .run(tauri::generate_context!())
        .expect("启动 Tauri 应用失败");
}