// ============================================================
// 练习 E11: 错误处理
// 目标: 用 thiserror 定义错误枚举并演示错误链
// 知识点: thiserror / #[from] 错误链 / map_err / ? 传播
// ============================================================

use thiserror::Error;

/// 应用级错误枚举：统一错误类型，thiserror 自动生成 Display。
#[derive(Debug, Error)]
enum AppError {
    #[error("输入无效: {0}")]
    InvalidInput(String),
    #[error("IO 错误: {0}")]
    Io(#[from] std::io::Error),
}

/// 错误序列化：把错误作为字符串传给前端（即 thiserror 的 Display 消息）。
impl serde::Serialize for AppError {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.to_string())
    }
}

/// 解析数字并翻倍；非法输入映射为 AppError::InvalidInput。
#[tauri::command]
fn parse_number(input: String) -> Result<f64, AppError> {
    let v: f64 = input
        .trim()
        .parse()
        .map_err(|_| AppError::InvalidInput(input))?;
    Ok(v * 2.0)
}

/// 读取一个不存在的文件，? 自动经 #[from] 触发 Io 错误链。
#[tauri::command]
fn read_marker() -> Result<String, AppError> {
    let content = std::fs::read_to_string("C:/nonexistent-marker.txt")?;
    Ok(format!("读到标记文件: {content}"))
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![parse_number, read_marker])
        .run(tauri::generate_context!())
        .expect("启动 Tauri 应用失败");
}