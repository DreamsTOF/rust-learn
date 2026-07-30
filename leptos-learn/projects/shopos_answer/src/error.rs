use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, Serialize, Deserialize, Error)]
pub enum AppError {
    #[error("未授权访问")]
    Unauthorized,
    #[error("权限不足")]
    Forbidden,
    #[error("资源不存在: {0}")]
    NotFound(String),
    #[error("验证失败: {0}")]
    ValidationError(String),
    #[error("库存不足: {0}")]
    InsufficientStock(String),
    #[error("优惠券无效: {0}")]
    InvalidCoupon(String),
    #[error("订单状态非法转换: {0}")]
    InvalidStateTransition(String),
    #[error("内部错误")]
    InternalError,
}

pub type AppResult<T> = Result<T, AppError>;
