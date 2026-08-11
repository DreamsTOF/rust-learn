// ============================================================
// 练习 E16: setup 钩子
// 目标: 在 setup 阶段初始化状态、执行异步任务与主线程回调
// TODO: 按照注释提示补全代码
// ============================================================

// 练习版允许存在未使用代码（TODO 尚未完成），
// 完成全部填空后该属性可删除。
#![allow(dead_code)]

use std::sync::Mutex;

/// 初始化状态：setup 中写入，命令中读取。
struct SetupState(Mutex<String>);

// === 步骤 1: 编写读取状态的命令 ————————————————————————————————————
// TODO: 添加 #[tauri::command] 属性
// TODO: 补全函数体：lock 后克隆字符串返回
// 提示: fn get_setup_state(state: State<SetupState>) -> Result<String, String>
//       需要时在文件顶部添加: use tauri::State;
fn get_setup_state(_state: tauri::State<SetupState>) -> Result<String, String> {
    // TODO: 补全 lock 与 clone（当前返回空串占位）
    Ok(String::new())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|_app| {
            println!("[setup] 应用启动，窗口已创建");

            // === 步骤 2: 注入共享状态 ————————————————————————————————————
            // TODO: 把 SetupState 注册为可注入状态（默认值 "未初始化"）
            // 提示: _app.manage(SetupState(Mutex::new("未初始化".into())));
            //       （需要 use tauri::Manager;）

            // === 步骤 3: 异步任务延迟广播 ————————————————————————————————————
            // TODO: clone handle → spawn 异步任务 → sleep 1 秒 → emit("init-done", "初始化完成")
            // 提示: let handle = _app.handle().clone();
            //       tauri::async_runtime::spawn(async move {
            //           tokio::time::sleep(std::time::Duration::from_secs(1)).await;
            //           let _ = handle.emit("init-done", "初始化完成");
            //       });
            //       （需要 use tauri::Emitter;）

            // === 步骤 4: 主线程回调 ————————————————————————————————————
            // TODO: 在主线程上执行一次打印
            // 提示: let _ = _app.run_on_main_thread(|| println!("[setup] run_on_main_thread 回调（主线程）"));

            Ok(())
        })
        // === 步骤 5: 注册命令 ————————————————————————————————————
        // TODO: 注册 get_setup_state
        // 提示: .invoke_handler(tauri::generate_handler![get_setup_state])
        .run(tauri::generate_context!())
        .expect("启动 Tauri 应用失败");
}