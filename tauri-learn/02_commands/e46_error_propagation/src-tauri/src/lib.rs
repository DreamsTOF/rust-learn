// ============================================================
// 练习 E46: 自定义错误传播
// 目标: thiserror 错误枚举 + 错误码，前端按 code 分类处理
// 知识点: thiserror / 自定义 Serialize / 错误码映射（承接 E11）
// TODO: 按照注释提示补全代码
// ============================================================

// 练习版允许存在未使用代码（TODO 尚未完成），
// 完成全部填空后该属性可删除。
#![allow(dead_code)]

use thiserror::Error;

/// 应用级错误枚举：thiserror 自动生成 Display 消息。
// === 步骤 1: 补全错误枚举变体 ——————————————————————————————
// TODO: 补上另外两个变体（NotFound(String) / Internal(String)）
// 提示: #[error("资源不存在: {0}")] NotFound(String),
//       #[error("内部错误: {0}")] Internal(String),
#[derive(Debug, Error)]
enum AppError {
    #[error("无效输入: {0}")]
    InvalidInput(String),
}

impl AppError {
    /// 每个错误变体对应一个 HTTP 风格错误码，前端据此分类展示。
    // === 步骤 2: 补全错误码映射 ————————————————————————————
    // TODO: 补上 NotFound → 404、Internal → 500 两个分支
    // 提示: AppError::NotFound(_) => 404,
    //       AppError::Internal(_) => 500,
    fn code(&self) -> i32 {
        match self {
            AppError::InvalidInput(_) => 400,
        }
    }
}

/// 统一错误载荷：前端拿到的错误就是一个 { code, message } 对象。
#[derive(Debug, serde::Serialize)]
struct ErrorBody {
    code: i32,
    message: String,
}

/// 自定义序列化：把 thiserror 枚举序列化为 { code, message }，
/// 这样前端可以直接按 code 分类，而不是去猜错误字符串。
impl serde::Serialize for AppError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // === 步骤 3: 构造 ErrorBody 并序列化 ——————————————————
        // TODO: 用 self.code() 与 self.to_string() 构造 ErrorBody
        // 提示: ErrorBody { code: self.code(), message: self.to_string() }
        //           .serialize(serializer)
        ErrorBody {
            code: 0,
            message: self.to_string(),
        }
        .serialize(serializer)
    }
}

/// 按 kind 返回不同错误，演示错误码传播链路。
// === 步骤 4: 补全 risky_operation 的匹配分支 —————————————————
// TODO: 补上 'missing' 与 'boom' 两个分支
// 提示: "missing" => Err(AppError::NotFound("demo 文件".into())),
//       "boom" => Err(AppError::Internal("数据库连接失败".into())),
#[tauri::command]
fn risky_operation(kind: String) -> Result<String, AppError> {
    match kind.as_str() {
        "bad" => Err(AppError::InvalidInput("内容不合法".into())),
        _ => Ok("操作成功".into()),
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![risky_operation])
        .run(tauri::generate_context!())
        .expect("启动 Tauri 应用失败");
}