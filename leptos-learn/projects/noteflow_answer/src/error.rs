use thiserror::Error;

#[derive(Error, Clone, Debug)]
pub enum AppError {
    #[error("文档未找到: {0}")]
    DocNotFound(String),
    #[error("用户未登录")]
    NotAuthenticated,
    #[error("无权限执行此操作")]
    PermissionDenied,
    #[error("工作区未找到: {0}")]
    WorkspaceNotFound(String),
    #[error("同步冲突")]
    SyncConflict,
    #[error("存储错误: {0}")]
    StorageError(String),
    #[error("{0}")]
    Generic(String),
}
