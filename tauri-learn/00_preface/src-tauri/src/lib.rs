 // ============================================================
 // Tauri v2 练习项目 — 导航首页 (Rust 后端)
 // ============================================================
 
 #[tauri::command]
 fn greet(name: &str) -> String {
     format!("你好, {}! 开始 Tauri v2 学习之旅", name)
 }
 
 #[cfg_attr(mobile, tauri::mobile_entry_point)]
 pub fn run() {
     tauri::Builder::default()
         .invoke_handler(tauri::generate_handler![greet])
         .run(tauri::generate_context!())
         .expect("启动 Tauri 应用失败");
 }
