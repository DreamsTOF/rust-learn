// ============================================================
// 练习 E44: 内容安全策略（CSP）
// 目标: 理解 security.csp 配置，并用命令读取当前 CSP 字符串
// 知识点: app.security.csp / default-src / style-src / connect-src / 配置读取
// TODO: 按照注释提示补全代码
// ============================================================

// 练习版允许存在未使用代码（TODO 尚未完成），
// 完成全部填空后该属性可删除。
#![allow(dead_code)]

// === 步骤 1: 编写 get_csp 命令 ————————————————————————————
// TODO: 添加 #[tauri::command] 属性（让函数成为可被前端调用的命令）
// TODO: 补全命令体：读取 app.config().app.security.csp 并转成字符串
// 提示: app.config().app.security.csp
//         .as_ref().map(|c| c.to_string()).unwrap_or_default()
fn get_csp(_app: tauri::AppHandle) -> Result<String, String> {
    // TODO: 把下面的占位替换为真实读取逻辑：
    //   app.config() 是 AppHandle 固有方法（无需 use tauri::Manager），
    //   security.csp 类型是 Option<Csp>，Csp 实现了 Display，
    //   未配置时返回空字符串即可
    // 提示: Ok(app.config().app.security.csp
    //         .as_ref().map(|c| c.to_string()).unwrap_or_default())
    Ok(String::new())
}

// === 步骤 2: 注册命令 ————————————————————————————————————
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        // TODO: 注册 get_csp 命令
        // 提示: .invoke_handler(tauri::generate_handler![get_csp])
        .run(tauri::generate_context!())
        .expect("启动 Tauri 应用失败");
}