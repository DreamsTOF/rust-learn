 // ============================================================
 // 模板: Minimal (Rust 后端)
 //
 // 最小化的 Tauri Rust 后端
 // ============================================================
 
 #[tauri::command]
 fn greet(name: &str) -> String {
     format!("你好, {}! 欢迎使用 Tauri v2", name)
 }
 
 #[cfg_attr(mobile, tauri::mobile_entry_point)]
 pub fn run() {
     tauri::Builder::default()
         .invoke_handler(tauri::generate_handler![greet])
         .run(tauri::generate_context!())
         .expect("启动 Tauri 应用失败");
 }
