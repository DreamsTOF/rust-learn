// ============================================================
// 练习 E11: 错误处理
// 目标: 用 thiserror 定义错误枚举并演示错误链
// TODO: 按照注释提示补全代码
// ============================================================

// 练习版允许存在未使用代码（TODO 尚未完成），
// 完成全部填空后该属性可删除。
#![allow(dead_code)]

// === 步骤 1: 定义错误枚举 AppError ————————————————————————————————————
// TODO: 添加 #[derive(Debug, thiserror::Error)] 派生
// TODO: 为变体添加 #[error("...")] 消息；Io 加 #[from] 实现自动转换
// 提示: #[error("输入无效: {0}")] InvalidInput(String)
//       #[error("IO 错误: {0}")] Io(#[from] std::io::Error)
// TODO: 为枚举实现 serde::Serialize（Tauri 命令要求错误类型可序列化，
//       序列化为 Display 消息字符串传给前端）：
// 提示: impl serde::Serialize for AppError {
//         fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
//           s.serialize_str(&self.to_string())
//         }
//       }
#[derive(Debug)]
enum AppError {
    InvalidInput(String),
    Io(std::io::Error),
}

// === 步骤 2: 编写 parse_number 命令 ————————————————————————————————————
// TODO: 添加 #[tauri::command] 属性
// TODO: 解析数字并 map_err 为 InvalidInput，返回 v * 2.0
// 提示: input.trim().parse::<f64>().map_err(|_| AppError::InvalidInput(input))?;
fn parse_number(_input: String) -> Result<f64, AppError> {
    // TODO: 补全解析与翻倍逻辑（当前返回 0.0 占位）
    Ok(0.0)
}

// === 步骤 3: 编写 read_marker 命令 ————————————————————————————————————
// TODO: 添加 #[tauri::command] 属性
// TODO: 读取不存在的文件，用 ? 传播 IO 错误（触发 Io 错误链）
// 提示: std::fs::read_to_string("C:/nonexistent-marker.txt")?;
fn read_marker() -> Result<String, AppError> {
    // TODO: 读取文件并返回内容（当前返回空串占位）
    Ok(String::new())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        // === 步骤 4: 注册命令 ————————————————————————————————————
        // TODO: 注册 parse_number 与 read_marker
        // 提示: .invoke_handler(tauri::generate_handler![parse_number, read_marker])
        .run(tauri::generate_context!())
        .expect("启动 Tauri 应用失败");
}