 // ============================================================
 // 模板: Vite + TypeScript (Rust 后端)
 //
 // 标准 Tauri v2 练习后端模板
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
