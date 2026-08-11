// ============================================================
// 练习 E05: 参数与返回值
// 目标: 掌握命令参数（字符串/数字/布尔/Vec/结构体）与 serde 序列化
// TODO: 按照注释提示补全代码
// ============================================================


// 练习版允许存在未使用代码（TODO 尚未完成），
// 完成全部填空后该属性可删除。
#![allow(dead_code)]

// === 步骤 1: 定义参数结构体 ————————————————————————————————
// 前端传来的结构体参数（JS 侧用 camelCase 字段名传参）
// TODO: 为 CalcInput 添加 serde::Deserialize 派生
#[derive(serde::Deserialize)]
struct CalcInput {
    a: i32,
    b: i32,
}

// 返回给前端的结果结构体
// TODO: 为 Summary 添加 serde::Serialize 派生
#[derive(serde::Serialize)]
struct Summary {
    text_length: usize,
    doubled: i32,
    reversed_flag: bool,
    item_count: usize,
    total: i32,
}

// === 步骤 2: 编写 analyze 命令 ————————————————————————————
// TODO: 添加 #[tauri::command] 属性
// TODO: 补全参数列表：text: String, number: i32, flag: bool, items: Vec<String>, calc: CalcInput
// TODO: 补全返回类型：Summary
fn analyze() -> Summary {
    // TODO: 计算并填充 Summary：
    //   text_length = text.chars().count()（中文字符数）
    //   doubled    = number * 2
    //   reversed_flag = !flag
    //   item_count = items.len()
    //   total      = calc.a + calc.b
    Summary {
        text_length: 0,
        doubled: 0,
        reversed_flag: false,
        item_count: 0,
        total: 0,
    }
}

// === 步骤 3: 注册命令 ————————————————————————————————————
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        // TODO: 注册 analyze 命令
        // 提示: .invoke_handler(tauri::generate_handler![analyze])
        .run(tauri::generate_context!())
        .expect("启动 Tauri 应用失败");
}