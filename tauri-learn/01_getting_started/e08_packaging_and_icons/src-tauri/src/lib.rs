// ============================================================
// 练习 E08: 打包与图标
// 目标: 理解 bundle 产物、图标与 identifier 规范
// TODO: 按照注释提示补全代码
// ============================================================

// 练习版允许存在未使用代码（TODO 尚未完成），
// 完成全部填空后该属性可删除。
#![allow(dead_code)]

// 返回给前端的打包信息结构体
#[derive(serde::Serialize)]
struct BundleInfo {
    identifier: String,
    product_name: String,
    version: String,
    icon_files: Vec<String>,
}

// === 步骤 1: 编写 bundle_info 命令 ————————————————————————
// 通过 AppHandle 注入读取应用配置与清单（均为固有方法，无需引入 trait）
// TODO: 添加 #[tauri::command] 属性
// TODO: 把参数 _app 改名为 app，并在函数内读取：
//   let config  = app.config();        // 应用配置（identifier）
//   let package = app.package_info();  // 应用清单（name / version）
fn bundle_info(_app: tauri::AppHandle) -> BundleInfo {
    BundleInfo {
        // TODO: 从 config.identifier 读取应用唯一标识（反向域名风格）
        identifier: String::new(),
        // TODO: 从 package.name 读取产品名
        product_name: String::new(),
        // TODO: 从 package.version 读取版本号（to_string()）
        version: String::new(),
        // TODO: 补充图标清单（Vec<String>），例如：
        //   "icons/icon.ico          # Windows 可执行文件嵌入图标"
        //   "icons/icon.png          # 通用 256×256 图标"
        //   "icons/32x32.png" / "icons/128x128.png" / "icons/128x128@2x.png"
        icon_files: vec![],
    }
}

// === 步骤 2: 注册命令 ————————————————————————————————————
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        // TODO: 注册 bundle_info 命令
        // 提示: .invoke_handler(tauri::generate_handler![bundle_info])
        .run(tauri::generate_context!())
        .expect("启动 Tauri 应用失败");
}