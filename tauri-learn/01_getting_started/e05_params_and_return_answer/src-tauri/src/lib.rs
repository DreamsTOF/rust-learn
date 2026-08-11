// ============================================================
// 练习 E05: 参数与返回值
// 目标: 掌握命令参数（字符串/数字/布尔/Vec/结构体）与 serde 序列化
// 知识点: Deserialize 参数 / Serialize 返回 / snake_case ↔ camelCase
// ============================================================

// 前端传来的结构体参数：
// Rust 侧字段是 snake_case，JS 侧自动使用 camelCase 传参
#[derive(serde::Deserialize)]
struct CalcInput {
    a: i32,
    b: i32,
}

// 返回给前端的结果结构体
#[derive(serde::Serialize)]
struct Summary {
    text_length: usize,
    doubled: i32,
    reversed_flag: bool,
    item_count: usize,
    total: i32,
}

/// 混合多种参数类型（String / i32 / bool / Vec / 结构体），
/// 返回结构化结果，演示完整序列化链路。
#[tauri::command]
fn analyze(
    text: String,
    number: i32,
    flag: bool,
    items: Vec<String>,
    calc: CalcInput,
) -> Summary {
    Summary {
        text_length: text.chars().count(),
        doubled: number * 2,
        reversed_flag: !flag,
        item_count: items.len(),
        total: calc.a + calc.b,
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![analyze])
        .run(tauri::generate_context!())
        .expect("启动 Tauri 应用失败");
}