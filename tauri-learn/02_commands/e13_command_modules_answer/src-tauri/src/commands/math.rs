// 数学运算命令子模块。

/// 加法：a + b。
#[tauri::command]
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

/// 减法：a - b。
#[tauri::command]
pub fn sub(a: i32, b: i32) -> i32 {
    a - b
}